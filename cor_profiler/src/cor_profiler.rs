use crate::types::*;
use crate::interfaces::*;
use crate::il_rewriter::*;

use crate::opcodes::OpCodes;
use crate::cor_helpers::{
    CorSignature,
    CorCallingConvention,
    CorElementType
};

use crate::metadata_helpers::{
    get_meta_data_interface,
    get_module_name,
    get_function_info,
    define_assembly_reference,
    define_member_ref,
    define_type_ref,
    get_module_info,
    il_test,
    new_user_string
};

use std::{
    ptr,
    ffi::c_void,
    cell::RefCell,
    ffi::OsStr,
    os::windows::ffi::OsStrExt,
    rc::Rc,
    collections::HashMap
};

extern crate env_logger;

extern crate com;
use com::{
    co_class,
    ComRc,
    ComPtr,
    interfaces::iunknown::IUnknown,
    sys::{HRESULT, S_OK},
    sys
};

use widestring::{
    U16String,
    U16CString
};

pub const CLSID_COR_PROFILER: sys::GUID = sys::GUID { 
    data1: 0xcf0d821e, 
    data2: 0x299b, 
    data3: 0x5307, 
    data4: [0xa3, 0xd8, 0xb2, 0x83, 0xc0, 0x39, 0x16, 0xdd]
};

macro_rules! check_failure {
    ($hr:expr, $msg: expr) => {
        if $hr < 0 {
            error!("{} failed with hr=0x{:x}", $msg, $hr);
            return E_FAIL;
        }
    }
}

fn function_seen(info: & ComRc<dyn ICorProfilerInfo10>, function_id: FunctionID) -> HRESULT {
    // let info_borrow = info.borrow();
    // let info = info_borrow.as_ref().unwrap();

    // info!("function seen 0x{:x}", function_id);

    let info2 = info.get_interface::<dyn ICorProfilerInfo2>().unwrap();
    
    let function_info = get_function_info(&info2, function_id);
    
    match function_info {
        Err(hr) => 
            hr,
        Ok(mut i) => {
            match get_module_name(&info2, i.module_id) {
                Err(hr) => return hr,
                Ok(module_name) => {
                    if module_name.ends_with("test.dll") && i.function_name == "TestMethod" {
                        
                        info!("request re_jit_with_inliners for {}", i.function_name);
                        
                        unsafe {
                            info.request_re_jit_with_inliners(
                                COR_PRF_REJIT_BLOCK_INLINING | COR_PRF_REJIT_INLINING_CALLBACKS,
                                1,
                                &mut i.module_id,
                                &mut i.metadata_token
                            );
                        }
                    }
                }
                _ => ()
            }
            S_OK
        }
    }
}

#[co_class(implements(ICorProfilerCallback8))]
pub(crate) struct CorProfiler<'a> { // TODO: Bug, lifetime erased
    prof_info: RefCell<Option<ComRc<dyn ICorProfilerInfo10>>>,
    hook_ref: RefCell<mdMemberRef>,
    data_container: RefCell<HashMap<String, Rc<dyn DataItem>>> // see Bug, should use &str
}

impl CorProfiler {
    pub(crate) fn new() -> Box<CorProfiler> {
        CorProfiler::allocate(
            RefCell::new(None),
            RefCell::new(0),
            RefCell::new(HashMap::new())
        )
    }

    pub(crate) fn get_profiler_info(&self) -> ComRc<dyn ICorProfilerInfo10> {
        ComRc::clone(self.prof_info.borrow().as_ref().unwrap())
    }
}

use crate::data_container::*;
impl DataContainer for CorProfiler {

    fn set_item(&self, key: String, item: Rc<dyn DataItem>) {
        self.data_container.borrow_mut().insert(key, item);
    }

    fn get_item(&self, key: String) -> Option<Rc<dyn DataItem>> {
        self.data_container.borrow().get(&key).map(Rc::clone)
    }
}

