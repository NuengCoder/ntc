#include "_linkedlist.h"
#include <stdlib.h>

LinkedList* linkedlist_create(void) {
LinkedList* list = (LinkedList*)malloc(sizeof(LinkedList));
if (!list) return NULL;
list->head = NULL;
list->size = 0;
return list;
}

void linkedlist_destroy(LinkedList* list, void (*free_elem)(void*)) {
if (!list) return;
LLNode* cur = list->head;
while (cur) {
LLNode* next = cur->next;
if (free_elem) free_elem(cur->data);
free(cur);
cur = next;
}
free(list);
}

static LLNode* node_create(void* data) {
LLNode* n = (LLNode*)malloc(sizeof(LLNode));
if (!n) return NULL;
n->data = data;
n->next = NULL;
return n;
}

int linkedlist_push_front(LinkedList* list, void* data) {
if (!list) return -1;
LLNode* n = node_create(data);
if (!n) return -1;
n->next = list->head;
list->head = n;
list->size++;
return 0;
}

int linkedlist_push_back(LinkedList* list, void* data) {
if (!list) return -1;
LLNode* n = node_create(data);
if (!n) return -1;
if (!list->head) {
list->head = n;
} else {
LLNode* cur = list->head;
while (cur->next) cur = cur->next;
cur->next = n;
}
list->size++;
return 0;
}

void* linkedlist_pop_front(LinkedList* list) {
if (!list || !list->head) return NULL;
LLNode* old = list->head;
void* data = old->data;
list->head = old->next;
free(old);
list->size--;
return data;
}

void* linkedlist_pop_back(LinkedList* list) {
if (!list || !list->head) return NULL;
if (!list->head->next) return linkedlist_pop_front(list);
LLNode* cur = list->head;
while (cur->next->next) cur = cur->next;
void* data = cur->next->data;
free(cur->next);
cur->next = NULL;
list->size--;
return data;
}

void* linkedlist_peek_front(LinkedList* list) {
return (list && list->head) ? list->head->data : NULL;
}

void* linkedlist_peek_back(LinkedList* list) {
if (!list || !list->head) return NULL;
LLNode* cur = list->head;
while (cur->next) cur = cur->next;
return cur->data;
}

LLNode* linkedlist_find(LinkedList* list, void* data,
int (*cmp)(const void*, const void*)) {
if (!list || !cmp) return NULL;
LLNode* cur = list->head;
while (cur) {
if (cmp(cur->data, data) == 0) return cur;
cur = cur->next;
}
return NULL;
}

int linkedlist_remove(LinkedList* list, void* data,
int (*cmp)(const void*, const void*),
void (*free_elem)(void*)) {
if (!list || !cmp || !list->head) return -1;
LLNode* cur = list->head;
LLNode* prev = NULL;
while (cur) {
if (cmp(cur->data, data) == 0) {
if (prev) prev->next = cur->next;
else list->head = cur->next;
if (free_elem) free_elem(cur->data);
free(cur);
list->size--;
return 0;
}
prev = cur;
cur = cur->next;
}
return -1;
}

int linkedlist_size(LinkedList* list) { return list ? list->size : 0; }