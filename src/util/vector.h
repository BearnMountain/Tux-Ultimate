#ifndef VECTOR_H_
#define VECTOR_H_

#include <stddef.h>

#define VECTOR_RESIZE_FACTOR 2

typedef struct Vector Vector;

Vector* vector_create(size_t stride);
void vector_destroy(Vector* v);
void vector_push(Vector* v, const void* e);
void* vector_get(Vector* v, size_t i);
size_t vector_size(const Vector* v);


#endif
