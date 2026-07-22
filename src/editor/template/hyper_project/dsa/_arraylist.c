#include "_arraylist.h"
#include <stdlib.h>
#include <string.h>

ArrayList* arraylist_create(int initial_capacity) {
ArrayList* list = (ArrayList*)malloc(sizeof(ArrayList));
if (!list) return NULL;
if (initial_capacity <= 0) initial_capacity = 16;
list->data = (void**)malloc(sizeof(void*) * initial_capacity);
if (!list->data) { free(list); return NULL; }
list->capacity = initial_capacity;
list->size = 0;
return list;
}

void arraylist_destroy(ArrayList* list, void (*free_elem)(void*)) {
if (!list) return;
if (free_elem) {
for (int i = 0; i < list->size; i++)
free_elem(list->data[i]);
}
free(list->data);
free(list);
}

static int grow(ArrayList* list) {
int new_cap = list->capacity * 2;
void** nd = (void**)realloc(list->data, sizeof(void*) * new_cap);
if (!nd) return -1;
list->data = nd;
list->capacity = new_cap;
return 0;
}

int arraylist_append(ArrayList* list, void* elem) {
if (!list) return -1;
if (list->size >= list->capacity && grow(list) != 0) return -1;
list->data[list->size++] = elem;
return 0;
}

int arraylist_insert(ArrayList* list, int index, void* elem) {
if (!list || index < 0 || index > list->size) return -1;
if (list->size >= list->capacity && grow(list) != 0) return -1;
memmove(&list->data[index + 1], &list->data[index],
sizeof(void*) * (list->size - index));
list->data[index] = elem;
list->size++;
return 0;
}

void* arraylist_get(ArrayList* list, int index) {
if (!list || index < 0 || index >= list->size) return NULL;
return list->data[index];
}

int arraylist_set(ArrayList* list, int index, void* elem) {
if (!list || index < 0 || index >= list->size) return -1;
list->data[index] = elem;
return 0;
}

void* arraylist_remove(ArrayList* list, int index) {
if (!list || index < 0 || index >= list->size) return NULL;
void* old = list->data[index];
memmove(&list->data[index], &list->data[index + 1],
sizeof(void*) * (list->size - index - 1));
list->size--;
return old;
}

int arraylist_size(ArrayList* list) { return list ? list->size : 0; }
int arraylist_capacity(ArrayList* list) { return list ? list->capacity : 0; }