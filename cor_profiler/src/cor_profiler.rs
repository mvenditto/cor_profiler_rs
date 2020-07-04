use crate::types::*;
use crate::interfaces::*;
use crate::il_rewriter::*;

use crate::opcodes::{
    CEE_LDSTR,
    CEE_LDARG_1,
    CEE_CALL
};
use crate::metadata_helpers::{
    get_meta_data_interface,
    get_module_name,
    get_function_info,
    define_assembly_reference,
    define_member_ref,
    define_type_ref,
    il_test,
    new_user_string
};

use std::{
    ptr,
    ffi::c_void,
    cell::RefCell,
    ffi::OsStr,
    os::windows::ffi::OsStrExt
};

extern crate env_logger;

extern crate com;
use com::{
    co_class,
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

fn function_seen(info: & ComPtr<dyn ICorProfilerInfo10>, function_id: FunctionID) -> HRESULT {
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
pub(crate) struct CorProfiler<'a> {
    prof_info: RefCell<Option<ComPtr<dyn ICorProfilerInfo10>>>,
    hook_ref: RefCell<mdMemberRef>
}

impl CorProfiler {
    pub(crate) fn new() -> Box<CorProfiler> {
        CorProfiler::allocate(
            RefCell::new(None),
            RefCell::new(0),
        )
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
                self.prof_info.replace(Some(info));
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
        
        if hr_status <= 0 {
            return hr_status;
        }
        
        let info_borrow = self.prof_info.borrow();
        let info = info_borrow.as_ref().unwrap();
        let info2 = info.get_interface::<dyn ICorProfilerInfo2>().unwrap();
        
        let assembly_emit = 
            get_meta_data_interface::<dyn IMetaDataAssemblyEmit>(info, module_id).unwrap();
        
        let metadata_emit = 
            get_meta_data_interface::<dyn IMetaDataEmit>(info, module_id).unwrap();
        

        let module_name = get_module_name(&info2, module_id).unwrap();

        if !module_name.ends_with("test.dll") {
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

        let maybe_method_ref = define_member_ref(
            &metadata_emit,
            type_ref,
            "Test",
            &[0x0, 0x1, 0x01, 0x0e]
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

        let info_borrow = self.prof_info.borrow();
        let info = info_borrow.as_ref().unwrap();

        // let info2 = info.get_interface::<dyn ICorProfilerInfo2>().unwrap();
        // let function_name = get_function_fully_qualified_name(&info2, function_id);
        
        /*
        match function_name {
            Ok(name) => 
                info!("func_name: {}", name),
            Err(hr) => 
                warn!("cannot get name for function 0x{:x} reason: hr=0x{:x}", function_id, hr)
        }*/

        function_seen(info, function_id); 

        S_OK 
    }
    
    unsafe fn jit_cached_function_search_finished(&self, function_id: FunctionID, result: COR_PRF_JIT_CACHE) -> HRESULT { 
        trace!("ICorProfilerCallback::jitcached_function_search_finished");
        
        let maybe_info = self.prof_info.borrow();
        let info = maybe_info.as_ref().unwrap();
        
        function_seen(info, function_id); 
        
        S_OK 
    }
}

impl ICorProfilerCallback2 for CorProfiler { }

impl ICorProfilerCallback3 for CorProfiler { }

#[allow(unused_variables)]
impl ICorProfilerCallback4 for CorProfiler {
   
    unsafe fn get_re_jit_parameters(&self, module_id: ModuleID, method_id: mdMethodDef, function_control: *mut *mut dyn ICorProfilerFunctionControl) -> HRESULT { 
        info!("ICorProfilerCallback4::get_re_jit_parameters"); 
        
        let info_borrow = self.prof_info.borrow();
        let info = info_borrow.as_ref().unwrap();
        let info1 = info.get_interface::<dyn ICorProfilerInfo>().unwrap();
        let info2 = info.get_interface::<dyn ICorProfilerInfo2>().unwrap();
        
        let rewriter = ILRewriter::new(
            (info1.as_raw()) as *mut *mut dyn ICorProfilerInfo,
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
            &info2, 
            module_id, 
            String::from("Test!")
        );

        let method_ref = *self.hook_ref.borrow();

        if method_ref <= 0 {
            error!("method_ref invalid 0x{:x}", method_ref);
        }

        let instr = rewriter.get_il_list().get_next().unwrap();
        let mut instr2 = instr;

        loop {
            info!("pre_opcode: 0x{:x}", instr2.opcode());
            
            match instr2.get_next() {
                Some(next) => instr2 = next,
                _ => {}
            }
            
            if instr2 == instr { break }
        }
        
        let mut instr_0 = ILInstr::new();
        instr_0.set_opcode(CEE_LDARG_1);
        rewriter.insert_before(instr, instr_0);

        
        let mut instr_1 = ILInstr::new();
        instr_1.set_opcode(CEE_CALL);
        instr_1.set_arg(ILInstrArgValue::Int32(method_ref));
        rewriter.insert_before(instr, instr_1);

        hr = rewriter.export();

        if hr < 0 {
            error!("export failed with hr=0x{:x}", hr);
        }

        loop {
            info!("after_opcode: 0x{:x}", instr2.opcode());
            
            match instr2.get_next() {
                Some(next) => instr2 = next,
                _ => {}
            }
            
            if instr2 == instr { break }
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
