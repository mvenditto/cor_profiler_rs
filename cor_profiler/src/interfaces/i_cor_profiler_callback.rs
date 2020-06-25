#![allow(unused_variables)]

use crate::types::*;

use std::ffi::c_void;

extern crate com;
use com::{
    com_interface, 
    ComPtr,
    interfaces::iunknown::IUnknown, 
    interfaces::iclass_factory::IClassFactory,
    sys::HRESULT,
    sys::S_OK
};


pub trait ICorProfilerAssemblyReferenceProvider { }

#[com_interface("F0963021-E1EA-4732-8581-E01B0BD3C0C6")]
pub trait ICorProfilerFunctionControl: IUnknown {
    unsafe fn set_codegen_flags(&self, flags: DWORD) -> HRESULT { S_OK }
    unsafe fn set_il_function_body(&self, cb_new_il_method_header: ULONG, pb_new_il_method_header: LPCBYTE) -> HRESULT { S_OK }
    unsafe fn set_il_instrumented_code_map(&self, c_il_map_entries: ULONG, rg_il_map_entries: *mut COR_IL_MAP) -> HRESULT { S_OK }
}

#[com_interface("176FBED1-A55C-4796-98CA-A9DA0EF883E7")]
pub trait ICorProfilerCallbackClass: IClassFactory {}

