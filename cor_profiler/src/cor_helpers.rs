#![allow(non_camel_case_types)]
#![allow(dead_code)]
#[macro_use]

use std::{slice, ffi::c_void, ptr};
use num_derive::FromPrimitive;
use num_derive::ToPrimitive; 

use crate::types::{
    mdToken,
    mdTypeDef,
    mdTypeRef,
    COR_SIGNATURE,
    ULONG,
    ULONG32
};

use com::{
    ComPtr,
    sys::HRESULT
};

pub(crate) struct CorSignature {
    arguments: Vec<COR_SIGNATURE>
}

impl CorSignature {

    pub fn new() -> CorSignature {
        CorSignature {
            arguments: vec![
                CorCallingConvention::IMAGE_CEE_CS_CALLCONV_DEFAULT as COR_SIGNATURE,   // callconv
                0,  // arguments number
                CorElementType::ELEMENT_TYPE_VOID as COR_SIGNATURE // return type
            ]
        }
    }

    pub fn call_conv(mut self, call_conv: CorCallingConvention) -> CorSignature {
        self.arguments[0] = call_conv as COR_SIGNATURE;
        self
    }

    pub fn ret(mut self, ret: CorElementType) -> CorSignature {
        self.arguments[2] = ret as COR_SIGNATURE;
        self
    }

    pub fn arg(mut self, arg: CorElementType) -> CorSignature {
        self.arguments[1] += 1;
        self.arguments.push(arg as COR_SIGNATURE);
        self
    }

    fn add_complex_arg(&mut self, token: mdToken, arg_type: CorElementType) {
        self.arguments[1] += 1;
        self.arguments.push(arg_type as COR_SIGNATURE);
        unsafe {
            let mut tk_len: ULONG = 0;
            let x = cor_sig_compress_token_2(token, &mut tk_len);
            let ct: &[COR_SIGNATURE] = slice::from_raw_parts(x, 4);
            for i in 0..tk_len {
                info!("push byte {:x} to signature", ct[i as usize]);
                self.arguments.push(ct[i as usize]);
            }
            info!("compressed_token: {:?} - {}", &ct[0 .. tk_len as usize], tk_len);
        }
    }

    pub fn arg_class(mut self, token: mdTypeDef) -> CorSignature {
        self.add_complex_arg(token, CorElementType::ELEMENT_TYPE_CLASS);
        self
    }

    pub fn arg_value_type(mut self, token: mdTypeDef) -> CorSignature {
        self.add_complex_arg(token, CorElementType::ELEMENT_TYPE_VALUETYPE);
        self
    }

    pub fn pack<'a>(self) -> Vec<COR_SIGNATURE> {
        info!("produced cor_sig: {:?}", self.arguments);
        self.arguments
    }
}


#[derive(FromPrimitive)]
#[derive(ToPrimitive)]
#[repr(u8)]
pub(crate) enum CorCallingConvention {
    IMAGE_CEE_CS_CALLCONV_DEFAULT       = 0x0,  
    IMAGE_CEE_CS_CALLCONV_VARARG        = 0x5,  
    IMAGE_CEE_CS_CALLCONV_FIELD         = 0x6,  
    IMAGE_CEE_CS_CALLCONV_LOCAL_SIG     = 0x7,  
    IMAGE_CEE_CS_CALLCONV_PROPERTY      = 0x8,  
    IMAGE_CEE_CS_CALLCONV_UNMGD         = 0x9,  
    IMAGE_CEE_CS_CALLCONV_GENERICINST   = 0xa,  
    IMAGE_CEE_CS_CALLCONV_NATIVEVARARG  = 0xb,  
    IMAGE_CEE_CS_CALLCONV_MAX           = 0xc,  
    IMAGE_CEE_CS_CALLCONV_MASK          = 0x0f,  
    IMAGE_CEE_CS_CALLCONV_HASTHIS       = 0x20,  
    IMAGE_CEE_CS_CALLCONV_EXPLICITTHIS  = 0x40,  
    IMAGE_CEE_CS_CALLCONV_GENERIC       = 0x10  
}

#[derive(FromPrimitive)]
#[derive(ToPrimitive)]
#[repr(u8)]
pub(crate) enum CorElementType {
    ELEMENT_TYPE_END            = 0x0,
    ELEMENT_TYPE_VOID           = 0x1,
    ELEMENT_TYPE_BOOLEAN        = 0x2,
    ELEMENT_TYPE_CHAR           = 0x3,
    ELEMENT_TYPE_I1             = 0x4,
    ELEMENT_TYPE_U1             = 0x5,
    ELEMENT_TYPE_I2             = 0x6,
    ELEMENT_TYPE_U2             = 0x7,
    ELEMENT_TYPE_I4             = 0x8,
    ELEMENT_TYPE_U4             = 0x9,
    ELEMENT_TYPE_I8             = 0xa,
    ELEMENT_TYPE_U8             = 0xb,
    ELEMENT_TYPE_R4             = 0xc,
    ELEMENT_TYPE_R8             = 0xd,
    ELEMENT_TYPE_STRING         = 0xe,

    ELEMENT_TYPE_PTR            = 0xf,
    ELEMENT_TYPE_BYREF          = 0x10,

