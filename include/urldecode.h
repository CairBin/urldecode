#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C"{
#endif
uintptr_t url_decode_c(const char *input, uintptr_t input_len, char *output, uintptr_t output_len);


#ifdef __cplusplus
}
#endif