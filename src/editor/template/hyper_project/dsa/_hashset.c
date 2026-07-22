#include "_hashset.h"
#include <stdlib.h>
#include <string.h>

static unsigned long hs_hash(const char* s) {
unsigned long h = 5381;
int c;
while ((c = *s++)) h = ((h << 5) + h) + (unsigned char)c;
return h;
}

HashSet* hashset_create(int initial_capacity) {
HashSet* s = (HashSet*)malloc(sizeof(HashSet));
if (!s) return NULL;
if (initial_capacity <= 0) initial_capacity = 32;
s->buckets = (HSEntry*)calloc(initial_capacity, sizeof(HSEntry));
if (!s->buckets) { free(s); return NULL; }
s->capacity = initial_capacity;
s->count = 0;
return s;
}

void hashset_destroy(HashSet* set) {
if (!set) return;
for (int i = 0; i < set->capacity; i++)
if (set->buckets[i].occupied) free(set->buckets[i].key);
free(set->buckets);
free(set);
}

static int hs_find_slot(HashSet* set, const char* key) {
unsigned long h = hs_hash(key);
int idx = (int)(h % (unsigned long)set->capacity);
for (int i = 0; i < set->capacity; i++) {
int si = (idx + i) % set->capacity;
if (!set->buckets[si].occupied) return si;
if (strcmp(set->buckets[si].key, key) == 0) return si;
}
return -1;
}

static int hs_resize(HashSet* set) {
int oc = set->capacity;
HSEntry* ob = set->buckets;
set->capacity = oc * 2;
set->buckets = (HSEntry*)calloc(set->capacity, sizeof(HSEntry));
if (!set->buckets) { set->buckets = ob; set->capacity = oc; return -1; }
int old_count = set->count;
set->count = 0;
for (int i = 0; i < oc; i++) {
if (ob[i].occupied) {
hashset_add(set, ob[i].key);
free(ob[i].key);
}
}
free(ob);
return old_count != set->count ? -1 : 0;
}

int hashset_add(HashSet* set, const char* key) {
if (!set || !key) return -1;
if (set->count * 2 >= set->capacity && hs_resize(set) != 0) return -1;
int si = hs_find_slot(set, key);
if (si < 0) return -1;
if (!set->buckets[si].occupied) {
set->buckets[si].key = strdup(key);
if (!set->buckets[si].key) return -1;
set->buckets[si].occupied = 1;
set->count++;
}
return 0;
}

int hashset_contains(HashSet* set, const char* key) {
if (!set || !key) return 0;
unsigned long h = hs_hash(key);
int idx = (int)(h % (unsigned long)set->capacity);
for (int i = 0; i < set->capacity; i++) {
int si = (idx + i) % set->capacity;
if (!set->buckets[si].occupied) return 0;
if (strcmp(set->buckets[si].key, key) == 0) return 1;
}
return 0;
}

int hashset_remove(HashSet* set, const char* key) {
if (!set || !key) return -1;
unsigned long h = hs_hash(key);
int idx = (int)(h % (unsigned long)set->capacity);
for (int i = 0; i < set->capacity; i++) {
int si = (idx + i) % set->capacity;
if (!set->buckets[si].occupied) return -1;
if (strcmp(set->buckets[si].key, key) == 0) {
free(set->buckets[si].key);
set->buckets[si].occupied = 0;
set->count--;
return 0;
}
}
return -1;
}

int hashset_size(HashSet* set) { return set ? set->count : 0; }