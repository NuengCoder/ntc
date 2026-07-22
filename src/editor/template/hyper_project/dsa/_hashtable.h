#include "_hashmap.h"
typedef HashMap HashTable;
#define hashtable_create(cap)        hashmap_create(cap)
#define hashtable_destroy(m, fv)     hashmap_destroy(m, fv)
#define hashtable_put(m, k, v)       hashmap_put(m, k, v)
#define hashtable_get(m, k)          hashmap_get(m, k)
#define hashtable_has_key(m, k)      hashmap_has_key(m, k)
#define hashtable_remove(m, k)       hashmap_remove(m, k)
#define hashtable_keys(m, c)         hashmap_keys(m, c)
#define hashtable_size(m)            hashmap_size(m)