impl ICorProfilerCallback for CorProfiler {
    unsafe fn initialize(&self, i_cor_profiler_info_unk: ComPtr<dyn IUnknown>) -> HRESULT { 
        trace!("ICorProfilerCallback::initialize"); 

        // try getting the ICorProfilerInfo10 interface from the IUnknown ptr we received
        let maybe_prof_info = 
            i_cor_profiler_info_unk.get_interface::<dyn ICorProfilerInfo10>();

        match maybe_prof_info {
            Some(info) => {
                self.prof_info.replace(Some(info.upgrade()));
            }
            None => {
                error!("Cannot get ICorProfilerInfo. Initialization Failed.");
                return E_FAIL
            }
        }

        let maybe_info = self.prof_info.borrow();
        let info = maybe_info.as_ref().unwrap();

        let event_mask_low = COR_PRF_ENABLE_REJIT |
            COR_PRF_MONITOR_JIT_COMPILATION |
            COR_PRF_MONITOR_CACHE_SEARCHES | 
            COR_PRF_MONITOR_MODULE_LOADS |
            COR_PRF_MONITOR_ASSEMBLY_LOADS |
            COR_PRF_DISABLE_ALL_NGEN_IMAGES;
        
        // set the profiler features we're interested in
        let hr = info.set_event_mask2(event_mask_low, 0x0);
        check_failure!(hr, "set_event_mask2");
        
        hr 
    }

    unsafe fn shutdown(&self) -> HRESULT { 
        trace!("ICorProfilerCallback::shutdown"); 

        (&*self.prof_info.borrow()).as_ref().unwrap().release();

        S_OK 
    }

    unsafe fn module_load_finished(&self, module_id: ModuleID, hr_status: HRESULT) -> HRESULT {
        
        /*if hr_status < 0 {
            return hr_status;
        }*/

        let info = &self.get_profiler_info();

        let assembly_emit = 
            get_meta_data_interface::<dyn IMetaDataAssemblyEmit>(info, module_id).unwrap();
        
        let metadata_emit = 
            get_meta_data_interface::<dyn IMetaDataEmit>(info, module_id).unwrap();
        

        let module_name = get_module_name(info, module_id).unwrap();

        if !module_name.ends_with("test.dll") {

            if module_name.contains("System.Private.CoreLib.dll") {
                // test: extract DateTime type metadata token
                info!("System.Private.CoreLib.dll");

                let metadata_import = 
                    get_meta_data_interface::<dyn IMetaDataImport>(info, module_id).unwrap();
                
                let mut date_time_tk: mdToken = 0;
                
                let type_name_native = OsStr::new("System.DateTime")
                    .encode_wide()
                    .chain(Some(0).into_iter())
                    .collect::<Vec<_>>();

                let hr = metadata_import.find_type_def_by_name(
                    type_name_native.as_ptr(),
                    0,
                    &mut date_time_tk
                );
                  
                if hr < 0 {
                    error!("find_type_def_by_name hr=0x{:x} ", hr);
                    return hr;
                }

                info!("DateTime token=0x{:x}", date_time_tk)
            }

            return S_OK;
        }

        info!("module_load_finished: {}", module_name);

        let maybe_assembly_ref = define_assembly_reference(
            &assembly_emit,
            &[0xf3, 0x3c, 0xbf, 0xca, 0x3a, 0x74, 0xa3, 0xba],
            "helpers",
            "neutral",
            "1.0.0.0"
        );

        let assembly_ref = match maybe_assembly_ref {
            Ok(assembly_ref) => assembly_ref,
            Err(hresult) => {
                error!("define_assembly_ref failed hr=0x{:x}", hresult);
                return hresult;
            }
        };

        info!("pushed helpers.dll ref to test.dll");

        let maybe_type_ref = define_type_ref(
            &metadata_emit,
            assembly_ref,
            "helpers.Class1"
        );

        let type_ref = match maybe_type_ref {
            Ok(type_ref) => type_ref,
            Err(hresult) => {
                error!("define_type_ref failed hr=0x{:x}", hresult);
                return hresult;
            }
        };

        info!("pushed helpers.Class1 (0x{:x}) ref to test.dll", type_ref);

        let signature = CorSignature::new()
            .call_conv(CorCallingConvention::IMAGE_CEE_CS_CALLCONV_DEFAULT)
            .ret(CorElementType::ELEMENT_TYPE_VOID)
            .arg(CorElementType::ELEMENT_TYPE_STRING);

        let maybe_method_ref = define_member_ref(
            &metadata_emit,
            type_ref,
            "Test",
            &signature.pack()
        );

        let method_ref = match maybe_method_ref {
            Ok(method_ref) => method_ref,
            Err(hresult) => {
                error!("define_member_ref failed hr=0x{:x}", hresult);
                return hresult;
            }
        };

        info!("pushed helpers.Class1.Test (0x{:x}) ref to test.dll", method_ref);

        self.hook_ref.replace(method_ref);

        S_OK
    }

