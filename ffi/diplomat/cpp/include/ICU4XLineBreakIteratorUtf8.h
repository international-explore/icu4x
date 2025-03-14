#ifndef ICU4XLineBreakIteratorUtf8_H
#define ICU4XLineBreakIteratorUtf8_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XLineBreakIteratorUtf8 ICU4XLineBreakIteratorUtf8;

int32_t ICU4XLineBreakIteratorUtf8_next(ICU4XLineBreakIteratorUtf8* self);
void ICU4XLineBreakIteratorUtf8_destroy(ICU4XLineBreakIteratorUtf8* self);

#ifdef __cplusplus
}
#endif
#endif
