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
    ComPtr,
    interfaces::iunknown::IUnknown,
    sys::HRESULT,
};

pub type RuntimeLoadedCallbackFnPtr = *mut c_void;

#[com_interface("00000100-0000-0000-C000-000000000046")]
pub trait IEnumUnknown: IUnknown {
    unsafe fn next(&self, celt: ULONG, rgelt: *mut *mut dyn IUnknown, pcelt_fetched: *mut ULONG) -> HRESULT;
    unsafe fn skip(&self, celt: ULONG) -> HRESULT;
    unsafe fn reset(&self) -> HRESULT;
    unsafe fn clone_enum(&self, ppenum: *mut *mut dyn IEnumUnknown) -> HRESULT;
}

#[com_interface("D332DB9E-B9B3-4125-8207-A14884F53216")]
pub trait ICLRMetaHost: IUnknown {
    unsafe fn get_runtime(&self, pwz_version: LPCWSTR, riid: REFIID, pp_runtime: *mut LPVOID) -> HRESULT;
    unsafe fn get_version_from_file(&self, pwz_file_path: LPCWSTR, pwz_buffer: LPWSTR, pcch_buffer: *mut DWORD) -> HRESULT;
    unsafe fn enumerate_installed_runtimes(&self, pp_enumerator: *mut *mut dyn IEnumUnknown) -> HRESULT;
    unsafe fn enumerate_loaded_runtimes(&self, hnd_process: HANDLE, pp_enumerator: *mut *mut dyn IEnumUnknown) -> HRESULT;
    unsafe fn request_runtime_loaded_notification(&self, p_callback_function: RuntimeLoadedCallbackFnPtr) -> HRESULT;
    unsafe fn query_legacy_v2_runtime_binding(&self, riid: REFIID, pp_unk: *mut LPVOID) -> HRESULT;
    unsafe fn exit_process(&self, i_exit_code: INT32) -> HRESULT;
}