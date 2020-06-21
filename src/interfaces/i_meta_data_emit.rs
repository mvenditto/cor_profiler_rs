use std::ffi::c_void;

extern crate com;
use com::{
    com_interface,
    interfaces::iunknown::IUnknown,
    sys::HRESULT,
};

use crate::types::*;
use crate::interfaces::{
    IMetaDataImport,
    IMetaDataAssemblyEmit,
    IMetaDataAssemblyImport
};

pub trait IStream { } // TODO
pub trait IMapToken { } // TODO

#[allow(dead_code)]
#[repr(C)]
pub enum CorSaveSize {  
    cssAccurate = 0x0000,
    cssQuick = 0x0001,
    cssDiscardTransientCAs = 0x0002  
}

#[repr(C)]
pub struct COR_SECATTR {
    pub tk_ctor: mdMemberRef,
    pub custom_attribute: *const c_void,
    cb_custom_attribute: ULONG
}

#[com_interface("F5DD9950-F693-42e6-830E-7B833E8146A9")]
pub trait IMetaDataEmit: IUnknown {
    unsafe fn set_module_props(&self, sz_name: LPCWSTR) -> HRESULT;
    unsafe fn save(&self, sz_file: LPCWSTR, dw_save_flags: DWORD) -> HRESULT;
    unsafe fn save_to_stream(&self, p_istream: *mut dyn IStream, dw_save_flags: DWORD) -> HRESULT;
    unsafe fn get_save_size(&self, f_save: CorSaveSize, pdw_save_size: *mut DWORD) -> HRESULT;
    unsafe fn define_type_def(&self, sz_type_def: LPCWSTR, dw_type_def_flags: DWORD, tk_extends: mdToken, rtk_implements: *mut mdToken, ptd: *mut mdTypeDef) -> HRESULT;
    unsafe fn define_nested_type(&self, sz_type_def: LPCWSTR, dw_type_def_flags: DWORD, tk_extends: mdToken, rtk_implements: *mut mdToken, td_encloser: mdTypeDef, ptd: *mut mdTypeDef) -> HRESULT;
    unsafe fn set_handler(&self, p_unk: *mut dyn IUnknown) -> HRESULT;
    unsafe fn define_method(&self, td: mdTypeDef, sz_name: LPCWSTR, dw_method_flags: DWORD, pv_sig_blob: PCCOR_SIGNATURE, cb_sig_blob: ULONG, ul_code_rva: ULONG, dw_impl_flags: DWORD, pmd: *mut mdMethodDef) -> HRESULT;
    unsafe fn define_method_impl(&self, td: mdTypeDef, tk_body: mdToken, tk_decl: mdToken) -> HRESULT;
    unsafe fn define_type_ref_by_name(&self, tk_resolution_scope: mdToken, sz_name: LPCWSTR, ptr: *mut mdTypeRef) -> HRESULT;
    unsafe fn define_import_type(&self, p_assem_import: *mut dyn IMetaDataAssemblyImport, pb_hash_value: *const c_void, cb_hash_value: ULONG, p_import: *mut dyn IMetaDataImport, td_import: mdTypeDef, p_assem_emit: *mut dyn IMetaDataAssemblyEmit, ptr: *mut mdTypeRef) -> HRESULT;
    unsafe fn define_member_ref(&self, tk_import: mdToken, sz_name: LPCWSTR, pv_sig_blob: PCCOR_SIGNATURE, cb_sig_blob: ULONG, pmr: *mut mdMemberRef) -> HRESULT;
    unsafe fn define_import_member(&self, p_assem_import: *mut dyn IMetaDataAssemblyImport, pb_hash_value: *const c_void, cb_hash_value: ULONG, p_import: *mut dyn IMetaDataImport, mb_member: mdToken, p_assem_emit: *mut dyn IMetaDataAssemblyEmit, tk_parent: mdToken, pmr: *mut mdMemberRef) -> HRESULT;
    unsafe fn define_event(&self, td: mdTypeDef, sz_event: LPCWSTR, dw_event_flags: DWORD, tk_event_type: mdToken, md_add_on: mdMethodDef, md_remove_on: mdMethodDef, md_fire: mdMethodDef, rmd_other_methods: *mut mdMethodDef, pmd_event: *mut mdEvent) -> HRESULT;
    unsafe fn set_class_layout(&self, td: mdTypeDef, dw_pack_size: DWORD, r_field_offsets: *mut COR_FIELD_OFFSET, ul_class_size: ULONG) -> HRESULT;
    unsafe fn delete_class_layout(&self, td: mdTypeDef) -> HRESULT;
    unsafe fn set_field_marshal(&self, tk: mdToken, pv_native_type: PCCOR_SIGNATURE, cb_native_type: ULONG) -> HRESULT;
    unsafe fn delete_field_marshal(&self, tk: mdToken) -> HRESULT;
    unsafe fn define_permission_set(&self, tk: mdToken, dw_action: DWORD, pv_permission: *mut *const c_void, cb_permission: ULONG, ppm: *mut mdPermission) -> HRESULT;
    unsafe fn set_rva(&self, md: mdMethodDef, ul_rva: ULONG) -> HRESULT;
    unsafe fn get_token_from_sig(&self, pv_sig: PCCOR_SIGNATURE, cb_sig: ULONG, pmsig: *mut mdSignature) -> HRESULT;
    unsafe fn define_module_ref(&self, sz_name: LPCWSTR, pmur: *mut mdModuleRef) -> HRESULT;
    unsafe fn set_parent(&self, mr: mdMemberRef, tk: mdToken) -> HRESULT;
    unsafe fn get_token_from_type_spec(&self, pv_sig: PCCOR_SIGNATURE, cb_sig: ULONG, ptypespec: *mut mdTypeSpec) -> HRESULT;
    unsafe fn save_to_memory(&self, pb_data: *mut c_void, cb_data: ULONG) -> HRESULT;
    unsafe fn define_user_string(&self, sz_string: LPCWSTR, cch_string: ULONG, pstk: *mut mdString) -> HRESULT;
    unsafe fn delete_token(&self, tk_obj: mdToken) -> HRESULT;
    unsafe fn set_method_props(&self, md: mdMethodDef, dw_method_flags: DWORD, ul_code_rva: ULONG, dw_impl_flags: DWORD) -> HRESULT;
    unsafe fn set_type_def_props(&self, td: mdTypeDef, dw_type_def_flags: DWORD, tk_extends: mdToken, rtk_implements: *mut mdToken) -> HRESULT;
    unsafe fn set_event_props(&self, ev: mdEvent, dw_event_flags: DWORD, tk_event_type: mdToken, md_add_on: mdMethodDef, md_remove_on: mdMethodDef, md_fire: mdMethodDef, rmd_other_methods: *mut mdMethodDef) -> HRESULT;
    unsafe fn set_permission_set_props(&self, tk: mdToken, dw_action: DWORD, pv_permission: *mut *const c_void, cb_permission: ULONG, ppm: *mut mdPermission) -> HRESULT;
    unsafe fn define_pinvoke_map(&self, tk: mdToken, dw_mapping_flags: DWORD, sz_import_name: LPCWSTR, mr_import_dll: mdModuleRef) -> HRESULT;
    unsafe fn set_pinvoke_map(&self, tk: mdToken, dw_mapping_flags: DWORD, sz_import_name: LPCWSTR, mr_import_dll: mdModuleRef) -> HRESULT;
    unsafe fn delete_pinvoke_map(&self, tk: mdToken) -> HRESULT;
    unsafe fn define_custom_attribute(&self, tk_owner: mdToken, tk_ctor: mdToken, p_custom_attribute: *mut *const c_void, cb_custom_attribute: ULONG, pcv: *mut mdCustomAttribute) -> HRESULT;
    unsafe fn set_custom_attribute_value(&self, pcv: mdCustomAttribute, p_custom_attribute: *mut *const c_void, cb_custom_attribute: ULONG) -> HRESULT;
    unsafe fn define_field(&self, td: mdTypeDef, sz_name: LPCWSTR, dw_field_flags: DWORD, pv_sig_blob: PCCOR_SIGNATURE, cb_sig_blob: ULONG, dw_cplus_type_flag: DWORD, p_value: *mut *const c_void, cch_value: ULONG, pmd: *mut mdFieldDef) -> HRESULT;
    unsafe fn define_property(&self, td: mdTypeDef, sz_property: LPCWSTR, dw_prop_flags: DWORD, pv_sig: PCCOR_SIGNATURE, cb_sig: ULONG, dw_cplus_type_flag: DWORD, p_value: *mut *const c_void, cch_value: ULONG, md_setter: mdMethodDef, md_getter: mdMethodDef, rmd_other_methods: *mut mdMethodDef, pmd_prop: *mut mdProperty) -> HRESULT;
    unsafe fn define_param(&self, md: mdMethodDef, ul_param_seq: ULONG, sz_name: LPCWSTR, dw_param_flags: DWORD, dw_cplus_type_flag: DWORD, p_value: *mut *const c_void, cch_value: ULONG, ppd: *mut mdParamDef) -> HRESULT;
    unsafe fn set_field_props(&self, fd: mdFieldDef, dw_field_flags: DWORD, dw_cplus_type_flag: DWORD, p_value: *mut *const c_void, cch_value: ULONG) -> HRESULT;
    unsafe fn set_property_props(&self, pr: mdProperty, dw_prop_flags: DWORD, dw_cplus_type_flag: DWORD, p_value: *mut *const c_void, cch_value: ULONG, md_setter: mdMethodDef, md_getter: mdMethodDef, rmd_other_methods: *mut mdMethodDef) -> HRESULT;
    unsafe fn set_param_props(&self, pd: mdParamDef, sz_name: LPCWSTR, dw_param_flags: DWORD, dw_cplus_type_flag: DWORD, p_value: *mut *const c_void, cch_value: ULONG) -> HRESULT;
    unsafe fn define_security_attribute_set(&self, tk_obj: mdToken, r_sec_attrs: *mut COR_SECATTR, c_sec_attrs: ULONG, pul_error_attr: *mut ULONG) -> HRESULT;
    unsafe fn apply_edit_and_continue(&self, p_import: *mut dyn IUnknown) -> HRESULT;
    unsafe fn translate_sig_with_scope(&self, p_assem_import: *mut dyn IMetaDataAssemblyImport, pb_hash_value: *const c_void, cb_hash_value: ULONG, import: *mut dyn IMetaDataImport, pb_sig_blob: PCCOR_SIGNATURE, cb_sig_blob: ULONG, p_assem_emit: *mut dyn IMetaDataAssemblyEmit, emit: *mut dyn IMetaDataEmit, pv_translated_sig: PCOR_SIGNATURE, cb_translated_sig_max: ULONG, pcb_translated_sig: *mut ULONG) -> HRESULT;
    unsafe fn set_method_impl_flags(&self, md: mdMethodDef, dw_impl_flags: DWORD) -> HRESULT;
    unsafe fn set_field_rva(&self, fd: mdFieldDef, ul_rva: ULONG) -> HRESULT;
    unsafe fn merge(&self, p_import: *mut dyn IMetaDataImport, p_host_map_token: *mut dyn IMapToken, p_handler: *mut dyn IUnknown) -> HRESULT;
    unsafe fn merge_end(&self) -> HRESULT;
}

