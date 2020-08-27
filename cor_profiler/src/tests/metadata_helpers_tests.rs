use crate::cor_helpers::{
    create_clr_metahost,
    get_latest_installed_runtime,
    get_clr_runtime_version_string,
    get_metadata_dispenser,
    load_library,
    get_clr_runtime_host
};

use crate::types::{
    ofReadWriteMask,
    mdTokenNil,
    mdAssemblyRef,
    LPWSTR,
    WCHAR,
    DWORD
};

use crate::interfaces::{
    IMetaDataDispenser,
    IMetaDataImport,
    IMetaDataImport2,
    ICLRRuntimeHost,
    ICLRMetaHost,
    ICLRRuntimeInfo,
    IMetaDataEmit2,
    IMetaDataAssemblyEmit,
    IMetaDataAssemblyImport,
    IEnumUnknown
};

use crate::metadata_helpers::{
    enum_assembly_refs,
    enum_type_refs,
    find_type_def_info,
    find_type_ref_info,
    define_assembly_reference,
    get_assembly_refs_iter,
    get_assembly_info
};

use std::{
    ptr,
    ffi::c_void
};

use crate::guids::IID_IMetaDataImport2;
use crate::utils::{
    to_widestring,
    relative_to_cwd
};

use crate::tests::common::{
    unwrap_or_fail,
    unwrap_or_fail_opt
};

use com::{
    ComPtr,
    ComRc,
    interfaces::IUnknown
};

use std::sync::Once;
use std::cell::RefCell;
use widestring::U16String;


static INIT: Once = Once::new();

struct MetaData {
    metadata_import: RefCell<Option<ComRc<dyn IMetaDataImport2>>>,
    metadata_assembly_import: RefCell<Option<ComRc<dyn IMetaDataAssemblyImport>>>,
    metadata_emit: RefCell<Option<ComRc<dyn IMetaDataEmit2>>>,
    metadata_assembly_emit: RefCell<Option<ComRc<dyn IMetaDataAssemblyEmit>>>
}

unsafe impl Sync for MetaData { }

impl MetaData {
    fn metadata_assembly_import(&self) -> ComRc<dyn IMetaDataAssemblyImport> {
        match self.metadata_assembly_import.borrow().as_ref() {
            Some(i) => ComRc::clone(&i),
            _ => panic!()
        }
    }

    fn metadata_import(&self) -> ComRc<dyn IMetaDataImport2> {
        match self.metadata_import.borrow().as_ref() {
            Some(i) => ComRc::clone(&i),
            _ => panic!()
        }
    }

    fn metadata_emit(&self) -> ComRc<dyn IMetaDataEmit2> {
        match self.metadata_emit.borrow().as_ref() {
            Some(i) => ComRc::clone(&i),
            _ => panic!()
        }
    }

    fn metadata_assembly_emit(&self) -> ComRc<dyn IMetaDataAssemblyEmit> {
        match self.metadata_assembly_emit.borrow().as_ref() {
            Some(i) => ComRc::clone(&i),
            _ => panic!()
        }
    }
}

struct ClrInstance {
    metahost: RefCell<Option<ComRc<dyn ICLRMetaHost>>>,
    runtime_info: RefCell<Option<ComRc<dyn ICLRRuntimeInfo>>>,
    runtime_host: RefCell<Option<ComRc<dyn ICLRRuntimeHost>>>
}

unsafe impl Sync for ClrInstance { }

impl ClrInstance {
    fn runtime_host(&self) -> ComRc<dyn ICLRRuntimeHost> {
        match self.runtime_host.borrow().as_ref() {
            Some(i) => ComRc::clone(&i),
            _ => panic!()
        }
    }
}

thread_local! {
    static METADATA: MetaData = MetaData {
        metadata_import: RefCell::new(None),
        metadata_assembly_import: RefCell::new(None),
        metadata_emit: RefCell::new(None),
        metadata_assembly_emit: RefCell::new(None),
    };

    static CLR: ClrInstance = ClrInstance {
        metahost: RefCell::new(None),
        runtime_info: RefCell::new(None),
        runtime_host: RefCell::new(None)
    };
}

