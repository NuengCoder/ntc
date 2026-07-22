#include "_hashmap.h"
#include <stdlib.h>
#include <string.h>

static unsigned long hash_str(const char* s) {
unsigned long h = 5381;
int c;
while ((c = *s++)) h = ((h << 5) + h) + (unsigned char)c;
return h;
}

HashMap* hashmap_create(int initial_capacity) {
HashMap* m = (HashMap*)malloc(sizeof(HashMap));
if (!m) return NULL;
if (initial_capacity <= 0) initial_capacity = 32;
m->buckets = (HMEntry*)calloc(initial_capacity, sizeof(HMEntry));
if (!m->buckets) { free(m); return NULL; }
m->capacity = initial_capacity;
m->count = 0;
return m;
}

void hashmap_destroy(HashMap* map, void (*free_val)(void*)) {
if (!map) return;
for (int i = 0; i < map->capacity; i++) {
if (map->buckets[i].occupied) {
free(map->buckets[i].key);
if (free_val) free_val(map->buckets[i].value);
}
}
free(map->buckets);
free(map);
}

static int find_slot(HashMap* map, const char* key) {
unsigned long h = hash_str(key);
int idx = (int)(h % (unsigned long)map->capacity);
for (int i = 0; i < map->capacity; i++) {
int si = (idx + i) % map->capacity;
if (!map->buckets[si].occupied) return si;
if (strcmp(map->buckets[si].key, key) == 0) return si;
}
return -1;
}

static int resize(HashMap* map) {
int oc = map->capacity;
HMEntry* ob = map->buckets;
map->capacity = oc * 2;
map->buckets = (HMEntry*)calloc(map->capacity, sizeof(HMEntry));
if (!map->buckets) { map->buckets = ob; map->capacity = oc; return -1; }
map->count = 0;
for (int i = 0; i < oc; i++) {
if (ob[i].occupied) {
hashmap_put(map, ob[i].key, ob[i].value);
free(ob[i].key);
}
}
free(ob);
return 0;
}

int hashmap_put(HashMap* map, const char* key, void* value) {
if (!map || !key) return -1;
if (map->count * 2 >= map->capacity && resize(map) != 0) return -1;
int si = find_slot(map, key);
if (si < 0) return -1;
if (!map->buckets[si].occupied) {
map->buckets[si].key = strdup(key);
if (!map->buckets[si].key) return -1;
map->buckets[si].occupied = 1;
map->count++;
}
map->buckets[si].value = value;
return 0;
}

void* hashmap_get(HashMap* map, const char* key) {
if (!map || !key) return NULL;
unsigned long h = hash_str(key);
int idx = (int)(h % (unsigned long)map->capacity);
for (int i = 0; i < map->capacity; i++) {
int si = (idx + i) % map->capacity;
if (!map->buckets[si].occupied) return NULL;
if (strcmp(map->buckets[si].key, key) == 0)
return map->buckets[si].value;
}
return NULL;
}

int hashmap_has_key(HashMap* map, const char* key) {
return hashmap_get(map, key) != NULL;
}

void* hashmap_remove(HashMap* map, const char* key) {
if (!map || !key) return NULL;
unsigned long h = hash_str(key);
int idx = (int)(h % (unsigned long)map->capacity);
for (int i = 0; i < map->capacity; i++) {
int si = (idx + i) % map->capacity;
if (!map->buckets[si].occupied) return NULL;
if (strcmp(map->buckets[si].key, key) == 0) {
void* v = map->buckets[si].value;
free(map->buckets[si].key);
map->buckets[si].occupied = 0;
map->count--;
return v;
}
}
return NULL;
}

char** hashmap_keys(HashMap* map, int* count) {
if (!map || !count) return NULL;
*count = map->count;
char** ks = (char**)malloc(sizeof(char*) * map->count);
if (!ks) { *count = 0; return NULL; }
int j = 0;
for (int i = 0; i < map->capacity; i++)
if (map->buckets[i].occupied)
ks[j++] = strdup(map->buckets[i].key);
return ks;
}

int hashmap_size(HashMap* map) { return map ? map->count : 0; }