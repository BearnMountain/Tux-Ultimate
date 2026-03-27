#ifndef DYNAMIC_ARRAY_H_
#define DYNAMIC_ARRAY_H_

#include <stddef.h>

#define DA_DEFAULT_SIZE 1
#define DA_RESIZE_FACTOR 2

typedef struct {
	void* buffer;
	size_t size;
	size_t capacity;
	size_t stride;
} Vector;

Vector* da_create(size_t length, size_t stride); // da_create(1, sizeof(char*)) for strings
void da_destroy(Vector* vec);

#endif
