use crate::types::*;

use std::ffi::c_void;

extern crate com;
use com::{
    com_interface,
    interfaces::iunknown::IUnknown,
    sys::HRESULT,
};

#[com_interface("809C652E-7396-11D2-9771-00A0C9B4D50C")]
pub trait IMetaDataDispenser: IUnknown {
    unsafe fn define_scope(&self, rclsid: REFCLSID, dw_create_flags: DWORD, riid: REFIID, ppiunkn: *mut *mut c_void) -> HRESULT;
    unsafe fn open_scope(&self, scope: LPCWSTR, dw_open_flags: DWORD, riid: REFIID, ppiunkn: *mut *mut c_void) -> HRESULT;
}