    ELEMENT_TYPE_VALUETYPE      = 0x11,
    ELEMENT_TYPE_CLASS          = 0x12,
    ELEMENT_TYPE_VAR            = 0x13,
    ELEMENT_TYPE_ARRAY          = 0x14,
    ELEMENT_TYPE_GENERICINST    = 0x15,
    ELEMENT_TYPE_TYPEDBYREF     = 0x16,

    ELEMENT_TYPE_I              = 0x18,
    ELEMENT_TYPE_U              = 0x19,
    ELEMENT_TYPE_FNPTR          = 0x1B,
    ELEMENT_TYPE_OBJECT         = 0x1C,
    ELEMENT_TYPE_SZARRAY        = 0x1D,
    ELEMENT_TYPE_MVAR           = 0x1e,

    ELEMENT_TYPE_CMOD_REQD      = 0x1F,
    ELEMENT_TYPE_CMOD_OPT       = 0x20,

    ELEMENT_TYPE_INTERNAL       = 0x21,
    ELEMENT_TYPE_MAX            = 0x22,

    ELEMENT_TYPE_MODIFIER       = 0x40,
    ELEMENT_TYPE_SENTINEL       = 0x01 | 0x40,
    ELEMENT_TYPE_PINNED         = 0x05 | 0x40
}

#[derive(FromPrimitive)]
#[derive(ToPrimitive)]
#[repr(u32)]
pub(crate) enum CorTokenType
{
    mdtModule               = 0x00000000,       //
    mdtTypeRef              = 0x01000000,       //
    mdtTypeDef              = 0x02000000,       //
    mdtFieldDef             = 0x04000000,       //
    mdtMethodDef            = 0x06000000,       //
    mdtParamDef             = 0x08000000,       //
    mdtInterfaceImpl        = 0x09000000,       //
    mdtMemberRef            = 0x0a000000,       //
    mdtCustomAttribute      = 0x0c000000,       //
    mdtPermission           = 0x0e000000,       //
    mdtSignature            = 0x11000000,       //
    mdtEvent                = 0x14000000,       //
    mdtProperty             = 0x17000000,       //
    mdtMethodImpl           = 0x19000000,       //
    mdtModuleRef            = 0x1a000000,       //
    mdtTypeSpec             = 0x1b000000,       //
    mdtAssembly             = 0x20000000,       //
    mdtAssemblyRef          = 0x23000000,       //
    mdtFile                 = 0x26000000,       //
    mdtExportedType         = 0x27000000,       //
    mdtManifestResource     = 0x28000000,       //
    mdtGenericParam         = 0x2a000000,       //
    mdtMethodSpec           = 0x2b000000,       //
    mdtGenericParamConstraint = 0x2c000000,
    mdtString               = 0x70000000,       //
    mdtName                 = 0x71000000,       //
    mdtBaseType             = 0x72000000,       // Leave this on the high end value. This does not correspond to metadata table
}

type C_ICLRMetaHost = *mut c_void;
type C_ICLRRuntimeInfo = *mut c_void;

pub(crate) struct ICLRMetaHost {
    native: C_ICLRMetaHost
}

pub(crate) struct ICLRRuntimeInfo {
    native: C_ICLRRuntimeInfo
}

impl ICLRMetaHost {
    pub fn create() -> Result<Self, HRESULT> {
        unsafe {
            let mut hr: HRESULT = 0;
            let meta_host_ptr = clr_create_meta_host(&mut hr);
            if hr < 0 {
                return Err(hr);
            }
            Ok(ICLRMetaHost { native: meta_host_ptr })
        }
    }

    pub fn get_installed_runtimes(self) -> Result<Vec<ICLRRuntimeInfo>, HRESULT> {
        unsafe {
            let mut runtimes_len: ULONG = 0;
            let mut runtimes: C_ICLRRuntimeInfo = ptr::null_mut();
            let hr = clr_get_installed_runtimes(
                self.native, runtimes, &mut runtimes_len);
            if hr < 0 { return Err(hr); }
            let ct: &[C_ICLRRuntimeInfo] = slice::from_raw_parts(
                &runtimes, runtimes_len as usize);
            let mut ret = 
                Vec::with_capacity(runtimes_len as usize);
            for x in ct {
                ret.push(ICLRRuntimeInfo { native: *x});
            }
            info!("num runtimes={}", ret.len());
            Ok(ret)
        }
    }
 }

#[link(name = "ILRewriter", kind="static")]
extern {
    pub fn clr_create_meta_host(hr: *mut HRESULT) -> C_ICLRMetaHost;

    pub fn clr_get_installed_runtimes(metahost: C_ICLRMetaHost, runtimes: C_ICLRRuntimeInfo, runtimes_len: *mut ULONG) -> HRESULT;

    pub fn cor_sig_compress_token(token: mdToken, out_buff: *mut c_void) -> ULONG;

    pub fn cor_sig_compress_token_2(token: mdToken, compressed_tk_len: *mut ULONG) -> *mut COR_SIGNATURE;

    pub fn cor_sig_uncompress_token_2(sig: *const COR_SIGNATURE, uncompressed_tk_len: *mut ULONG) -> mdToken;
}