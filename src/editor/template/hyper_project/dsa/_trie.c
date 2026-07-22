#include "_trie.h"
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

static TrieNode* tn_create(void) {
TrieNode* n = (TrieNode*)calloc(1, sizeof(TrieNode));
if (!n) return NULL;
n->is_end = 0;
n->count = 0;
for (int i = 0; i < TRIE_ALPHABET; i++) n->children[i] = NULL;
return n;
}

Trie* trie_create(void) {
Trie* t = (Trie*)malloc(sizeof(Trie));
if (!t) return NULL;
t->root = tn_create();
if (!t->root) { free(t); return NULL; }
t->size = 0;
return t;
}

static void tn_destroy(TrieNode* n) {
if (!n) return;
for (int i = 0; i < TRIE_ALPHABET; i++)
tn_destroy(n->children[i]);
free(n);
}

void trie_destroy(Trie* t) {
if (!t) return;
tn_destroy(t->root);
free(t);
}

int trie_insert(Trie* t, const char* word) {
if (!t || !word) return -1;
TrieNode* cur = t->root;
for (int i = 0; word[i]; i++) {
int idx = tolower((unsigned char)word[i]) - 'a';
if (idx < 0 || idx >= TRIE_ALPHABET) continue;
if (!cur->children[idx]) {
cur->children[idx] = tn_create();
if (!cur->children[idx]) return -1;
}
cur = cur->children[idx];
cur->count++;
}
if (!cur->is_end) { cur->is_end = 1; t->size++; }
return 0;
}

int trie_search(Trie* t, const char* word) {
if (!t || !word) return 0;
TrieNode* cur = t->root;
for (int i = 0; word[i]; i++) {
int idx = tolower((unsigned char)word[i]) - 'a';
if (idx < 0 || idx >= TRIE_ALPHABET || !cur->children[idx]) return 0;
cur = cur->children[idx];
}
return cur->is_end;
}

int trie_starts_with(Trie* t, const char* prefix) {
if (!t || !prefix) return 0;
TrieNode* cur = t->root;
for (int i = 0; prefix[i]; i++) {
int idx = tolower((unsigned char)prefix[i]) - 'a';
if (idx < 0 || idx >= TRIE_ALPHABET || !cur->children[idx]) return 0;
cur = cur->children[idx];
}
return 1;
}

static int has_children(TrieNode* n) {
for (int i = 0; i < TRIE_ALPHABET; i++)
if (n->children[i]) return 1;
return 0;
}

static int tn_delete(TrieNode* n, const char* word, int depth) {
if (!n) return 0;
if (word[depth] == '\0') {
if (n->is_end) { n->is_end = 0; return 1; }
return 0;
}
int idx = tolower((unsigned char)word[depth]) - 'a';
if (idx < 0 || idx >= TRIE_ALPHABET) return 0;
if (tn_delete(n->children[idx], word, depth + 1)) {
n->children[idx]->count--;
if (n->children[idx]->count == 0 && !n->children[idx]->is_end && !has_children(n->children[idx])) {
tn_destroy(n->children[idx]);
n->children[idx] = NULL;
}
return 1;
}
return 0;
}

int trie_delete(Trie* t, const char* word) {
if (!t || !word || !trie_search(t, word)) return -1;
t->size--;
tn_delete(t->root, word, 0);
return 0;
}

int trie_size(Trie* t) { return t ? t->size : 0; }