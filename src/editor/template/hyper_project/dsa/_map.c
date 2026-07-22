#include "_map.h"
#include <stdlib.h>
#include <string.h>

static MapNode* mn_create(const char* key, void* value) {
MapNode* n = (MapNode*)malloc(sizeof(MapNode));
if (!n) return NULL;
n->key = strdup(key);
n->value = value;
n->left = n->right = NULL;
return n;
}

Map* map_create(int (*cmp)(const char*, const char*)) {
Map* m = (Map*)malloc(sizeof(Map));
if (!m) return NULL;
m->root = NULL;
m->cmp = cmp ? cmp : (int (*)(const char*, const char*))strcmp;
m->size = 0;
return m;
}

static void map_destroy_sub(MapNode* n, void (*fv)(void*)) {
if (!n) return;
map_destroy_sub(n->left, fv);
map_destroy_sub(n->right, fv);
free(n->key);
if (fv) fv(n->value);
free(n);
}

void map_destroy(Map* m, void (*free_val)(void*)) {
if (!m) return;
map_destroy_sub(m->root, free_val);
free(m);
}

int map_put(Map* m, const char* key, void* value) {
if (!m || !key) return -1;
MapNode* n = mn_create(key, value);
if (!n) return -1;
if (!m->root) { m->root = n; m->size++; return 0; }
MapNode* cur = m->root;
while (1) {
int c = m->cmp(key, cur->key);
if (c == 0) { free(n->key); free(n); cur->value = value; return 0; }
if (c < 0) { if (!cur->left) { cur->left = n; break; } cur = cur->left; }
else { if (!cur->right) { cur->right = n; break; } cur = cur->right; }
}
m->size++;
return 0;
}

void* map_get(Map* m, const char* key) {
if (!m || !key) return NULL;
MapNode* cur = m->root;
while (cur) {
int c = m->cmp(key, cur->key);
if (c == 0) return cur->value;
cur = c < 0 ? cur->left : cur->right;
}
return NULL;
}

static MapNode* map_min(MapNode* n) {
while (n && n->left) n = n->left;
return n;
}

static MapNode* map_remove_sub(MapNode* n, const char* key, int (*cmp)(const char*, const char*),
void (*fv)(void*), int* removed) {
if (!n) return NULL;
int c = cmp(key, n->key);
if (c < 0) n->left = map_remove_sub(n->left, key, cmp, fv, removed);
else if (c > 0) n->right = map_remove_sub(n->right, key, cmp, fv, removed);
else {
*removed = 1;
free(n->key);
if (fv) fv(n->value);
if (!n->left) { MapNode* r = n->right; free(n); return r; }
if (!n->right) { MapNode* l = n->left; free(n); return l; }
MapNode* m = map_min(n->right);
n->key = strdup(m->key);
n->value = m->value;
n->right = map_remove_sub(n->right, m->key, cmp, NULL, &(int){0});
}
return n;
}

void* map_remove(Map* m, const char* key) {
if (!m || !key) return NULL;
void* old = map_get(m, key);
if (!old) return NULL;
int removed = 0;
m->root = map_remove_sub(m->root, key, m->cmp, NULL, &removed);
if (removed) m->size--;
return old;
}

int map_has_key(Map* m, const char* key) { return map_get(m, key) != NULL; }
int map_size(Map* m) { return m ? m->size : 0; }