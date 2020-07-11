#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::{slice, ffi::c_void};
use num_derive::FromPrimitive;
use num_derive::ToPrimitive; 

use crate::types::{
    mdToken,
    mdTypeDef,
    mdTypeRef,
    COR_SIGNATURE,
    ULONG
};

pub(crate) struct CorSignature {
    arguments: Vec<COR_SIGNATURE>
}

impl CorSignature {

    pub fn new() -> CorSignature {
        CorSignature {
            arguments: vec![
                CorCallingConvention::IMAGE_CEE_CS_CALLCONV_DEFAULT as COR_SIGNATURE,
                0,
                CorElementType::ELEMENT_TYPE_VOID as COR_SIGNATURE
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
        // let compressed_tok_pointer = self.arguments.as_mut_ptr() as *mut c_void;
        unsafe {
            let mut tk_len: ULONG = 0;
            let x = cor_sig_compress_token_2(token, &mut tk_len);
            let ct: &[COR_SIGNATURE] = slice::from_raw_parts(x, 4);
            for i in 0..tk_len {
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

#[link(name = "ILRewriter", kind="static")]
extern {
    pub fn cor_sig_compress_token(token: mdToken, out_buff: *mut c_void) -> ULONG;

    pub fn cor_sig_compress_token_2(token: mdToken, compressed_tk_len: *mut ULONG) -> *mut COR_SIGNATURE;
}