#[com_interface("176FBED1-A55C-4796-98CA-A9DA0EF883E7")]
pub trait ICorProfilerCallback: IUnknown {
    unsafe fn initialize(&self, p_icor_profiler_info_unk: ComPtr<dyn IUnknown>) -> HRESULT { S_OK }
    unsafe fn shutdown(&self) -> HRESULT { S_OK }
    unsafe fn app_domain_creation_started(&self, app_domain_id: AppDomainID) -> HRESULT { S_OK }
    unsafe fn app_domain_creation_finished(&self, app_domain_id: AppDomainID, hr_status: HRESULT) -> HRESULT { S_OK }
    unsafe fn app_domain_shutdown_started(&self, app_domain_id: AppDomainID) -> HRESULT { S_OK }
    unsafe fn app_domain_shutdown_finished(&self, app_domain_id: AppDomainID, hr_status: HRESULT) -> HRESULT { S_OK }
    unsafe fn assembly_load_started(&self, assembly_id: AssemblyID) -> HRESULT { S_OK }
    unsafe fn assembly_load_finished(&self, assembly_id: AssemblyID, hr_status: HRESULT) -> HRESULT { S_OK }
    unsafe fn assembly_unload_started(&self, assembly_id: AssemblyID) -> HRESULT { S_OK }
    unsafe fn assembly_unload_finished(&self, assembly_id: AssemblyID, hr_status: HRESULT) -> HRESULT { S_OK }
    unsafe fn module_load_started(&self, module_id: ModuleID) -> HRESULT { S_OK }
    unsafe fn module_load_finished(&self, module_id: ModuleID, hr_status: HRESULT) -> HRESULT { S_OK }
    unsafe fn module_unload_started(&self, module_id: ModuleID) -> HRESULT { S_OK }
    unsafe fn module_unload_finished(&self, module_id: ModuleID, hr_status: HRESULT) -> HRESULT { S_OK }
    unsafe fn module_attached_to_assembly(&self, module_id: ModuleID, assembly_id: AssemblyID) -> HRESULT { S_OK }
    unsafe fn class_load_started(&self, class_id: ClassID) -> HRESULT { S_OK }
    unsafe fn class_load_finished(&self, class_id: ClassID, hr_status: HRESULT) -> HRESULT { S_OK }
    unsafe fn class_unload_started(&self, class_id: ClassID) -> HRESULT { S_OK }
    unsafe fn class_unload_finished(&self, class_id: ClassID, hr_status: HRESULT) -> HRESULT { S_OK }
    unsafe fn function_unload_started(&self, function_id: FunctionID) -> HRESULT { S_OK }
    unsafe fn jit_compilation_started(&self, function_id: FunctionID, f_is_safe_to_block: BOOL) -> HRESULT { S_OK }
    unsafe fn jit_compilation_finished(&self, function_id: FunctionID, hr_status: HRESULT, f_is_safe_to_block: BOOL) -> HRESULT { S_OK }
    unsafe fn jit_cached_function_search_started(&self, function_id: FunctionID, pb_use_cached_function: *mut BOOL) -> HRESULT { S_OK }
    unsafe fn jit_cached_function_search_finished(&self, function_id: FunctionID, result: COR_PRF_JIT_CACHE) -> HRESULT { S_OK }
    unsafe fn jit_function_pitched(&self, function_id: FunctionID) -> HRESULT { S_OK }
    unsafe fn jit_inlining(&self, caller_id: FunctionID, callee_id: FunctionID, pf_should_inline: *mut BOOL) -> HRESULT { S_OK }
    unsafe fn thread_created(&self, thread_id: ThreadID) -> HRESULT { S_OK }
    unsafe fn thread_destroyed(&self, thread_id: ThreadID) -> HRESULT { S_OK }
    unsafe fn thread_assigned_to_osthread(&self, managed_thread_id: ThreadID, os_thread_id: DWORD) -> HRESULT { S_OK }
    unsafe fn remoting_client_invocation_started(&self) -> HRESULT { S_OK }
    unsafe fn remoting_client_sending_message(&self, p_cookie: *mut GUID, f_is_async: BOOL) -> HRESULT { S_OK }
    unsafe fn remoting_client_receiving_reply(&self, p_cookie: *mut GUID, f_is_async: BOOL) -> HRESULT { S_OK }
    unsafe fn remoting_client_invocation_finished(&self) -> HRESULT { S_OK }
    unsafe fn remoting_server_receiving_message(&self, p_cookie: *mut GUID, f_is_async: BOOL) -> HRESULT { S_OK }
    unsafe fn remoting_server_invocation_started(&self) -> HRESULT { S_OK }
    unsafe fn remoting_server_invocation_returned(&self) -> HRESULT { S_OK }
    unsafe fn remoting_server_sending_reply(&self, p_cookie: *mut GUID, f_is_async: BOOL) -> HRESULT { S_OK }
    unsafe fn unmanaged_to_managed_transition(&self, function_id: FunctionID, reason: COR_PRF_TRANSITION_REASON) -> HRESULT { S_OK }
    unsafe fn managed_to_unmanaged_transition(&self, function_id: FunctionID, reason: COR_PRF_TRANSITION_REASON) -> HRESULT { S_OK }
    unsafe fn runtime_suspend_started(&self, suspend_reason: COR_PRF_SUSPEND_REASON) -> HRESULT { S_OK }
    unsafe fn runtime_suspend_finished(&self) -> HRESULT { S_OK }
    unsafe fn runtime_suspend_aborted(&self) -> HRESULT { S_OK }
    unsafe fn runtime_resume_started(&self) -> HRESULT { S_OK }
    unsafe fn runtime_resume_finished(&self) -> HRESULT { S_OK }
    unsafe fn runtime_thread_suspended(&self, thread_id: ThreadID) -> HRESULT { S_OK }
    unsafe fn runtime_thread_resumed(&self, thread_id: ThreadID) -> HRESULT { S_OK }
    unsafe fn moved_references(&self, c_moved_object_idranges: ULONG, old_object_idrange_start: *mut ObjectID, new_object_idrange_start: *mut ObjectID, c_object_idrange_length: *mut ULONG) -> HRESULT { S_OK }
    unsafe fn object_allocated(&self, object_id: ObjectID, class_id: ClassID) -> HRESULT { S_OK }
    unsafe fn objects_allocated_by_class(&self, c_class_count: ULONG, class_ids: *mut ClassID, c_objects: *mut ULONG) -> HRESULT { S_OK }
    unsafe fn object_references(&self, object_id: ObjectID, class_id: ClassID, c_object_refs: ULONG, object_ref_ids: *mut ObjectID) -> HRESULT { S_OK }
    unsafe fn root_references(&self, c_root_refs: ULONG, root_ref_ids: *mut ObjectID) -> HRESULT { S_OK }
    unsafe fn exception_thrown(&self, thrown_object_id: ObjectID) -> HRESULT { S_OK }
    unsafe fn exception_search_function_enter(&self, function_id: FunctionID) -> HRESULT { S_OK }
    unsafe fn exception_search_function_leave(&self) -> HRESULT { S_OK }
    unsafe fn exception_search_filter_enter(&self, function_id: FunctionID) -> HRESULT { S_OK }
    unsafe fn exception_search_filter_leave(&self) -> HRESULT { S_OK }
    unsafe fn exception_search_catcher_found(&self, function_id: FunctionID) -> HRESULT { S_OK }
    unsafe fn exception_os_handler_enter(&self, __unused: UINT_PTR) -> HRESULT { S_OK }
    unsafe fn exception_os_handler_leave(&self, __unused: UINT_PTR) -> HRESULT { S_OK }
    unsafe fn exception_unwind_function_enter(&self, function_id: FunctionID) -> HRESULT { S_OK }
    unsafe fn exception_unwind_function_leave(&self) -> HRESULT { S_OK }
    unsafe fn exception_unwind_finally_enter(&self, function_id: FunctionID) -> HRESULT { S_OK }
    unsafe fn exception_unwind_finally_leave(&self) -> HRESULT { S_OK }
    unsafe fn exception_catcher_enter(&self, function_id: FunctionID, object_id: ObjectID) -> HRESULT { S_OK }
    unsafe fn exception_catcher_leave(&self) -> HRESULT { S_OK }
    unsafe fn com_classic_vtable_created(&self, wrapped_class_id: ClassID, implemented_iid: REFGUID, p_vtable: *mut c_void, c_slots: ULONG) -> HRESULT { S_OK }
    unsafe fn com_classic_vtable_destroyed(&self, wrapped_class_id: ClassID, implemented_iid: REFGUID, p_vtable: *mut c_void) -> HRESULT { S_OK }
    unsafe fn exception_clr_catcher_found(&self) -> HRESULT { S_OK }
    unsafe fn exception_clr_catcher_execute(&self) -> HRESULT { S_OK }
}

