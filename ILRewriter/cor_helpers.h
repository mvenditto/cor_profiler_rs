#pragma once
#include <corhlpr.h>


extern "C" {

	__declspec(dllexport) UINT __cdecl cor_sig_compress_token(
		mdToken token,
		void* out_buffer
	);

	__declspec(dllexport) COR_SIGNATURE* cor_sig_compress_token_2(mdToken token, ULONG* compressed_tk_length);

	__declspec(dllexport) mdToken cor_sig_uncompress_token_2(COR_SIGNATURE* sig, ULONG* uncompressed_tk_length);

}