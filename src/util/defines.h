#ifndef DEFINES_H_
#define DEFINES_H_

#include <stdint.h>

typedef uint8_t   u8;
typedef int8_t    i8;
typedef uint32_t  u32;
typedef uint16_t  u16;
typedef uint64_t  u64;
typedef int32_t   i32;
typedef int64_t   i64;
typedef float     f32;
typedef double    f64;
typedef u8        b8;

#define STR(x) #x
#define XSTR(x) STR(x)

#endif