#[com_interface("8A8CC829-CCF2-49fe-BBAE-0F022228071A")]
pub trait ICorProfilerCallback2: ICorProfilerCallback {
    unsafe fn thread_name_changed(&self, thread_id: ThreadID, cch_name: ULONG, name: *mut WCHAR) -> HRESULT { S_OK }
    unsafe fn garbage_collection_started(&self, c_generations: i32, generation_collected: *mut BOOL, reason: COR_PRF_GC_REASON) -> HRESULT { S_OK }
    unsafe fn surviving_references(&self, c_surviving_object_idranges: ULONG, object_idrange_start: *mut ObjectID, c_object_idrange_length: *mut ULONG) -> HRESULT { S_OK }
    unsafe fn garbage_collection_finished(&self) -> HRESULT { S_OK }
    unsafe fn finalizeable_object_queued(&self, finalizer_flags: DWORD, object_id: ObjectID) -> HRESULT { S_OK }
    unsafe fn root_references2(&self, c_root_refs: ULONG, root_ref_ids: *mut ObjectID, root_kinds: *mut COR_PRF_GC_ROOT_KIND, root_flags: *mut COR_PRF_GC_ROOT_FLAGS, root_ids: *mut UINT_PTR) -> HRESULT { S_OK }
    unsafe fn handle_created(&self, handle_id: GCHandleID, initial_object_id: ObjectID) -> HRESULT { S_OK }
    unsafe fn handle_destroyed(&self, handle_id: GCHandleID) -> HRESULT { S_OK }
}

