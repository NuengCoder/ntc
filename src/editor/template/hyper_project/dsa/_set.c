#include "_set.h"
#include <stdlib.h>
#include <string.h>

static SetNode* sn_create(const char* key) {
SetNode* n = (SetNode*)malloc(sizeof(SetNode));
if (!n) return NULL;
n->key = strdup(key);
n->left = n->right = NULL;
return n;
}

Set* set_create(int (*cmp)(const char*, const char*)) {
Set* s = (Set*)malloc(sizeof(Set));
if (!s) return NULL;
s->root = NULL;
s->cmp = cmp ? cmp : (int (*)(const char*, const char*))strcmp;
s->size = 0;
return s;
}

static void set_destroy_sub(SetNode* n) {
if (!n) return;
set_destroy_sub(n->left);
set_destroy_sub(n->right);
free(n->key);
free(n);
}

void set_destroy(Set* s) {
if (!s) return;
set_destroy_sub(s->root);
free(s);
}

int set_add(Set* s, const char* key) {
if (!s || !key) return -1;
SetNode* n = sn_create(key);
if (!n) return -1;
if (!s->root) { s->root = n; s->size++; return 0; }
SetNode* cur = s->root;
while (1) {
int c = s->cmp(key, cur->key);
if (c == 0) { free(n->key); free(n); return 0; }
if (c < 0) { if (!cur->left) { cur->left = n; break; } cur = cur->left; }
else { if (!cur->right) { cur->right = n; break; } cur = cur->right; }
}
s->size++;
return 0;
}

int set_contains(Set* s, const char* key) {
if (!s || !key) return 0;
SetNode* cur = s->root;
while (cur) {
int c = s->cmp(key, cur->key);
if (c == 0) return 1;
cur = c < 0 ? cur->left : cur->right;
}
return 0;
}

static SetNode* set_min(SetNode* n) {
while (n && n->left) n = n->left;
return n;
}

static SetNode* set_remove_sub(SetNode* n, const char* key, int (*cmp)(const char*, const char*),
int* removed) {
if (!n) return NULL;
int c = cmp(key, n->key);
if (c < 0) n->left = set_remove_sub(n->left, key, cmp, removed);
else if (c > 0) n->right = set_remove_sub(n->right, key, cmp, removed);
else {
*removed = 1;
free(n->key);
if (!n->left) { SetNode* r = n->right; free(n); return r; }
if (!n->right) { SetNode* l = n->left; free(n); return l; }
SetNode* m = set_min(n->right);
n->key = strdup(m->key);
n->right = set_remove_sub(n->right, m->key, cmp, &(int){0});
}
return n;
}

int set_remove(Set* s, const char* key) {
if (!s || !key || !set_contains(s, key)) return -1;
int removed = 0;
s->root = set_remove_sub(s->root, key, s->cmp, &removed);
if (removed) s->size--;
return 0;
}

int set_size(Set* s) { return s ? s->size : 0; }