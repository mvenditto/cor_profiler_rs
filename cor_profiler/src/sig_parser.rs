use core::ffi::c_void;
use com::sys::HRESULT;
use crate::types::{
    PCOR_SIGNATURE,
    PCCOR_SIGNATURE,
    ULONG
};

#[link(name = "Native", kind="static")]
extern {
    pub fn parse_signature(callbacks: *mut c_void, signature: PCCOR_SIGNATURE, sig_size: ULONG) -> HRESULT;
}