#[com_interface("F5DD9950-F693-42e6-830E-7B833E8146A9")]
pub trait IMetaDataEmit2:  IMetaDataEmit {
    unsafe fn define_method_spec(&self, tk_parent: mdToken, pv_sig_blob: PCCOR_SIGNATURE, cb_sig_blob: ULONG, pmi: *mut mdMethodSpec) -> HRESULT;
    unsafe fn get_delta_save_size(&self, f_save: CorSaveSize, pdw_save_size: *mut DWORD) -> HRESULT;
    unsafe fn save_delta(&self, sz_file: LPCWSTR, dw_save_flags: DWORD) -> HRESULT;
    unsafe fn save_delta_to_stream(&self, p_istream: *mut dyn IStream, dw_save_flags: DWORD) -> HRESULT;
    unsafe fn save_delta_to_memory(&self, pb_data: *mut c_void, cb_data: ULONG) -> HRESULT;
    unsafe fn define_generic_param(&self, tk: mdToken, ul_param_seq: ULONG, dw_param_flags: DWORD, szname: LPCWSTR, reserved: DWORD, rtk_constraints: *mut mdToken, pgp: *mut mdGenericParam) -> HRESULT;
    unsafe fn set_generic_param_props(&self, gp: mdGenericParam, dw_param_flags: DWORD, sz_name: LPCWSTR, reserved: DWORD, rtk_constraints: *mut mdToken) -> HRESULT;
    unsafe fn reset_enclog(&self) -> HRESULT;
}
