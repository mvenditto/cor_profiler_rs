#include "cor_helpers.h"

UINT __cdecl cor_sig_compress_token(mdToken token, void* out_buffer)
{
    return CorSigCompressToken(token, out_buffer);
}
