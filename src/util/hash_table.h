#ifndef HASH_TABLE_H_
#define HASH_TABLE_H_

#include "src/util/defines.h"
#include <stddef.h>

// Hash table structure: create with HashTable_create, free with HashTable_destroy.
typedef struct HashTable HashTable;

// initializers
HashTable* hash_table_create(void);
void hash_table_destroy(HashTable* table);

void* hash_table_get(HashTable* table, const char* key);
// key is copied to new address
const char* hash_table_set(HashTable* table, const char* key, void* value);

// Return number of items in hash table.
size_t hash_table_length(HashTable* table);

// Hash table iterator:
//		create with HashTable_iterator 
//		iterate with HashTable_next
typedef struct {
    const char* key;
    void* value;

    // Don't use these fields directly.
    HashTable* _table;
    size_t _index;
} HashTableIterator;

HashTableIterator hash_table_iterator(HashTable* table);
b8 hash_table_next(HashTableIterator* it);

#endif // _HashTable_H
