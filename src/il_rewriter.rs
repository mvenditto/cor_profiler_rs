#[macro_use] 
use crate::types::*;

use crate::interfaces::{
    ICorProfilerInfo,
    ICorProfilerFunctionControl,
    IMetaDataImport
};

use crate::opcodes::*;

use com::{
    ComPtr,
    sys::HRESULT
};

use std::{
    ptr,
    ffi::c_void,
    cell::RefCell
};

pub struct ILRewriter {
    rewriter: C_ILRewriter,
    pointed_instr: C_ILInstr,
    head_instr: C_ILInstr
}

pub(crate) enum ILInstrArgValue {
    Int8(INT8),
    Int16(INT16),
    Int32(INT32),
    Int64(INT64),
    Instr(ILInstr)
}

pub struct ILInstr {
    instr: C_ILInstr,
}

impl<'a> ILInstr {
    pub(crate) fn new() -> ILInstr {
        unsafe { ILInstr {instr: new_il_instruction()} }
    }

    pub(crate) fn get_next(&self) -> Option<ILInstr> {
        unsafe {
            let next_instr = get_next(self.instr);
            if next_instr.is_null() {
                None
            }
            else
            {
                Some(ILInstr { instr: next_instr })
            }
        }
    }

    pub(crate) fn opcode(&self) -> UINT {
        unsafe {
            return get_opcode(self.instr);
        }
    }

    pub(crate) fn set_opcode(&mut self, opcode: UINT) {
        unsafe {
            return set_opcode(self.instr, opcode);
        }
    }

    pub(crate) fn set_arg(&self, arg: ILInstrArgValue) {
        unsafe {
            match arg {
                ILInstrArgValue::Int8(x) => set_arg_8(self.instr, x),
                ILInstrArgValue::Int16(x) => set_arg_16(self.instr, x),
                ILInstrArgValue::Int32(x) => set_arg_32(self.instr, x),
                ILInstrArgValue::Int64(x) => set_arg_64(self.instr, x),
                ILInstrArgValue::Instr(x) => set_arg_instr(self.instr, x.instr)
            }
        }
    }

    pub(crate) fn set_arg_8(&self, arg: INT8) {
        unsafe {
            set_arg_8(self.instr, arg);
        }
    }

    pub(crate) fn set_arg_16(&self, arg: INT16) {
        unsafe {
            set_arg_16(self.instr, arg);
        }
    }

    pub(crate) fn set_arg_32(&self, arg: INT32) {
        unsafe {
            set_arg_32(self.instr, arg);
        }
    }

    pub(crate) fn set_arg_64(&self, arg: INT64) {
        unsafe {
            set_arg_64(self.instr, arg);
        }
    }

    pub(crate) fn set_arg_instr(&self, arg: ILInstr) {
        unsafe {
            set_arg_instr(self.instr, arg.instr);
        }
    }
}

impl PartialEq for ILInstr {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.instr,other.instr)
    }
}

impl Clone for ILInstr {
    fn clone_from(&mut self, source: &Self) {
        self.instr = source.instr;
    }

    fn clone(&self) -> Self {
        ILInstr { instr: self.instr }
    }
}

impl Copy for ILInstr {
    
}

impl Eq for ILInstr {}

impl Iterator for ILRewriter {
    
    type Item = ILInstr;

    fn next(&mut self) -> Option<Self::Item> {
        
        let head = self.head_instr;
        let curr = self.pointed_instr;

        unsafe {
            let next = get_next(curr);
        
            if next.is_null() || next == head {
                None
            }
            else
            {
                self.pointed_instr = next;
                Some(ILInstr { instr: next })
            }
        }
    }

}

impl ILRewriter {

    pub(crate) fn new(
        info:*mut *mut dyn ICorProfilerInfo,
        fc: *mut *mut dyn ICorProfilerFunctionControl,
        module_id: ModuleID,
        method_tk: mdToken
    ) -> ILRewriter {
        unsafe { 
            let r = new_il_rewriter(
                info, 
                fc, 
                module_id, 
                method_tk
            ); 

            let head_instr = get_il_list(r);

            return ILRewriter { 
                rewriter: r, 
                head_instr: head_instr,
                pointed_instr: head_instr
            }
        }
    }
    
    pub(crate) fn import(&self) -> HRESULT {
        if self.rewriter.is_null() {
            return E_FAIL;
        }
        unsafe { return import(self.rewriter); }
    }

    pub(crate) fn export(&self) -> HRESULT {
        assert!(self.rewriter.is_null() == false);
        unsafe { return emit(self.rewriter); }
    }

    pub(crate) fn get_il_list(&self) -> ILInstr {
        unsafe { ILInstr { instr: get_il_list(self.rewriter) } }
    }

    pub(crate) fn insert_before(&self, before: ILInstr, what: ILInstr) {
        unsafe {
            insert_before(self.rewriter, before.instr, what.instr);
        }
    }
}

impl Drop for ILRewriter {
    fn drop(&mut self) {
        unsafe {
            del_il_rewriter(self.rewriter);
        }
    }
}

type C_ILRewriter = *mut c_void;
type C_ILInstr = *mut c_void;

#[link(name = "ILRewriter", kind="static")]
extern {
    #[allow(improper_ctypes)]
    pub fn new_il_rewriter(
        info: *mut *mut dyn ICorProfilerInfo,
        fc: *mut *mut dyn ICorProfilerFunctionControl,
        module_id: ModuleID,
        method_tk: mdToken
    ) -> C_ILRewriter;

    pub fn del_il_rewriter(rewriter: C_ILRewriter);

    pub fn import(rewriter: C_ILRewriter) -> HRESULT;

    pub fn emit(rewriter: C_ILRewriter) -> HRESULT;

    pub fn new_il_instruction() -> C_ILInstr;

    pub fn insert_before(rewriter: C_ILRewriter, il_where: C_ILInstr, il_what: C_ILInstr);

    pub fn insert_after(rewriter: C_ILRewriter, il_where: C_ILInstr, il_what: C_ILInstr);

    pub fn get_il_list(rewriter: C_ILRewriter) -> C_ILInstr;

    pub fn get_opcode(instr: C_ILInstr) -> UINT;

    pub fn set_opcode(instr: C_ILInstr, opcode: UINT);

    pub fn set_arg_64(instr: C_ILInstr, arg: INT64);

    pub fn set_arg_32(instr: C_ILInstr, arg: INT32);

    pub fn set_arg_16(instr: C_ILInstr, arg: INT16);

    pub fn set_arg_8(instr: C_ILInstr, arg: INT8);

    pub fn set_arg_instr(instr: C_ILInstr, arg: C_ILInstr);

    pub fn get_next(instr: C_ILInstr) -> C_ILInstr;
}