    unsafe fn jit_compilation_finished(&self, function_id: FunctionID, hr_status: HRESULT, f_is_safe_to_block: BOOL) -> HRESULT { 
        /*trace!("ICorProfilerCallback::jitcompilation_finished");*/ 
        trace!("ICorProfilerCallback::jitcached_function_search_finished"); 

        function_seen(&self.get_profiler_info(), function_id); 

        S_OK 
    }
    
    unsafe fn jit_cached_function_search_finished(&self, function_id: FunctionID, result: COR_PRF_JIT_CACHE) -> HRESULT { 
        trace!("ICorProfilerCallback::jitcached_function_search_finished");
        
        function_seen(&self.get_profiler_info(), function_id); 
        
        S_OK 
    }
}

impl ICorProfilerCallback2 for CorProfiler { }

impl ICorProfilerCallback3 for CorProfiler { }

#[allow(unused_variables)]
impl ICorProfilerCallback4 for CorProfiler {
   
    unsafe fn get_re_jit_parameters(&self, module_id: ModuleID, method_id: mdMethodDef, function_control: *mut *mut dyn ICorProfilerFunctionControl) -> HRESULT { 
        info!("ICorProfilerCallback4::get_re_jit_parameters"); 
        
        let info = &self.get_profiler_info();
        
        let mut rewriter = ILRewriter::new(
            (info.as_raw()) as *mut *mut dyn ICorProfilerInfo,
            function_control,
            module_id,
            method_id
        );

        info!("Created IL Rewriter!");

        let mut hr = rewriter.import();

        if hr < 0 {
            error!("import failed with hr=0x{:x}", hr);
        }

        let new_string = new_user_string(
            info, 
            module_id, 
            String::from("Test!")
        );

        let method_ref = *self.hook_ref.borrow();

        if method_ref <= 0 {
            error!("method_ref invalid 0x{:x}", method_ref);
        }

        let head = rewriter.get_il_list();
        let instr = head.get_next().unwrap();

        for i in &mut rewriter {
            info!("pre_opcode: 0x{:x}", i.opcode());
        }
        
        let mut instr_0 = ILInstr::new();
        instr_0.set_opcode(OpCodes::CEE_LDARG_1);
        rewriter.insert_before(instr, instr_0);

        
        let mut instr_1 = ILInstr::new();
        instr_1.set_opcode(OpCodes::CEE_CALL);
        instr_1.set_arg(ILInstrArgValue::Int32(method_ref));
        rewriter.insert_before(instr, instr_1);

        hr = rewriter.export();

        if hr < 0 {
            error!("export failed with hr=0x{:x}", hr);
        }

        for i in &mut rewriter {
            info!("after_opcode: 0x{:x}", i.opcode());
        }

        
        /*
        match new_string {
            Err(hr) => {
                error!("cannot define user string hr=0x{:x}", hr)
            },
            Ok(token) => {
                let head_instr = rewriter.get_il_list();
                let mut instr = head_instr;
                loop {
                    if instr.opcode() == CEE_LDSTR {
                        info!("found ldstr");
                        instr.set_arg_32(token);
                    }
                    info!("0x{:x}", instr.opcode());
                    
                    match instr.get_next() {
                        Some(next) => instr = next,
                        _ => {}
                    }
                    
                    if instr == head_instr { break }
                }

                hr = rewriter.export();

                if hr < 0 {
                    error!("export failed with hr=0x{:x}", hr);
                }
            }
        }*/

        S_OK 
    }
}

impl ICorProfilerCallback5 for CorProfiler { }

impl ICorProfilerCallback6 for CorProfiler { }

impl ICorProfilerCallback7 for CorProfiler { }

impl ICorProfilerCallback8 for CorProfiler { }
