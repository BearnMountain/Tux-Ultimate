#include "vector.h"
#include <stdlib.h>
#include <string.h>

struct Vector {
    void*  data;
    size_t size;
    size_t capacity;
    size_t elem_size;
};

Vector* vector_create(size_t stride) {
    Vector* v = malloc(sizeof(Vector));
    v->size = 0;
    v->capacity = 8;
    v->elem_size = stride;
    v->data = malloc(v->capacity * stride);
    return v;
}

void vector_destroy(Vector* v) {
    free(v->data);
    free(v);
}

static void vector_resize(Vector* v) {
    v->capacity *= 2;
    v->data = realloc(v->data, v->capacity * v->elem_size);
}

void vector_push(Vector* v, const void* e) {
    if (v->size == v->capacity)
        vector_resize(v);

    void* dst = (char*)v->data + v->size * v->elem_size;
    memcpy(dst, e, v->elem_size);
    v->size++;
}

void* vector_get(Vector* v, size_t i) {
    if (i >= v->size)
        return NULL;

    return (char*)v->data + i * v->elem_size;
}

size_t vector_size(const Vector* v) {
    return v->size;
}
