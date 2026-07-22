#include "_rope.h"
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

static RopeNode* rn_create(const char* s) {
RopeNode* n = (RopeNode*)malloc(sizeof(RopeNode));
if (!n) return NULL;
n->str = s ? strdup(s) : strdup("");
n->len = s ? (int)strlen(s) : 0;
n->left = n->right = NULL;
return n;
}

Rope* rope_create(const char* initial) {
Rope* r = (Rope*)malloc(sizeof(Rope));
if (!r) return NULL;
r->root = rn_create(initial ? initial : "");
r->total_len = r->root ? r->root->len : 0;
return r;
}

static void rn_destroy(RopeNode* n) {
if (!n) return;
rn_destroy(n->left);
rn_destroy(n->right);
free(n->str);
free(n);
}

void rope_destroy(Rope* r) {
if (!r) return;
rn_destroy(r->root);
free(r);
}

static char* rope_flatten(RopeNode* n) {
if (!n) return strdup("");
if (!n->left && !n->right) return strdup(n->str);
char* l = rope_flatten(n->left);
char* r = rope_flatten(n->right);
char* res = (char*)malloc(strlen(l) + strlen(r) + 1);
if (res) { sprintf(res, "%s%s", l, r); }
free(l); free(r);
return res;
}

int rope_insert(Rope* r, int index, const char* s) {
if (!r || !s || index < 0 || index > r->total_len) return -1;
char* flat = rope_flatten(r->root);
if (!flat) return -1;
int flen = (int)strlen(flat);
char* before = (char*)malloc(index + 1);
char* after  = (char*)malloc(flen - index + 1);
if (!before || !after) { free(flat); free(before); free(after); return -1; }
strncpy(before, flat, index); before[index] = '\0';
strcpy(after, flat + index);
char* new_flat = (char*)malloc(strlen(before) + strlen(s) + strlen(after) + 1);
if (!new_flat) { free(flat); free(before); free(after); return -1; }
strcpy(new_flat, before);
strcat(new_flat, s);
strcat(new_flat, after);
rn_destroy(r->root);
r->root = rn_create(new_flat);
r->total_len = (int)strlen(new_flat);
free(new_flat); free(flat); free(before); free(after);
return 0;
}

int rope_delete(Rope* r, int start, int end) {
if (!r || start < 0 || end > r->total_len || start >= end) return -1;
char* flat = rope_flatten(r->root);
if (!flat) return -1;
int flen = (int)strlen(flat);
char* before = (char*)malloc(start + 1);
if (!before) { free(flat); return -1; }
strncpy(before, flat, start); before[start] = '\0';
char* new_flat = (char*)malloc(strlen(before) + (flen - end) + 1);
if (!new_flat) { free(flat); free(before); return -1; }
strcpy(new_flat, before);
strcat(new_flat, flat + end);
rn_destroy(r->root);
r->root = rn_create(new_flat);
r->total_len = (int)strlen(new_flat);
free(new_flat); free(flat); free(before);
return 0;
}

char* rope_substr(Rope* r, int start, int end) {
if (!r || start < 0 || end > r->total_len || start >= end) return NULL;
char* flat = rope_flatten(r->root);
if (!flat) return NULL;
char* sub = (char*)malloc(end - start + 1);
if (!sub) { free(flat); return NULL; }
strncpy(sub, flat + start, end - start);
sub[end - start] = '\0';
free(flat);
return sub;
}

char rope_char_at(Rope* r, int index) {
if (!r || index < 0 || index >= r->total_len) return '\0';
char* flat = rope_flatten(r->root);
if (!flat) return '\0';
char c = flat[index];
free(flat);
return c;
}

int rope_len(Rope* r) { return r ? r->total_len : 0; }