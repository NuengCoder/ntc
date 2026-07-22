typedef struct HMEntry {
    char* key;
    void* value;
    int  occupied;
} HMEntry;

typedef struct HashMap {
    HMEntry* buckets;
    int capacity;
    int count;
} HashMap;

HashMap* hashmap_create(int initial_capacity);
void     hashmap_destroy(HashMap* map, void (*free_val)(void*));
int      hashmap_put(HashMap* map, const char* key, void* value);
void*    hashmap_get(HashMap* map, const char* key);
int      hashmap_has_key(HashMap* map, const char* key);
void*    hashmap_remove(HashMap* map, const char* key);
char**   hashmap_keys(HashMap* map, int* count);
int      hashmap_size(HashMap* map);