fn setup() {
    info!("do setup()");
    let metahost = create_clr_metahost().unwrap();
    let runtime = get_latest_installed_runtime(&metahost).unwrap();
    let runtime_version = get_clr_runtime_version_string(&runtime).unwrap();
    info!("latest runtime version: {}", runtime_version);
    let mut scope = std::env::current_dir().unwrap();
    scope.pop();
    scope.push(r#"examples\SampleLibrary\SampleLibrary\bin\Debug\netstandard2.0\SampleLibrary.dll"#);
    let scope_path = scope.to_string_lossy();
    info!("scope: {}", scope_path);
    let metadata_dispenser = get_metadata_dispenser(&runtime).unwrap();
    let scope_wstr = to_widestring(&scope_path);
    unsafe {
        let mut unkn: *mut c_void = ptr::null_mut();

        let hr = metadata_dispenser.open_scope(
            scope_wstr.as_ptr(), 
            ofReadWriteMask, 
            &IID_IMetaDataImport2, 
            &mut unkn);

        if hr < 0 {
            panic!("failed to open {} metadata hr=0x{:x}", scope_path, hr);
        }

        let runtime_host = get_clr_runtime_host(&runtime).unwrap();

        CLR.with(|clr|{
            clr.metahost.replace(Some(metahost));
            clr.runtime_info.replace(Some(runtime));
            clr.runtime_host.replace(Some(runtime_host))
        });
        
        let iunk = 
            ComPtr::<dyn IUnknown>::new(unkn as *mut _).upgrade();

        let metadata_import = 
            iunk.get_interface::<dyn IMetaDataImport2>().unwrap();

        let metadata_assembly_import = 
            iunk.get_interface::<dyn IMetaDataAssemblyImport>().unwrap();
            
        let metadata_emit = 
            iunk.get_interface::<dyn IMetaDataEmit2>().unwrap();

        let metadata_assembly_emit = 
            iunk.get_interface::<dyn IMetaDataAssemblyEmit>().unwrap();

        METADATA.with(|md|{
            md.metadata_import.replace(Some(metadata_import));
        });

        METADATA.with(|md|{
            md.metadata_assembly_import.replace(Some(metadata_assembly_import));
        });

        METADATA.with(|md|{
            md.metadata_emit.replace(Some(metadata_emit));
        });

        METADATA.with(|md|{
            md.metadata_assembly_emit.replace(Some(metadata_assembly_emit));
        });

        metadata_dispenser.release();
    }
}

fn init() {
    INIT.call_once(setup);
}

#[test]
pub fn clr_runtime_host_test() {
    init();
    CLR.with(|clr| {
        let runtime_host = clr.runtime_host();

        unsafe {
            let assembly = to_widestring(
                &relative_to_cwd(r#"examples\SampleLibrary\SampleProgram\bin\Debug\net48\SampleProgram.dll"#));
           
            let type_name = to_widestring("SampleProgram.Program");
            let method = to_widestring("TestMethod");
            let arg = to_widestring("42+42");
            let mut hr = runtime_host.start();
            assert_ok!(hr, "should start clr");
            let mut retval: DWORD = 0;
            hr = runtime_host.execute_in_default_app_domain(
                assembly.as_ptr(),
                type_name.as_ptr(),
                method.as_ptr(),
                arg.as_ptr(),
                &mut retval
            );
            assert_ok!(hr, "should execute_in_default_app_domain");
            assert_eq!(retval, 84 as DWORD);
            hr = runtime_host.stop();
            assert_ok!(hr, "should stop clr");
        }
    });
}

#[test]
pub fn clr_runtime_load_assembly() {
    init();
    CLR.with(|clr| {
        let _runtime_info = clr.runtime_info.borrow();
        let runtime_info = _runtime_info.as_ref().unwrap();
        let lib = r#"System.Net.Http.dll"#;
        let result = load_library(&runtime_info, lib);
        /*
        match result {
            Err(hr) => println!("hr=0x{:x}", hr),
            Ok(hmodule) => println!("hr=0x{:p}", hmodule as PVOID)
        }
        */
        assert!(result.is_ok(), "should return a module handle");
    });
}

#[test]
pub fn should_find_assembly_ref() {
    init();
    METADATA.with(|md|{

        let metadata_assembly_import = md.metadata_assembly_import();

        let result = unwrap_or_fail(
            enum_assembly_refs(&metadata_assembly_import,"netstandard"),
            "should not return Error");

        assert!(result.is_some(), "netstandard.dll should be referenced");
    });
}

#[test]
pub fn test_get_assembly_refs_iter() {
    init();
    METADATA.with(|md|{

        let metadata_assembly_import = md.metadata_assembly_import();
        for assembly_ref in get_assembly_refs_iter(&metadata_assembly_import) {
            let name = get_assembly_info(&metadata_assembly_import, assembly_ref).unwrap().assembly_name;
            info!("name: {}", name);
        }
    });
}

#[test]
pub fn should_not_find_assembly_ref() {
    init();
    METADATA.with(|md|{

        let metadata_assembly_import = md.metadata_assembly_import();

        let result = unwrap_or_fail(
            enum_assembly_refs(&metadata_assembly_import,"System.IO.Compression"),
            "should not return Error");

        assert!(result.is_none(), "System.IO.Compression.dll should NOT be referenced");
    });
}

#[test]
pub fn should_find_type_ref_enumerating() {
    init();
    METADATA.with(|md|{

        let metadata_import = md.metadata_import()
            .get_interface::<dyn IMetaDataImport>().unwrap();

        let result = unwrap_or_fail(
            enum_type_refs(&metadata_import, "System.Object"),
            "should not return Error");

        assert!(result.is_some(), "System.Object should be referenced");
    });
}

#[test]
pub fn should_not_find_type_ref_enumerating() {
    init();
    METADATA.with(|md|{

        let metadata_import = md.metadata_import()
            .get_interface::<dyn IMetaDataImport>().unwrap();

        let result = unwrap_or_fail(
            enum_type_refs(&metadata_import, "System.Net.Http.HttpClient"),
            "should not return Error");

        assert!(result.is_none(), "System.Net.Http.dll should NOT be referenced");
    });
}

#[test]
pub fn should_retrieve_existing_type_info() {
    init();
    METADATA.with(|md|{

        let metadata_import = md.metadata_import();

        let result = unwrap_or_fail(
            find_type_def_info(
                &metadata_import, 
                "SampleLibrary.Class1", 
            mdTokenNil),
            "should not return Error");

        let type_info = unwrap_or_fail_opt(
            result, 
            "should retrive info about SampleLibrary.Class1");

        assert_eq!(type_info.type_name, "SampleLibrary.Class1");
    });
}

#[test]
pub fn should_not_error_retrieving_not_existing_type_info() {
    init();
    METADATA.with(|md|{

        let metadata_import = md.metadata_import();

        let result = unwrap_or_fail(
            find_type_def_info(
                &metadata_import, 
                "System.Net.Http.HttpClient",
                mdTokenNil),
        "should not return Error");

       assert!(
           result.is_none(), 
          "should NOT retrive info about System.Net.Http.HttpClient"
        );
    });
}

#[test]
pub fn should_retrieve_existing_nested_class_info() {
    init();
    METADATA.with(|md|{

        let metadata_import = md.metadata_import();

        let class_1 = unwrap_or_fail(
            find_type_def_info(
                &metadata_import, 
                "SampleLibrary.Class1",  
            mdTokenNil),
            "should not return Error");
        
        let class_1_type_info = unwrap_or_fail_opt(
            class_1, 
            "SampleLibrary.Class1 type info");

        assert_eq!(class_1_type_info.type_name, "SampleLibrary.Class1");

        let class_1_token = class_1_type_info.metadata_token;

        let class_1_child = unwrap_or_fail(
            find_type_def_info(
                &metadata_import, 
                "Class1Child", 
                class_1_token),
            "should not return Error");

        let class_1_child_type_info = unwrap_or_fail_opt(
            class_1_child, 
            "SampleLibrary.Class1Child type info");   
        
        assert!(is_td_nested!(class_1_child_type_info.metadata_token), "");
        assert_eq!(class_1_child_type_info.type_name, "Class1Child");
        assert_eq!(class_1_child_type_info.parent_token, class_1_token);

    });
}

#[test]
pub fn should_find_type_ref() {
    init();
    METADATA.with(|md|{
        let metadata_import = md.metadata_import();
        let metadata_assembly_import = md.metadata_assembly_import();
        
        let maybe_assembly_ref = unwrap_or_fail(enum_assembly_refs(
            &metadata_assembly_import, 
            "netstandard"),
            "should not return Error");
        
        let assembly_ref: mdAssemblyRef = unwrap_or_fail_opt(
            maybe_assembly_ref, 
            "should find netstandard ref");
        
        let maybe_type_info = unwrap_or_fail(
            find_type_ref_info(
                &metadata_import, 
                "System.Object",  
                assembly_ref),
            "should not return Error");
        
        let type_info = unwrap_or_fail_opt(
            maybe_type_info, 
            "should find System.Object type info");
        
        assert_eq!(type_info.type_name, "System.Object");
        
    });
}

#[test]
pub fn should_inject_assembly_ref() {
    init();
    METADATA.with(|md|{
        let metadata_assembly_import = md.metadata_assembly_import();
        let metadata_assembly_emit = md.metadata_assembly_emit();

        let system_net_http_assembly_ref = 
            enum_assembly_refs(&metadata_assembly_import, "System.Net.Http");

        assert!(system_net_http_assembly_ref.unwrap().is_none(), "System.Net.Http should not be referenced");

        let new_assembly_ref = unwrap_or_fail(define_assembly_reference(
            &metadata_assembly_emit, 
            &[0xb0, 0x3f, 0x5f, 0x7f, 0x11, 0xd5, 0x0a, 0x3a], 
            "System.Net.Http", 
            "neutral", 
            "4.2.2.0"
        ), "should be able to create an assembly reference");

        assert_ne!(new_assembly_ref, mdTokenNil);

        let maybe_typeref  = 
            enum_assembly_refs(&metadata_assembly_import, "System.Net.Http");
        
        assert!(maybe_typeref.unwrap().is_some(), "System.Net.Http should now be referenced");

    });
}