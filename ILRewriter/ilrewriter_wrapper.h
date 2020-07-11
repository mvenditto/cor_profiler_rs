#pragma once
#include "ilrewriter.h"


extern "C" {

    typedef void* C_ILRewriter;
    typedef ILInstr* C_ILInstr;

    __declspec(dllexport) C_ILRewriter __cdecl new_il_rewriter(
        ICorProfilerInfo * pICorProfilerInfo, 
        ICorProfilerFunctionControl * pICorProfilerFunctionControl,
        ModuleID moduleID, 
        mdToken tkMethod);

    /* ILREwriter */

    __declspec(dllexport) void __cdecl del_il_rewriter(C_ILRewriter);

    __declspec(dllexport) HRESULT __cdecl import(C_ILRewriter rewriter);

    __declspec(dllexport) C_ILInstr __cdecl new_il_instruction();

    __declspec(dllexport) void __cdecl insert_before(C_ILRewriter rewriter, C_ILInstr pWhere, C_ILInstr pWhat);
    
    __declspec(dllexport) void __cdecl insert_after(C_ILRewriter rewriter, C_ILInstr pWhere, C_ILInstr pWhat);

    __declspec(dllexport) C_ILInstr __cdecl get_il_list(C_ILRewriter rewriter);

    __declspec(dllexport) HRESULT __cdecl emit(C_ILRewriter rewriter);

    /* ILInstr */

    __declspec(dllexport) void __cdecl set_prev(C_ILInstr instr, C_ILInstr prev);

    __declspec(dllexport) void __cdecl set_next(C_ILInstr instr, C_ILInstr next);

    __declspec(dllexport) C_ILInstr __cdecl get_prev(C_ILInstr instr);

    __declspec(dllexport) C_ILInstr __cdecl get_next(C_ILInstr instr);

    __declspec(dllexport) UINT32 __cdecl get_opcode(C_ILInstr instr);

    __declspec(dllexport) void __cdecl set_opcode(C_ILInstr instr, UINT32 opcode);

    __declspec(dllexport) void __cdecl set_arg_8(C_ILInstr instr, INT8 arg);

    __declspec(dllexport) void __cdecl set_arg_16(C_ILInstr instr, INT16 arg);

    __declspec(dllexport) void __cdecl set_arg_32(C_ILInstr instr, INT32 arg);

    __declspec(dllexport) void __cdecl set_arg_64(C_ILInstr instr, INT64 arg);

    __declspec(dllexport) void __cdecl set_arg_instr(C_ILInstr instr, C_ILInstr target);

    __declspec(dllexport) INT8 __cdecl get_arg_8(C_ILInstr instr);

}
