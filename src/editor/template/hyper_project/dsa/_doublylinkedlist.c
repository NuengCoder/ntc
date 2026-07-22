#include "_doublylinkedlist.h"
#include <stdlib.h>

DoublyLinkedList* doublylinkedlist_create(void) {
DoublyLinkedList* list = (DoublyLinkedList*)malloc(sizeof(DoublyLinkedList));
if (!list) return NULL;
list->head = list->tail = NULL;
list->size = 0;
return list;
}

void doublylinkedlist_destroy(DoublyLinkedList* list, void (*free_elem)(void*)) {
if (!list) return;
DLNode* cur = list->head;
while (cur) {
DLNode* next = cur->next;
if (free_elem) free_elem(cur->data);
free(cur);
cur = next;
}
free(list);
}

static DLNode* dlnode_create(void* data) {
DLNode* n = (DLNode*)malloc(sizeof(DLNode));
if (!n) return NULL;
n->data = data;
n->prev = n->next = NULL;
return n;
}

int doublylinkedlist_push_front(DoublyLinkedList* list, void* data) {
if (!list) return -1;
DLNode* n = dlnode_create(data);
if (!n) return -1;
n->next = list->head;
if (list->head) list->head->prev = n;
else list->tail = n;
list->head = n;
list->size++;
return 0;
}

int doublylinkedlist_push_back(DoublyLinkedList* list, void* data) {
if (!list) return -1;
DLNode* n = dlnode_create(data);
if (!n) return -1;
n->prev = list->tail;
if (list->tail) list->tail->next = n;
else list->head = n;
list->tail = n;
list->size++;
return 0;
}

void* doublylinkedlist_pop_front(DoublyLinkedList* list) {
if (!list || !list->head) return NULL;
DLNode* old = list->head;
void* data = old->data;
list->head = old->next;
if (list->head) list->head->prev = NULL;
else list->tail = NULL;
free(old);
list->size--;
return data;
}

void* doublylinkedlist_pop_back(DoublyLinkedList* list) {
if (!list || !list->tail) return NULL;
DLNode* old = list->tail;
void* data = old->data;
list->tail = old->prev;
if (list->tail) list->tail->next = NULL;
else list->head = NULL;
free(old);
list->size--;
return data;
}

void* doublylinkedlist_peek_front(DoublyLinkedList* list) {
return (list && list->head) ? list->head->data : NULL;
}

void* doublylinkedlist_peek_back(DoublyLinkedList* list) {
return (list && list->tail) ? list->tail->data : NULL;
}

int doublylinkedlist_size(DoublyLinkedList* list) { return list ? list->size : 0; }