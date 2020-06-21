use std::ffi::c_void;

use com::{
    com_interface,
    interfaces::iunknown::IUnknown,
    sys::HRESULT,
};

use crate::types::*;

#[com_interface("211EF15B-5317-4438-B196-DEC87B887693")]
pub trait IMetaDataAssemblyEmit:  IUnknown {
    unsafe fn define_assembly(&self, pb_public_key: *const c_void, cb_public_key: ULONG, ul_hash_alg_id: ULONG, sz_name: LPCWSTR, p_meta_data: *const ASSEMBLYMETADATA, dw_assembly_flags: DWORD, pma: *mut mdAssembly) -> HRESULT;
    unsafe fn define_assembly_ref(&self, pb_public_key_or_token: *const c_void, cb_public_key_or_token: ULONG, sz_name: LPCWSTR, p_meta_data: *const ASSEMBLYMETADATA, pb_hash_value: *const c_void, cb_hash_value: ULONG, dw_assembly_ref_flags: DWORD, pmdar: *mut mdAssemblyRef) -> HRESULT;
    unsafe fn define_file(&self, sz_name: LPCWSTR, pb_hash_value: *const c_void, cb_hash_value: ULONG, dw_file_flags: DWORD, pmdf: *mut mdFile) -> HRESULT;
    unsafe fn define_exported_type(&self, sz_name: LPCWSTR, tk_implementation: mdToken, tk_type_def: mdTypeDef, dw_exported_type_flags: DWORD, pmdct: *mut mdExportedType) -> HRESULT;
    unsafe fn define_manifest_resource(&self, sz_name: LPCWSTR, tk_implementation: mdToken, dw_offset: DWORD, dw_resource_flags: DWORD, pmdmr: *mut mdManifestResource) -> HRESULT;
    unsafe fn set_assembly_props(&self, pma: mdAssembly, pb_public_key: *const c_void, cb_public_key: ULONG, ul_hash_alg_id: ULONG, sz_name: LPCWSTR, p_meta_data: *const ASSEMBLYMETADATA, dw_assembly_flags: DWORD) -> HRESULT;
    unsafe fn set_assembly_ref_props(&self, ar: mdAssemblyRef, pb_public_key_or_token: *const c_void, cb_public_key_or_token: ULONG, sz_name: LPCWSTR, p_meta_data: *const ASSEMBLYMETADATA, pb_hash_value: *const c_void, cb_hash_value: ULONG, dw_assembly_ref_flags: DWORD) -> HRESULT;
    unsafe fn set_file_props(&self, file: mdFile, pb_hash_value: *const c_void, cb_hash_value: ULONG, dw_file_flags: DWORD) -> HRESULT;
    unsafe fn set_exported_type_props(&self, ct: mdExportedType, tk_implementation: mdToken, tk_type_def: mdTypeDef, dw_exported_type_flags: DWORD) -> HRESULT;
    unsafe fn set_manifest_resource_props(&self, mr: mdManifestResource, tk_implementation: mdToken, dw_offset: DWORD, dw_resource_flags: DWORD) -> HRESULT;
}