#[com_interface("4FD2ED52-7731-4b8d-9469-03D2CC3086C5")]
pub trait ICorProfilerCallback3: ICorProfilerCallback2 {
    unsafe fn initialize_for_attach(&self, p_cor_profiler_info_unk: *mut dyn IUnknown, pv_client_data: *mut c_void, cb_client_data: UINT) -> HRESULT { S_OK }
    unsafe fn profiler_attach_complete(&self) -> HRESULT { S_OK }
    unsafe fn profiler_detach_succeeded(&self) -> HRESULT { S_OK }
}

#[com_interface("7B63B2E3-107D-4d48-B2F6-F61E229470D2")]
pub trait ICorProfilerCallback4: ICorProfilerCallback3 {
    unsafe fn re_jit_compilation_started(&self, function_id: FunctionID, rejit_id: ReJITID, f_is_safe_to_block: BOOL) -> HRESULT { S_OK }
    unsafe fn get_re_jit_parameters(&self, module_id: ModuleID, method_id: mdMethodDef, p_function_control: *mut *mut dyn ICorProfilerFunctionControl) -> HRESULT { S_OK }
    unsafe fn re_jit_compilation_finished(&self, function_id: FunctionID, rejit_id: ReJITID, hr_status: HRESULT, f_is_safe_to_block: BOOL) -> HRESULT { S_OK }
    unsafe fn re_jit_error(&self, module_id: ModuleID, method_id: mdMethodDef, function_id: FunctionID, hr_status: HRESULT) -> HRESULT { S_OK }
    unsafe fn moved_references2(&self, c_moved_object_idranges: ULONG, old_object_idrange_start: *mut ObjectID, new_object_idrange_start: *mut ObjectID, c_object_idrange_length: *mut SIZE_T) -> HRESULT { S_OK }
    unsafe fn surviving_references2(&self, c_surviving_object_idranges: ULONG, object_idrange_start: *mut ObjectID, c_object_idrange_length: *mut SIZE_T) -> HRESULT { S_OK }
}

#[com_interface("8DFBA405-8C9F-45F8-BFFA-83B14CEF78B5")]
pub trait ICorProfilerCallback5: ICorProfilerCallback4 {
    unsafe fn conditional_weak_table_element_references(&self, c_root_refs: ULONG, key_ref_ids: *mut ObjectID, value_ref_ids: *mut ObjectID, root_ids: *mut GCHandleID) -> HRESULT { S_OK }
}

#[com_interface("FC13DF4B-4448-4F4F-950C-BA8D19D00C36")]
pub trait ICorProfilerCallback6: ICorProfilerCallback5 {
    unsafe fn get_assembly_references(&self, wsz_assembly_path: *const WCHAR, p_asm_ref_provider: *mut dyn ICorProfilerAssemblyReferenceProvider) -> HRESULT { S_OK }
}

#[com_interface("F76A2DBA-1D52-4539-866C-2AA518F9EFC3")]
pub trait ICorProfilerCallback7: ICorProfilerCallback6 {
    unsafe fn module_in_memory_symbols_updated(&self, module_id: ModuleID) -> HRESULT { S_OK }
}

#[com_interface("5BED9B15-C079-4D47-BFE2-215A140C07E0")]
pub trait ICorProfilerCallback8: ICorProfilerCallback7 {
    unsafe fn dynamic_method_jit_compilation_started(&self, function_id: FunctionID, f_is_safe_to_block: BOOL, p_ilheader: LPCBYTE, cb_ilheader: ULONG) -> HRESULT { S_OK }
    unsafe fn dynamic_method_jit_compilation_finished(&self, function_id: FunctionID, hr_status: HRESULT, f_is_safe_to_block: BOOL) -> HRESULT { S_OK }
}