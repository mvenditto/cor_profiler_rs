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

pub struct ILInstr {
    instr: C_ILInstr
}

impl<'a> ILInstr {


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
}

impl PartialEq for ILInstr {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.instr,other.instr)
    }
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

    pub fn new_il_instruction(rewriter: C_ILRewriter) -> C_ILInstr;

    pub fn insert_before(il_where: C_ILInstr, il_what: C_ILInstr);

    pub fn insert_after(il_where: C_ILInstr, il_what: C_ILInstr);

    pub fn get_il_list(rewriter: C_ILRewriter) -> C_ILInstr;

    pub fn get_opcode(instr: C_ILInstr) -> UINT;

    pub fn set_opcode(instr: C_ILInstr, opcode: UINT);

    pub fn get_next(instr: C_ILInstr) -> C_ILInstr;
}
