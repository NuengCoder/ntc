#include "_circularlinkedlist.h"
#include <stdlib.h>

CircularLinkedList* circularlinkedlist_create(void) {
CircularLinkedList* list = (CircularLinkedList*)malloc(sizeof(CircularLinkedList));
if (!list) return NULL;
list->tail = NULL;
list->size = 0;
return list;
}

void circularlinkedlist_destroy(CircularLinkedList* list, void (*free_elem)(void*)) {
if (!list || !list->tail) { free(list); return; }
CLNode* start = list->tail->next;
CLNode* cur = start;
do {
CLNode* next = cur->next;
if (free_elem) free_elem(cur->data);
free(cur);
cur = next;
} while (cur != start);
free(list);
}

static CLNode* clnode_create(void* data) {
CLNode* n = (CLNode*)malloc(sizeof(CLNode));
if (!n) return NULL;
n->data = data;
n->next = n;
return n;
}

int circularlinkedlist_push_front(CircularLinkedList* list, void* data) {
if (!list) return -1;
CLNode* n = clnode_create(data);
if (!n) return -1;
if (!list->tail) {
list->tail = n;
} else {
n->next = list->tail->next;
list->tail->next = n;
}
list->size++;
return 0;
}

int circularlinkedlist_push_back(CircularLinkedList* list, void* data) {
if (!list) return -1;
CLNode* n = clnode_create(data);
if (!n) return -1;
if (!list->tail) {
list->tail = n;
} else {
n->next = list->tail->next;
list->tail->next = n;
list->tail = n;
}
list->size++;
return 0;
}

void* circularlinkedlist_pop_front(CircularLinkedList* list) {
if (!list || !list->tail) return NULL;
CLNode* head = list->tail->next;
void* data = head->data;
if (head == list->tail) {
list->tail = NULL;
} else {
list->tail->next = head->next;
}
free(head);
list->size--;
return data;
}

void* circularlinkedlist_pop_back(CircularLinkedList* list) {
if (!list || !list->tail) return NULL;
CLNode* head = list->tail->next;
void* data = list->tail->data;
if (head == list->tail) {
free(list->tail);
list->tail = NULL;
} else {
CLNode* cur = head;
while (cur->next != list->tail) cur = cur->next;
cur->next = head;
free(list->tail);
list->tail = cur;
}
list->size--;
return data;
}

void* circularlinkedlist_peek_front(CircularLinkedList* list) {
if (!list || !list->tail) return NULL;
return list->tail->next->data;
}

void* circularlinkedlist_peek_back(CircularLinkedList* list) {
return (list && list->tail) ? list->tail->data : NULL;
}

int circularlinkedlist_size(CircularLinkedList* list) { return list ? list->size : 0; }