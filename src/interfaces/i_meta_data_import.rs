use crate::types::*;

use std::ffi::c_void;

extern crate com;
use com::{
    com_interface,
    interfaces::iunknown::IUnknown,
    sys::HRESULT,
};

#[com_interface("7DAC8207-D3AE-4C75-9B67-92801A497D44")]
pub trait IMetaDataImport: IUnknown {
    unsafe fn close_enum(&self, h_enum: HCORENUM) -> c_void;
    unsafe fn count_enum(&self, h_enum: HCORENUM, pul_count: *mut ULONG) -> HRESULT;
    unsafe fn reset_enum(&self, h_enum: HCORENUM, ul_pos: ULONG) -> HRESULT;
    unsafe fn enum_type_defs(&self, ph_enum: *mut HCORENUM, rg_type_defs: *mut mdTypeDef, c_max: ULONG, pc_type_defs: *mut ULONG) -> HRESULT;
    unsafe fn enum_interface_impls(&self, ph_enum: *mut HCORENUM, td: mdTypeDef, r_impls: *mut mdInterfaceImpl, c_max: ULONG, pc_impls: *mut ULONG) -> HRESULT;
    unsafe fn enum_type_refs(&self, ph_enum: *mut HCORENUM, rg_type_refs: *mut mdTypeRef, c_max: ULONG, pc_type_refs: *mut ULONG) -> HRESULT;
    unsafe fn find_type_def_by_name(&self, sz_type_def: LPCWSTR, tk_enclosing_class: mdToken, ptk_type_def: *mut mdTypeDef) -> HRESULT;
    unsafe fn get_scope_props(&self, sz_name: LPWSTR, cch_name: ULONG, pch_name: *mut ULONG, pmvid: *mut GUID) -> HRESULT;
    unsafe fn get_module_from_scope(&self, ptk_module: *mut mdModule) -> HRESULT;
    unsafe fn get_type_def_props(&self, tk_type_def: mdTypeDef, sz_type_def: LPWSTR, cch_type_def: ULONG, pch_type_def: *mut ULONG, pdw_type_def_flags: *mut DWORD, ptk_extends: *mut mdToken) -> HRESULT;
    unsafe fn get_interface_impl_props(&self, tk_interface_impl: mdInterfaceImpl, ptk_class: *mut mdTypeDef, ptk_iface: *mut mdToken) -> HRESULT;
    unsafe fn get_type_ref_props(&self, tk_type_ref: mdTypeRef, ptk_resolution_scope: *mut mdToken, sz_name: LPWSTR, cch_name: ULONG, pch_name: *mut ULONG) -> HRESULT;
    unsafe fn resolve_type_ref(&self, tk_type_ref: mdTypeRef, riid: REFIID, pp_iscope: *mut *mut dyn IUnknown, ptk_type_def: *mut mdTypeDef) -> HRESULT;
    unsafe fn enum_members(&self, ph_enum: *mut HCORENUM, tk_type_def: mdTypeDef, rg_members: *mut mdToken, c_max: ULONG, pc_tokens: *mut ULONG) -> HRESULT;
    unsafe fn enum_members_with_name(&self, ph_enum: *mut HCORENUM, tk_type_def: mdTypeDef, sz_name: LPCWSTR, rg_members: *mut mdToken, c_max: ULONG, pc_tokens: *mut ULONG) -> HRESULT;
    unsafe fn enum_methods(&self, ph_enum: *mut HCORENUM, tk_type_def: mdTypeDef, rg_methods: *mut mdMethodDef, c_max: ULONG, pc_tokens: *mut ULONG) -> HRESULT;
    unsafe fn enum_methods_with_name(&self, ph_enum: *mut HCORENUM, tk_type_def: mdTypeDef, sz_name: LPCWSTR, rg_methods: *mut mdMethodDef, c_max: ULONG, pc_tokens: *mut ULONG) -> HRESULT;
    unsafe fn enum_fields(&self, ph_enum: *mut HCORENUM, tk_type_def: mdTypeDef, rg_fields: *mut mdFieldDef, c_max: ULONG, pc_tokens: *mut ULONG) -> HRESULT;
    unsafe fn enum_fields_with_name(&self, ph_enum: *mut HCORENUM, tk_type_def: mdTypeDef, sz_name: LPCWSTR, r_fields: *mut mdFieldDef, c_max: ULONG, pc_tokens: *mut ULONG) -> HRESULT;
    unsafe fn enum_params(&self, ph_enum: *mut HCORENUM, tk_method_def: mdMethodDef, r_params: *mut mdParamDef, c_max: ULONG, pc_tokens: *mut ULONG) -> HRESULT;
    unsafe fn enum_member_refs(&self, ph_enum: *mut HCORENUM, tk_parent: mdToken, rg_member_refs: *mut mdMemberRef, c_max: ULONG, pc_tokens: *mut ULONG) -> HRESULT;
    unsafe fn enum_method_impls(&self, ph_enum: *mut HCORENUM, tk_type_def: mdTypeDef, r_method_body: *mut mdToken, r_method_decl: *mut mdToken, c_max: ULONG, pc_tokens: *mut ULONG) -> HRESULT;
    unsafe fn enum_permission_sets(&self, ph_enum: *mut HCORENUM, tk: mdToken, dw_actions: DWORD, r_permission: *mut mdPermission, c_max: ULONG, pc_tokens: *mut ULONG) -> HRESULT;
    unsafe fn find_member(&self, tk_type_def: mdTypeDef, sz_name: LPCWSTR, pv_sig_blob: PCCOR_SIGNATURE, cb_sig_blob: ULONG, pmb: *mut mdToken) -> HRESULT;
    unsafe fn find_method(&self, tk_type_def: mdTypeDef, sz_name: LPCWSTR, pv_sig_blob: PCCOR_SIGNATURE, cb_sig_blob: ULONG, ptk_method_def: *mut mdMethodDef) -> HRESULT;
    unsafe fn find_field(&self, tk_type_def: mdTypeDef, sz_name: LPCWSTR, pv_sig_blob: PCCOR_SIGNATURE, cb_sig_blob: ULONG, ptk_field_def: *mut mdFieldDef) -> HRESULT;
    unsafe fn find_member_ref(&self, tk_type_ref: mdTypeRef, sz_name: LPCWSTR, pv_sig_blob: PCCOR_SIGNATURE, cb_sig_blob: ULONG, p_member_ref: *mut mdMemberRef) -> HRESULT;
    unsafe fn get_method_props(&self, tk_method_def: mdMethodDef, ptk_class: *mut mdTypeDef, sz_method: LPWSTR, cch_method: ULONG, pch_method: *mut ULONG, pdw_attr: *mut DWORD, ppv_sig_blob: *mut PCCOR_SIGNATURE, pcb_sig_blob: *mut ULONG, pul_code_rva: *mut ULONG, pdw_impl_flags: *mut DWORD) -> HRESULT;
    unsafe fn get_member_ref_props(&self, tk_member_ref: mdMemberRef, ptk: *mut mdToken, sz_member: LPWSTR, cch_member: ULONG, pch_member: *mut ULONG, ppv_sig_blob: *mut PCCOR_SIGNATURE, pcb_sig_blob: *mut ULONG) -> HRESULT;
    unsafe fn enum_properties(&self, ph_enum: *mut HCORENUM, tk_typ_def: mdTypeDef, rg_properties: *mut mdProperty, c_max: ULONG, pc_properties: *mut ULONG) -> HRESULT;
    unsafe fn enum_events(&self, ph_enum: *mut HCORENUM, tk_typ_def: mdTypeDef, rg_events: *mut mdEvent, c_max: ULONG, pc_events: *mut ULONG) -> HRESULT;
    unsafe fn get_event_props(&self, tk_event: mdEvent, ptk_class: *mut mdTypeDef, sz_event: LPWSTR, cch_event: ULONG, pch_event: *mut ULONG, pdw_event_flags: *mut DWORD, ptk_event_type: *mut mdToken, ptk_add_on: *mut mdMethodDef, ptk_remove_on: *mut mdMethodDef, tkk_fire: *mut mdMethodDef, rg_other_method: *mut mdMethodDef, c_max: ULONG, pc_other_method: *mut ULONG) -> HRESULT;
    unsafe fn enum_method_semantics(&self, ph_enum: *mut HCORENUM, tk_method_def: mdMethodDef, rg_event_prop: *mut mdToken, c_max: ULONG, pc_event_prop: *mut ULONG) -> HRESULT;
    unsafe fn get_method_semantics(&self, tk_method_def: mdMethodDef, tk_event_prop: mdToken, pdw_semantics_flags: *mut DWORD) -> HRESULT;
    unsafe fn get_class_layout(&self, tk_type_def: mdTypeDef, pdw_pack_size: *mut DWORD, rg_field_offset: *mut COR_FIELD_OFFSET, c_max: ULONG, pc_field_offset: *mut ULONG, pul_class_size: *mut ULONG) -> HRESULT;
    unsafe fn get_field_marshal(&self, tk: mdToken, ppv_native_type: *mut PCCOR_SIGNATURE, pcb_native_type: *mut ULONG) -> HRESULT;
    unsafe fn get_rva(&self, tk: mdToken, pul_code_rva: *mut ULONG, pdw_impl_flags: *mut DWORD) -> HRESULT;
    unsafe fn get_permission_set_props(&self, tk: mdPermission, pdw_action: *mut DWORD, ppv_permission: *const *const BYTE, pcb_permission: *mut ULONG) -> HRESULT;
    unsafe fn get_sig_from_token(&self, tk_signature: mdSignature, ppv_sig: *mut PCCOR_SIGNATURE, pcb_sig: *mut ULONG) -> HRESULT;
    unsafe fn get_module_ref_props(&self, tk_module_ref: mdModuleRef, sz_name: LPWSTR, cch_name: ULONG, pch_name: *mut ULONG) -> HRESULT;
    unsafe fn enum_module_refs(&self, ph_enum: *mut HCORENUM, rg_module_refs: *mut mdModuleRef, c_max: ULONG, pc_module_refs: *mut ULONG) -> HRESULT;
    unsafe fn get_type_spec_from_token(&self, tk_type_spec: mdTypeSpec, ppv_sig: *mut PCCOR_SIGNATURE, pcb_sig: *mut ULONG) -> HRESULT;
    unsafe fn get_name_from_token(&self, tk: mdToken, psz_utf8_name_ptr: *mut MDUTF8CSTR) -> HRESULT;
    unsafe fn enum_unresolved_methods(&self, ph_enum: *mut HCORENUM, rg_methods: *mut mdToken, c_max: ULONG, pc_tokens: *mut ULONG) -> HRESULT;
    unsafe fn get_user_string(&self, tk_string: mdString, sz_string: LPWSTR, cch_string: ULONG, pch_string: *mut ULONG) -> HRESULT;
    unsafe fn get_pinvoke_map(&self, tk: mdToken, pdw_mapping_flags: *mut DWORD, sz_import_name: LPWSTR, cch_import_name: ULONG, pch_import_name: *mut ULONG, ptk_import_dll: *mut mdModuleRef) -> HRESULT;
    unsafe fn enum_signatures(&self, ph_enum: *mut HCORENUM, rg_signatures: *mut mdSignature, c_max: ULONG, pc_signatures: *mut ULONG) -> HRESULT;
    unsafe fn enum_type_specs(&self, ph_enum: *mut HCORENUM, rg_type_specs: *mut mdTypeSpec, c_max: ULONG, pc_type_specs: *mut ULONG) -> HRESULT;
    unsafe fn enum_user_strings(&self, ph_enum: *mut HCORENUM, rg_strings: *mut mdString, c_max: ULONG, pc_strings: *mut ULONG) -> HRESULT;
    unsafe fn get_param_for_method_index(&self, tk_method_def: mdMethodDef, ul_param_seq: ULONG, ptk_param_def: *mut mdParamDef) -> HRESULT;
    unsafe fn enum_custom_attributes(&self, ph_enum: *mut HCORENUM, tk: mdToken, tk_type: mdToken, rg_custom_attributes: *mut mdCustomAttribute, c_max: ULONG, pc_custom_attributes: *mut ULONG) -> HRESULT;
    unsafe fn get_custom_attribute_props(&self, cv: mdCustomAttribute, ptk_obj: *mut mdToken, ptk_type: *mut mdToken, pp_blob: *const *const BYTE, pcb_blob: *mut ULONG) -> HRESULT;
    unsafe fn find_type_ref(&self, tk_resolution_scope: mdToken, sz_name: LPCWSTR, tk_type_ref: *mut mdTypeRef) -> HRESULT;
    unsafe fn get_member_props(&self, tk_member: mdToken, ptk_type_def: *mut mdTypeDef, sz_member: LPWSTR, cch_member: ULONG, pch_member: *mut ULONG, pdw_attr: *mut DWORD, ppv_sig_blob: *mut PCCOR_SIGNATURE, pcb_sig_blob: *mut ULONG, pul_code_rva: *mut ULONG, pdw_impl_flags: *mut DWORD, pdw_cplus_type_flag: *mut DWORD, pp_value: *mut UVCP_CONSTANT, pcch_value: *mut ULONG) -> HRESULT;
    unsafe fn get_field_props(&self, tk_field_def: mdFieldDef, ptk_type_def: *mut mdTypeDef, sz_field: LPWSTR, cch_field: ULONG, pch_field: *mut ULONG, pdw_attr: *mut DWORD, ppv_sig_blob: *mut PCCOR_SIGNATURE, pcb_sig_blob: *mut ULONG, pdw_cplus_type_flag: *mut DWORD, pp_value: *mut UVCP_CONSTANT, pcch_value: *mut ULONG) -> HRESULT;
    unsafe fn get_property_props(&self, prop: mdProperty, ptk_type_def: *mut mdTypeDef, sz_property: LPWSTR, cch_property: ULONG, pch_property: *mut ULONG, pdw_prop_flags: *mut DWORD, ppv_sig_blob: *mut PCCOR_SIGNATURE, pcb_sig_blob: *mut ULONG, pdw_cplus_type_flag: *mut DWORD, pp_default_value: *mut UVCP_CONSTANT, pcch_default_value: *mut ULONG, ptk_setter: *mut mdMethodDef, ptk_getter: *mut mdMethodDef, rg_other_method: *mut mdMethodDef, c_max: ULONG, pc_other_method: *mut ULONG) -> HRESULT;
    unsafe fn get_param_props(&self, tk_param_def: mdParamDef, ptk_method_def: *mut mdMethodDef, pul_sequence: *mut ULONG, sz_name: LPWSTR, cch_name: ULONG, pch_name: *mut ULONG, pdw_attr: *mut DWORD, pdw_cplus_type_flag: *mut DWORD, pp_value: *mut UVCP_CONSTANT, pcch_value: *mut ULONG) -> HRESULT;
    unsafe fn get_custom_attribute_by_name(&self, tk_obj: mdToken, sz_name: LPCWSTR, pp_data: *const *const BYTE, pcb_data: *mut ULONG) -> HRESULT;
    unsafe fn is_valid_token(&self, tk: mdToken) -> BOOL;
    unsafe fn get_nested_class_props(&self, td_nested_class: mdTypeDef, ptd_enclosing_class: *mut mdTypeDef) -> HRESULT;
    unsafe fn get_native_call_conv_from_sig(&self, pv_sig: *const BYTE, cb_sig: ULONG, p_call_conv: *mut ULONG) -> HRESULT;
    unsafe fn is_global(&self, tk: mdToken, pb_is_global: *mut INT) -> HRESULT;
}