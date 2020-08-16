use crate::types::*;

use std::ffi::c_void;

extern crate com;
use com::{
    com_interface,
    interfaces::iunknown::IUnknown,
    sys::HRESULT,
};


#[com_interface("A0EFB28B-6EE2-4d7b-B983-A75EF7BEEDB8")]
pub trait IMethodMalloc: IUnknown {
    unsafe fn alloc(&self, cb: ULONG) -> PVOID;
}

type CorElementType = ULONG;

#[com_interface("28B5557D-3F3F-48b4-90B2-5F9EEA2F6C48")]
pub trait ICorProfilerInfo: IUnknown {
    unsafe fn get_class_from_object(&self, object_id: ObjectID, p_class_id: *mut ClassID) -> HRESULT;
    unsafe fn get_class_from_token(&self, module_id: ModuleID, type_def: mdTypeDef, p_class_id: *mut ClassID) -> HRESULT;
    unsafe fn get_code_info(&self, function_id: FunctionID, p_start: *mut LPCBYTE, pc_size: *mut ULONG) -> HRESULT;
    unsafe fn get_event_mask(&self, pdw_events: *mut DWORD) -> HRESULT;
    unsafe fn get_function_from_ip(&self, ip: LPCBYTE, p_function_id: *mut FunctionID) -> HRESULT;
    unsafe fn get_function_from_token(&self, module_id: ModuleID, token: mdToken, p_function_id: *mut FunctionID) -> HRESULT;
    unsafe fn get_handle_from_thread(&self, thread_id: ThreadID, ph_thread: *mut HANDLE) -> HRESULT;
    unsafe fn get_object_size(&self, object_id: ObjectID, pc_size: *mut ULONG) -> HRESULT;
    unsafe fn is_array_class(&self, class_id: ClassID, p_base_elem_type: *mut CorElementType, p_base_class_id: *mut ClassID, pc_rank: *mut ULONG) -> HRESULT;
    unsafe fn get_thread_info(&self, thread_id: ThreadID, pdw_win32_thread_id: *mut DWORD) -> HRESULT;
    unsafe fn get_current_thread_id(&self, p_thread_id: *mut ThreadID) -> HRESULT;
    unsafe fn get_class_idinfo(&self, class_id: ClassID, p_module_id: *mut ModuleID, p_type_def_token: *mut mdTypeDef) -> HRESULT;
    unsafe fn get_function_info(&self, function_id: FunctionID, p_class_id: *mut ClassID, p_module_id: *mut ModuleID, p_token: *mut mdToken) -> HRESULT;
    unsafe fn set_event_mask(&self, dw_events: DWORD) -> HRESULT;
    unsafe fn set_enter_leave_function_hooks(&self, p_func_enter: *mut FunctionEnter, p_func_leave: *mut FunctionLeave, p_func_tailcall: *mut FunctionTailcall) -> HRESULT;
    unsafe fn set_function_idmapper(&self, p_func: *mut FunctionIDMapper) -> HRESULT;
    unsafe fn get_token_and_meta_data_from_function(&self, function_id: FunctionID, riid: REFIID, pp_import: *mut *mut dyn IUnknown, p_token: *mut mdToken) -> HRESULT;
    unsafe fn get_module_info(&self, module_id: ModuleID, pp_base_load_address: *mut LPCBYTE, cch_name: ULONG, pcch_name: *mut ULONG, sz_name: *mut WCHAR, p_assembly_id: *mut AssemblyID) -> HRESULT;
    unsafe fn get_module_meta_data(&self, module_id: ModuleID, dw_open_flags: DWORD, riid: REFIID, pp_out: *mut *mut c_void) -> HRESULT;
    unsafe fn get_il_function_body(&self, module_id: ModuleID, method_id: mdMethodDef, pp_method_header: *mut LPCBYTE, pcb_method_size: *mut ULONG) -> HRESULT;
    unsafe fn get_ilfunction_body_allocator(&self, module_id: ModuleID, pp_malloc: *mut *mut dyn IMethodMalloc) -> HRESULT;
    unsafe fn set_ilfunction_body(&self, module_id: ModuleID, methodid: mdMethodDef, pb_new_ilmethod_header: LPCBYTE) -> HRESULT;
    unsafe fn get_app_domain_info(&self, app_domain_id: AppDomainID, cch_name: ULONG, pcch_name: *mut ULONG, sz_name: *mut WCHAR, p_process_id: *mut ProcessID) -> HRESULT;
    unsafe fn get_assembly_info(&self, assembly_id: AssemblyID, cch_name: ULONG, pcch_name: *mut ULONG, sz_name: *mut WCHAR, p_app_domain_id: *mut AppDomainID, p_module_id: *mut ModuleID) -> HRESULT;
    unsafe fn set_function_re_jit(&self, function_id: FunctionID) -> HRESULT;
    unsafe fn force_gc(&self) -> HRESULT;
    unsafe fn set_ilinstrumented_code_map(&self, function_id: FunctionID, f_start_jit: BOOL, c_ilmap_entries: ULONG, rg_ilmap_entries: *mut COR_IL_MAP) -> HRESULT;
    unsafe fn get_inproc_inspection_interface(&self, ppicd: *mut *mut dyn IUnknown) -> HRESULT;
    unsafe fn get_inproc_inspection_ithis_thread(&self, ppicd: *mut *mut dyn IUnknown) -> HRESULT;
    unsafe fn get_thread_context(&self, thread_id: ThreadID, p_context_id: *mut ContextID) -> HRESULT;
    unsafe fn begin_inproc_debugging(&self, f_this_thread_only: BOOL, pdw_profiler_context: *mut DWORD) -> HRESULT;
    unsafe fn end_inproc_debugging(&self, dw_profiler_context: DWORD) -> HRESULT;
    unsafe fn get_ilto_native_mapping(&self, function_id: FunctionID, c_map: ULONG32, pc_map: *mut ULONG32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> HRESULT;
}

#[com_interface("CC0935CD-A518-487d-B0BB-A93214E65478")]
pub trait ICorProfilerInfo2: ICorProfilerInfo {
    unsafe fn do_stack_snapshot(&self, thread: ThreadID, callback: *mut StackSnapshotCallback, info_flags: ULONG32, client_data: *mut c_void, context: *mut BYTE, context_size: ULONG32) -> HRESULT;
    unsafe fn set_enter_leave_function_hooks2(&self, p_func_enter: *mut FunctionEnter2, p_func_leave: *mut FunctionLeave2, p_func_tailcall: *mut FunctionTailcall2) -> HRESULT;
    unsafe fn get_function_info2(&self, func_id: FunctionID, frame_info: COR_PRF_FRAME_INFO, p_class_id: *mut ClassID, p_module_id: *mut ModuleID, p_token: *mut mdToken, c_type_args: ULONG32, pc_type_args: *mut ULONG32, type_args: *mut ClassID) -> HRESULT;
    unsafe fn get_string_layout(&self, p_buffer_length_offset: *mut ULONG, p_string_length_offset: *mut ULONG, p_buffer_offset: *mut ULONG) -> HRESULT;
    unsafe fn get_class_layout(&self, class_id: ClassID, r_field_offset: *mut COR_FIELD_OFFSET, c_field_offset: ULONG, pc_field_offset: *mut ULONG, pul_class_size: *mut ULONG) -> HRESULT;
    unsafe fn get_class_idinfo2(&self, class_id: ClassID, p_module_id: *mut ModuleID, p_type_def_token: *mut mdTypeDef, p_parent_class_id: *mut ClassID, c_num_type_args: ULONG32, pc_num_type_args: *mut ULONG32, type_args: *mut ClassID) -> HRESULT;
    unsafe fn get_code_info2(&self, function_id: FunctionID, c_code_infos: ULONG32, pc_code_infos: *mut ULONG32, code_infos: *mut COR_PRF_CODE_INFO) -> HRESULT;
    unsafe fn get_class_from_token_and_type_args(&self, module_id: ModuleID, type_def: mdTypeDef, c_type_args: ULONG32, type_args: *mut ClassID, p_class_id: *mut ClassID) -> HRESULT;
    unsafe fn get_function_from_token_and_type_args(&self, module_id: ModuleID, func_def: mdMethodDef, class_id: ClassID, c_type_args: ULONG32, type_args: *mut ClassID, p_function_id: *mut FunctionID) -> HRESULT;
    unsafe fn enum_module_frozen_objects(&self, module_id: ModuleID, pp_enum: *mut *mut dyn ICorProfilerObjectEnum) -> HRESULT;
    unsafe fn get_array_object_info(&self, object_id: ObjectID, c_dimensions: ULONG32, p_dimension_sizes: *mut ULONG32, p_dimension_lower_bounds: *mut INT, pp_data: *mut *mut BYTE) -> HRESULT;
    unsafe fn get_box_class_layout(&self, class_id: ClassID, p_buffer_offset: *mut ULONG32) -> HRESULT;
    unsafe fn get_thread_app_domain(&self, thread_id: ThreadID, p_app_domain_id: *mut AppDomainID) -> HRESULT;
    unsafe fn get_rvastatic_address(&self, class_id: ClassID, field_token: mdFieldDef, pp_address: *mut *mut c_void) -> HRESULT;
    unsafe fn get_app_domain_static_address(&self, class_id: ClassID, field_token: mdFieldDef, app_domain_id: AppDomainID, pp_address: *mut *mut c_void) -> HRESULT;
    unsafe fn get_thread_static_address(&self, class_id: ClassID, field_token: mdFieldDef, thread_id: ThreadID, pp_address: *mut *mut c_void) -> HRESULT;
    unsafe fn get_context_static_address(&self, class_id: ClassID, field_token: mdFieldDef, context_id: ContextID, pp_address: *mut *mut c_void) -> HRESULT;
    unsafe fn get_static_field_info(&self, class_id: ClassID, field_token: mdFieldDef, p_field_info: *mut COR_PRF_STATIC_TYPE) -> HRESULT;
    unsafe fn get_generation_bounds(&self, c_object_ranges: ULONG, pc_object_ranges: *mut ULONG, ranges: *mut COR_PRF_GC_GENERATION_RANGE) -> HRESULT;
    unsafe fn get_object_generation(&self, object_id: ObjectID, range: *mut COR_PRF_GC_GENERATION_RANGE) -> HRESULT;
    unsafe fn get_notified_exception_clause_info(&self, pinfo: *mut COR_PRF_EX_CLAUSE_INFO) -> HRESULT;
}

#[com_interface("B555ED4F-452A-4E54-8B39-B5360BAD32A0")]
pub trait ICorProfilerInfo3: ICorProfilerInfo2 {
    unsafe fn enum_jited_functions(&self, pp_enum: *mut *mut dyn ICorProfilerFunctionEnum) -> HRESULT;
    unsafe fn request_profiler_detach(&self, dw_expected_completion_milliseconds: DWORD) -> HRESULT;
    unsafe fn set_function_idmapper2(&self, p_func: *mut FunctionIDMapper2, client_data: *mut c_void) -> HRESULT;
    unsafe fn get_string_layout2(&self, p_string_length_offset: *mut ULONG, p_buffer_offset: *mut ULONG) -> HRESULT;
    unsafe fn set_enter_leave_function_hooks3(&self, p_func_enter3: *mut FunctionEnter3, p_func_leave3: *mut FunctionLeave3, p_func_tailcall3: *mut FunctionTailcall3) -> HRESULT;
    unsafe fn set_enter_leave_function_hooks3_with_info(&self, p_func_enter3_with_info: *mut FunctionEnter3WithInfo, p_func_leave3_with_info: *mut FunctionLeave3WithInfo, p_func_tailcall3_with_info: *mut FunctionTailcall3WithInfo) -> HRESULT;
    unsafe fn get_function_enter3_info(&self, function_id: FunctionID, elt_info: COR_PRF_ELT_INFO, p_frame_info: *mut COR_PRF_FRAME_INFO, pcb_argument_info: *mut ULONG, p_argument_info: *mut COR_PRF_FUNCTION_ARGUMENT_INFO) -> HRESULT;
    unsafe fn get_function_leave3_info(&self, function_id: FunctionID, elt_info: COR_PRF_ELT_INFO, p_frame_info: *mut COR_PRF_FRAME_INFO, p_retval_range: *mut COR_PRF_FUNCTION_ARGUMENT_RANGE) -> HRESULT;
    unsafe fn get_function_tailcall3_info(&self, function_id: FunctionID, elt_info: COR_PRF_ELT_INFO, p_frame_info: *mut COR_PRF_FRAME_INFO) -> HRESULT;
    unsafe fn enum_modules(&self, pp_enum: *mut *mut dyn ICorProfilerModuleEnum) -> HRESULT;
    unsafe fn get_runtime_information(&self, p_clr_instance_id: *mut USHORT, p_runtime_type: *mut COR_PRF_RUNTIME_TYPE, p_major_version: *mut USHORT, p_minor_version: *mut USHORT, p_build_number: *mut USHORT, p_qfeversion: *mut USHORT, cch_version_string: ULONG, pcch_version_string: *mut ULONG, sz_version_string: *mut WCHAR) -> HRESULT;
    unsafe fn get_thread_static_address2(&self, class_id: ClassID, field_token: mdFieldDef, app_domain_id: AppDomainID, thread_id: ThreadID, pp_address: *mut *mut c_void) -> HRESULT;
    unsafe fn get_app_domains_containing_module(&self, module_id: ModuleID, c_app_domain_ids: ULONG32, pc_app_domain_ids: *mut ULONG32, app_domain_ids: *mut AppDomainID) -> HRESULT;
    unsafe fn get_module_info2(&self, module_id: ModuleID, pp_base_load_address: *mut LPCBYTE, cch_name: ULONG, pcch_name: *mut ULONG, sz_name: *mut WCHAR, p_assembly_id: *mut AssemblyID, pdw_module_flags: *mut DWORD) -> HRESULT;
}

#[com_interface("0d8fdcaa-6257-47bf-b1bf-94dac88466ee")]
pub trait ICorProfilerInfo4: ICorProfilerInfo3 {
    unsafe fn enum_threads(&self, pp_enum: *mut *mut dyn ICorProfilerThreadEnum) -> HRESULT;
    unsafe fn initialize_current_thread(&self) -> HRESULT;
    unsafe fn request_re_jit(&self, c_functions: ULONG, module_ids: *mut ModuleID, method_ids: *mut mdMethodDef) -> HRESULT;
    unsafe fn request_revert(&self, c_functions: ULONG, module_ids: *mut ModuleID, method_ids: *mut mdMethodDef, status: *mut HRESULT) -> HRESULT;
    unsafe fn get_code_info3(&self, function_id: FunctionID, re_jit_id: ReJITID, c_code_infos: ULONG32, pc_code_infos: *mut ULONG32, code_infos: *mut COR_PRF_CODE_INFO) -> HRESULT;
    unsafe fn get_function_from_ip2(&self, ip: LPCBYTE, p_function_id: *mut FunctionID, p_re_jit_id: *mut ReJITID) -> HRESULT;
    unsafe fn get_re_jitids(&self, function_id: FunctionID, c_re_jit_ids: ULONG, pc_re_jit_ids: *mut ULONG, re_jit_ids: *mut ReJITID) -> HRESULT;
    unsafe fn get_ilto_native_mapping2(&self, function_id: FunctionID, re_jit_id: ReJITID, c_map: ULONG32, pc_map: *mut ULONG32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> HRESULT;
    unsafe fn enum_jited_functions2(&self, pp_enum: *mut *mut dyn ICorProfilerFunctionEnum) -> HRESULT;
    unsafe fn get_object_size2(&self, object_id: ObjectID, pc_size: *mut SIZE_T) -> HRESULT;
}

#[com_interface("07602928-CE38-4B83-81E7-74ADAF781214")]
pub trait ICorProfilerInfo5: ICorProfilerInfo4 {
    unsafe fn get_event_mask2(&self, pdw_events_low: *mut DWORD, pdw_events_high: *mut DWORD) -> HRESULT;
    unsafe fn set_event_mask2(&self, dw_events_low: DWORD, dw_events_high: DWORD) -> HRESULT;
}

#[com_interface("F30A070D-BFFB-46A7-B1D8-8781EF7B698A")]
pub trait ICorProfilerInfo6: ICorProfilerInfo5 {
    unsafe fn enum_ngen_module_methods_inlining_this_method(&self, inliners_module_id: ModuleID, inlinee_module_id: ModuleID, inlinee_method_id: mdMethodDef, incomplete_data: *mut BOOL, pp_enum: *mut *mut dyn ICorProfilerMethodEnum) -> HRESULT;
}

#[com_interface("9AEECC0D-63E0-4187-8C00-E312F503F663")]
pub trait ICorProfilerInfo7: ICorProfilerInfo6 {
    unsafe fn apply_meta_data(&self, module_id: ModuleID) -> HRESULT;
    unsafe fn get_in_memory_symbols_length(&self, module_id: ModuleID, p_count_symbol_bytes: *mut DWORD) -> HRESULT;
    unsafe fn read_in_memory_symbols(&self, module_id: ModuleID, symbols_read_offset: DWORD, p_symbol_bytes: *mut BYTE, count_symbol_bytes: DWORD, p_count_symbol_bytes_read: *mut DWORD) -> HRESULT;
}

#[com_interface("C5AC80A6-782E-4716-8044-39598C60CFBF")]
pub trait ICorProfilerInfo8: ICorProfilerInfo7 {
    unsafe fn is_function_dynamic(&self, function_id: FunctionID, is_dynamic: *mut BOOL) -> HRESULT;
    unsafe fn get_function_from_ip3(&self, ip: LPCBYTE, function_id: *mut FunctionID, p_re_jit_id: *mut ReJITID) -> HRESULT;
    unsafe fn get_dynamic_function_info(&self, function_id: FunctionID, module_id: *mut ModuleID, ppv_sig: *mut PCCOR_SIGNATURE, pb_sig: *mut ULONG, cch_name: ULONG, pcch_name: *mut ULONG, wsz_name: *mut WCHAR) -> HRESULT;
}

#[com_interface("008170db-f8cc-4796-9a51-dc8aa0b47012")]
pub trait ICorProfilerInfo9: ICorProfilerInfo8 {
    unsafe fn get_native_code_start_addresses(&self, function_id: FunctionID, re_jit_id: ReJITID, c_code_start_addresses: ULONG32, pc_code_start_addresses: *mut ULONG32, code_start_addresses: *mut UINT_PTR) -> HRESULT;
    unsafe fn get_ilto_native_mapping3(&self, p_native_code_start_address: UINT_PTR, c_map: ULONG32, pc_map: *mut ULONG32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> HRESULT;
    unsafe fn get_code_info4(&self, p_native_code_start_address: UINT_PTR, c_code_infos: ULONG32, pc_code_infos: *mut ULONG32, code_infos: *mut COR_PRF_CODE_INFO) -> HRESULT;
}

#[com_interface("2F1B5152-C869-40C9-AA5F-3ABE026BD720")]
pub trait ICorProfilerInfo10: ICorProfilerInfo9 {
    unsafe fn enumerate_object_references(&self, object_id: ObjectID, callback: ObjectReferenceCallback, client_data: *mut c_void) -> HRESULT;
    unsafe fn is_frozen_object(&self, object_id: ObjectID, pb_frozen: *mut BOOL) -> HRESULT;
    unsafe fn get_lohobject_size_threshold(&self, p_threshold: *mut DWORD) -> HRESULT;
    unsafe fn request_re_jit_with_inliners(&self, dw_rejit_flags: DWORD, c_functions: ULONG, module_ids: *mut ModuleID, method_ids: *mut mdMethodDef) -> HRESULT;
    unsafe fn suspend_runtime(&self) -> HRESULT;
    unsafe fn resume_runtime(&self) -> HRESULT;
}