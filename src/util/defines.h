#ifndef DEFINES_H_
#define DEFINES_H_

#include <stdint.h>

typedef uint8_t   u8;
typedef uint32_t  u32;
typedef uint64_t  u64;
typedef int32_t   i32;
typedef int64_t   i64;
typedef float     f32;
typedef double    f64;
typedef u8        b8;

#define STR(x) #x
#define XSTR(x) STR(x)

#ifndef true
#define true 1
#endif

#ifndef false
#define false 0
#endif

#endif
