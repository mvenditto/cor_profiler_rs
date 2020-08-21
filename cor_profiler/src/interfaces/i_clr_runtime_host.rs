use crate::types::*;

use std::ffi::c_void;

extern crate com;
use com::{
    com_interface,
    interfaces::iunknown::IUnknown,
    sys::HRESULT,
};

pub type IHostControl = *mut c_void;
pub type ICLRControl = *mut c_void;
pub type FExecuteInAppDomainCallback = *mut c_void;

#[com_interface("90F1A06C-7712-4762-86B5-7A5EBA6BDB02")]
pub trait ICLRRuntimeHost: IUnknown {
    unsafe fn start(&self) -> HRESULT;
    unsafe fn stop(&self) -> HRESULT;
    unsafe fn set_host_control(&self, p_host_control: *mut IHostControl) -> HRESULT;
    unsafe fn get_clrcontrol(&self, p_clrcontrol: *mut *mut ICLRControl) -> HRESULT;
    unsafe fn unload_app_domain(&self, dw_app_domain_id: DWORD, f_wait_until_done: BOOL) -> HRESULT;
    unsafe fn execute_in_app_domain(&self, dw_app_domain_id: DWORD, p_callback: FExecuteInAppDomainCallback, cookie: *mut c_void) -> HRESULT;
    unsafe fn get_current_app_domain_id(&self, pdw_app_domain_id: *mut DWORD) -> HRESULT;
    unsafe fn execute_application(&self, pwz_app_full_name: LPCWSTR, dw_manifest_paths: DWORD, ppwz_manifest_paths: *mut LPCWSTR, dw_activation_data: DWORD, ppwz_activation_data: *mut LPCWSTR, p_return_value: *mut INT32) -> HRESULT;
    unsafe fn execute_in_default_app_domain(&self, pwz_assembly_path: LPCWSTR, pwz_type_name: LPCWSTR, pwz_method_name: LPCWSTR, pwz_argument: LPCWSTR, p_return_value: *mut DWORD) -> HRESULT;
}