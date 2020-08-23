use crate::types::*;

use crate::types::{
    LPWSTR,
    LPVOID,
    HMODULE,
    DWORD,
    HANDLE
};

use std::ffi::c_void;

extern crate com;
use com::{
    com_interface,
    interfaces::iunknown::IUnknown,
    sys::HRESULT,
};

#[com_interface("BD39D1D2-BA2F-486a-89B0-B4B0CB466891")]
pub trait ICLRRuntimeInfo: IUnknown {
    unsafe fn get_version_string(&self, pwz_buffer: LPWSTR, pcch_buffer: *mut DWORD) -> HRESULT;
    unsafe fn get_runtime_directory(&self, pwz_buffer: LPWSTR, pcch_buffer: *mut DWORD) -> HRESULT;
    unsafe fn is_loaded(&self, hnd_process: HANDLE, pb_loaded: *mut BOOL) -> HRESULT;
    unsafe fn load_error_string(&self, i_resource_id: UINT, pwz_buffer: LPWSTR, pcch_buffer: *mut DWORD, i_locale_id: LONG) -> HRESULT;
    unsafe fn load_library(&self, pwz_dll_name: LPCWSTR, phnd_module: *mut HMODULE) -> HRESULT;
    unsafe fn get_proc_address(&self, psz_proc_name: LPCSTR, pp_proc: *mut LPVOID) -> HRESULT;
    unsafe fn get_interface2(&self, rclsid: REFCLSID, riid: REFIID, pp_unk: *mut LPVOID) -> HRESULT;
    unsafe fn is_loadable(&self, pb_loadable: *mut BOOL) -> HRESULT;
    unsafe fn set_default_startup_flags(&self, dw_startup_flags: DWORD, pwz_host_config_file: LPCWSTR) -> HRESULT;
    unsafe fn get_default_startup_flags(&self, pdw_startup_flags: *mut DWORD, pwz_host_config_file: LPWSTR, pcch_host_config_file: *mut DWORD) -> HRESULT;
    unsafe fn bind_as_legacy_v2_runtime(&self) -> HRESULT;
    unsafe fn is_started(&self, pb_started: *mut BOOL, pdw_startup_flags: *mut DWORD) -> HRESULT;
}