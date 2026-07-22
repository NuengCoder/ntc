typedef struct HSEntry {
    char* key;
    int   occupied;
} HSEntry;

typedef struct HashSet {
    HSEntry* buckets;
    int capacity;
    int count;
} HashSet;

HashSet* hashset_create(int initial_capacity);
void     hashset_destroy(HashSet* set);
int      hashset_add(HashSet* set, const char* key);
int      hashset_contains(HashSet* set, const char* key);
int      hashset_remove(HashSet* set, const char* key);
int      hashset_size(HashSet* set);