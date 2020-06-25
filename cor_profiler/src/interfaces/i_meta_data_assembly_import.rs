use std::ffi::c_void;

use com::{
    com_interface,
    interfaces::iunknown::IUnknown,
    sys::HRESULT,
};

use crate::types::*;

#[com_interface("EE62470B-E94B-424E-9B7C-2F00C9249F93")]
pub trait IMetaDataAssemblyImport: IUnknown {
    unsafe fn get_assembly_props(&self, mda: mdAssembly, ppb_public_key: *const *const BYTE, pcb_public_key: *mut ULONG, pul_hash_alg_id: *mut ULONG, sz_name: LPWSTR, cch_name: ULONG, pch_name: *mut ULONG, p_meta_data: *mut ASSEMBLYMETADATA, pdw_assembly_flags: *mut DWORD) -> HRESULT;
    unsafe fn get_assembly_ref_props(&self, mdar: mdAssemblyRef, ppb_public_key_or_token: *const *const BYTE, pcb_public_key_or_token: *mut ULONG, sz_name: LPWSTR, cch_name: ULONG, pch_name: *mut ULONG, p_meta_data: *mut ASSEMBLYMETADATA, ppb_hash_value: *const *const BYTE, pcb_hash_value: *mut ULONG, pdw_assembly_ref_flags: *mut DWORD) -> HRESULT;
    unsafe fn get_file_props(&self, mdf: mdFile, sz_name: LPWSTR, cch_name: ULONG, pch_name: *mut ULONG, ppb_hash_value: *const *const BYTE, pcb_hash_value: *mut ULONG, pdw_file_flags: *mut DWORD) -> HRESULT;
    unsafe fn get_exported_type_props(&self, mdct: mdExportedType, sz_name: LPWSTR, cch_name: ULONG, pch_name: *mut ULONG, ptk_implementation: *mut mdToken, ptk_type_def: *mut mdTypeDef, pdw_exported_type_flags: *mut DWORD) -> HRESULT;
    unsafe fn get_manifest_resource_props(&self, mdmr: mdManifestResource, sz_name: LPWSTR, cch_name: ULONG, pch_name: *mut ULONG, ptk_implementation: *mut mdToken, pdw_offset: *mut DWORD, pdw_resource_flags: *mut DWORD) -> HRESULT;
    unsafe fn enum_assembly_refs(&self, ph_enum: *mut HCORENUM, r_assembly_refs: *mut mdAssemblyRef, c_max: ULONG, pc_tokens: *mut ULONG) -> HRESULT;
    unsafe fn enum_files(&self, ph_enum: *mut HCORENUM, r_files: *mut mdFile, c_max: ULONG, pc_tokens: *mut ULONG) -> HRESULT;
    unsafe fn enum_exported_types(&self, ph_enum: *mut HCORENUM, r_exported_types: *mut mdExportedType, c_max: ULONG, pc_tokens: *mut ULONG) -> HRESULT;
    unsafe fn enum_manifest_resources(&self, ph_enum: *mut HCORENUM, r_manifest_resources: *mut mdManifestResource, c_max: ULONG, pc_tokens: *mut ULONG) -> HRESULT;
    unsafe fn get_assembly_from_scope(&self, ptk_assembly: *mut mdAssembly) -> HRESULT;
    unsafe fn find_exported_type_by_name(&self, sz_name: LPCWSTR, mdt_exported_type: mdToken, ptk_exported_type: *mut mdExportedType) -> HRESULT;
    unsafe fn find_manifest_resource_by_name(&self, sz_name: LPCWSTR, ptk_manifest_resource: *mut mdManifestResource) -> HRESULT;
    unsafe fn close_enum(&self, h_enum: HCORENUM) -> c_void;
    unsafe fn find_assemblies_by_name(&self, sz_app_base: LPCWSTR, sz_private_bin: LPCWSTR, sz_assembly_name: LPCWSTR, pp_iunk: *mut *mut dyn IUnknown, c_max: ULONG, pc_assemblies: *mut ULONG) -> HRESULT;
}