#include "ilrewriter_wrapper.h"
#include <iostream>

using namespace std; 

C_ILRewriter new_il_rewriter(ICorProfilerInfo* pICorProfilerInfo, ICorProfilerFunctionControl* pICorProfilerFunctionControl, ModuleID moduleID, mdToken tkMethod)
{
    auto rewriter = new ILRewriter(pICorProfilerInfo, pICorProfilerFunctionControl, moduleID, tkMethod);
    HRESULT hr = rewriter->Initialize();
    return reinterpret_cast<void*>(rewriter);
}

void del_il_rewriter(C_ILRewriter rewriter) {
    delete reinterpret_cast<ILRewriter*>(rewriter);
}

HRESULT import(C_ILRewriter rewriter)
{
    return reinterpret_cast<ILRewriter*>(rewriter)->Import();
}

C_ILInstr new_il_instruction()
{
    return new ILInstr();
}

void insert_before(C_ILRewriter rewriter, C_ILInstr pWhere, C_ILInstr pWhat)
{
    auto _rewriter = reinterpret_cast<ILRewriter*>(rewriter);
    _rewriter->IncInstrCount();
    _rewriter->InsertBefore(pWhere, pWhat);
}

void insert_after(C_ILRewriter rewriter, C_ILInstr pWhere, C_ILInstr pWhat)
{
    auto _rewriter = reinterpret_cast<ILRewriter*>(rewriter);
    _rewriter->IncInstrCount();
    _rewriter->InsertAfter(pWhere, pWhat);
}

C_ILInstr get_il_list(C_ILRewriter rewriter)
{
    return reinterpret_cast<ILRewriter*>(rewriter)->GetILList();
}

HRESULT emit(C_ILRewriter rewriter)
{
    return reinterpret_cast<ILRewriter*>(rewriter)->Export();
}

void __cdecl set_prev(C_ILInstr instr, C_ILInstr prev)
{
    instr->m_pPrev = prev;
}

void __cdecl set_next(C_ILInstr instr, C_ILInstr next)
{
    instr->m_pNext = next;
}

C_ILInstr __cdecl get_prev(C_ILInstr instr)
{
    return instr->m_pPrev;
}

C_ILInstr __cdecl get_next(C_ILInstr instr)
{
    return instr->m_pNext;
}

UINT32 __cdecl get_opcode(C_ILInstr instr)
{
    return instr->m_opcode;
}

void __cdecl set_opcode(C_ILInstr instr, UINT32 opcode)
{
    instr->m_opcode = opcode;
}

void __cdecl set_arg_8(C_ILInstr instr, INT8 arg)
{
    instr->m_Arg8 = arg;
}

void __cdecl set_arg_16(C_ILInstr instr, INT16 arg)
{
    instr->m_Arg16 = arg;
}

void __cdecl set_arg_32(C_ILInstr instr, INT32 arg)
{
    instr->m_Arg32 = arg;
}

void __cdecl set_arg_64(C_ILInstr instr, INT64 arg)
{
    instr->m_Arg64 = arg;
}

void __cdecl set_arg_instr(C_ILInstr instr, C_ILInstr target)
{
    instr->m_pTarget = target;
}

INT8 __cdecl get_arg_8(C_ILInstr instr) 
{
    return instr->m_Arg8;
}

INT32 __cdecl get_arg_32(C_ILInstr instr) 
{
    return instr->m_Arg32;
}