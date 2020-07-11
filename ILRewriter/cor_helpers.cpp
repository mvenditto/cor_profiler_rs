#include "cor_helpers.h"
#include <iostream>

UINT __cdecl cor_sig_compress_token(mdToken token, void* out_buffer)
{
    return CorSigCompressToken(token, out_buffer);
}

COR_SIGNATURE* __cdecl cor_sig_compress_token_2(mdToken token, ULONG* compressed_tk_length)
{
    auto buff = new COR_SIGNATURE[4];
    *compressed_tk_length = CorSigCompressToken(token, (void*)(&buff[0]));
    std::cout << "cor_sig_compress_token_2 length=" << compressed_tk_length << "\n";
    return &buff[0];
}
