use std::{
    ffi::OsStr,
    os::windows::ffi::OsStrExt
};

pub(crate) fn to_widestring( value : &str ) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(std::iter::once(0)).collect()
}