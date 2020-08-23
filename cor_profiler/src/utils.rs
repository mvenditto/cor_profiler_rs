use std::{
    ffi::OsStr,
    os::windows::ffi::OsStrExt
};

#[cfg(windows)] 
pub(crate) fn to_widestring( value : &str ) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(std::iter::once(0)).collect()
}

/*
#[cfg(windows)]     
fn from_wide_string(s: &[WCHAR]) -> String {         
    use std::ffi::OsString;         
   use std::os::windows::ffi::OsStringExt;         
   let slice = s.split(|&v| v == 0).next().unwrap();         
   OsString::from_wide(slice).to_string_lossy().into()     
}
*/