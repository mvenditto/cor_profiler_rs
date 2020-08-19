use crate::cor_helpers::{
    ICLRMetaHost
};

use crate::types::{
    ofReadWriteMask, REFIID
};

use crate::interfaces::{
    IMetaDataDispenser,
    IMetaDataImport,
    IMetaDataImport2,
    IMetaDataAssemblyImport
};

use crate::metadata_helpers::{
    enum_assembly_refs,
    enum_type_refs
};

use std::{
    ptr,
    ffi::c_void
};

use crate::guids::IID_IMetaDataImport2;
use crate::utils::to_widestring;
use crate::tests::common::unwrap_or_fail;

use com::{
    ComPtr,
    ComInterface,
    ComRc,
    interfaces::IUnknown,
    sys::{HRESULT, S_OK}
};

use std::sync::Once;
use std::cell::RefCell;


static INIT: Once = Once::new();

struct MetaData {
    metadata_import: RefCell<Option<ComRc<dyn IMetaDataImport2>>>,
    metadata_assembly_import: RefCell<Option<ComRc<dyn IMetaDataAssemblyImport>>>,
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
}

thread_local! {
    static METADATA: MetaData = MetaData {
        metadata_import: RefCell::new(None),
        metadata_assembly_import: RefCell::new(None)
    };
}

fn setup() {
    info!("do setup()");
    let metahost = ICLRMetaHost::create().unwrap();
    let runtime = metahost.get_latest_installed_runtime().unwrap();
    let runtime_version = runtime.get_version_string().unwrap();
    info!("latest runtime version: {}", runtime_version);
    let mut scope = std::env::current_dir().unwrap();
    scope.pop();
    scope.push(r#"examples\SampleLibrary\SampleLibrary\bin\Debug\netstandard2.0\SampleLibrary.dll"#);
    let scope_path = scope.to_string_lossy();
    info!("scope: {}", scope_path);
    let metadata_dispenser = runtime.get_metadata_dispenser().unwrap();
    let scope_wstr = to_widestring(&scope_path);
    unsafe {
        let mut unkn: *mut c_void = ptr::null_mut();

        let hr = metadata_dispenser.open_scope(
            scope_wstr.as_ptr(), 
            ofReadWriteMask, 
            &IID_IMetaDataImport2, 
            &mut unkn);

        if hr < 0 {
            panic!("failed to open {} metadata hr=0x{:x}",scope_path, hr);
        }
        
        let iunk = 
            ComPtr::<dyn IUnknown>::new(unkn as *mut _).upgrade();

        let metadata_import = 
            iunk.get_interface::<dyn IMetaDataImport2>().unwrap();

        let metadata_assembly_import = 
            iunk.get_interface::<dyn IMetaDataAssemblyImport>().unwrap();

        METADATA.with(|md|{
            md.metadata_import.replace(Some(metadata_import));
        });

        METADATA.with(|md|{
            md.metadata_assembly_import.replace(Some(metadata_assembly_import));
        });

        metadata_dispenser.release();
    }
}

fn init() {
    INIT.call_once(setup);
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

        unsafe { metadata_assembly_import.release(); }
    });
}

#[test]
pub fn should_not_find_assembly_ref() {
    init();
    METADATA.with(|md|{

        let metadata_assembly_import = md.metadata_assembly_import();

        let result = unwrap_or_fail(
            enum_assembly_refs(&metadata_assembly_import,"System.Net.Http"),
            "should not return Error");

        assert!(result.is_none(), "System.Net.Http.dll should NOT be referenced");

        unsafe { metadata_assembly_import.release(); }
    });
}

#[test]
pub fn should_find_type_ref() {
    init();
    METADATA.with(|md|{

        let metadata_import = md.metadata_import()
            .get_interface::<dyn IMetaDataImport>().unwrap();

        let result = unwrap_or_fail(
            enum_type_refs(&metadata_import, "System.Object"),
            "should not return Error");

        assert!(result.is_some(), "System.Object should be referenced");

        unsafe { metadata_import.release(); }
    });
}

#[test]
pub fn should_not_find_type_ref() {
    init();
    METADATA.with(|md|{

        let metadata_import = md.metadata_import()
            .get_interface::<dyn IMetaDataImport>().unwrap();

        let result = unwrap_or_fail(
            enum_type_refs(&metadata_import, "System.Net.Http.HttpClient"),
            "should not return Error");

        assert!(result.is_none(), "System.Net.Http.dll should NOT be referenced");

        unsafe { metadata_import.release(); }
    });
}