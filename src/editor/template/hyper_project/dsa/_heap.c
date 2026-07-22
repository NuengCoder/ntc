#include "_heap.h"
#include <stdlib.h>
#include <string.h>

Heap* heap_create(int initial_capacity, int (*cmp)(const void*, const void*)) {
Heap* h = (Heap*)malloc(sizeof(Heap));
if (!h) return NULL;
if (initial_capacity <= 0) initial_capacity = 16;
h->data = (void**)malloc(sizeof(void*) * initial_capacity);
if (!h->data) { free(h); return NULL; }
h->capacity = initial_capacity;
h->size = 0;
h->cmp = cmp;
return h;
}

void heap_destroy(Heap* h, void (*free_elem)(void*)) {
if (!h) return;
if (free_elem)
for (int i = 0; i < h->size; i++) free_elem(h->data[i]);
free(h->data);
free(h);
}

static void swap(void** a, void** b) { void* t = *a; *a = *b; *b = t; }

int heap_push(Heap* h, void* elem) {
if (!h) return -1;
if (h->size >= h->capacity) {
int nc = h->capacity * 2;
void** nd = (void**)realloc(h->data, sizeof(void*) * nc);
if (!nd) return -1;
h->data = nd;
h->capacity = nc;
}
int i = h->size++;
h->data[i] = elem;
while (i > 0) {
int p = (i - 1) / 2;
if (h->cmp(h->data[i], h->data[p]) >= 0) break;
swap(&h->data[i], &h->data[p]);
i = p;
}
return 0;
}

void* heap_pop(Heap* h) {
if (!h || h->size == 0) return NULL;
void* top = h->data[0];
h->data[0] = h->data[--h->size];
int i = 0;
while (1) {
int smallest = i;
int l = 2 * i + 1, r = 2 * i + 2;
if (l < h->size && h->cmp(h->data[l], h->data[smallest]) < 0) smallest = l;
if (r < h->size && h->cmp(h->data[r], h->data[smallest]) < 0) smallest = r;
if (smallest == i) break;
swap(&h->data[i], &h->data[smallest]);
i = smallest;
}
return top;
}

void* heap_peek(Heap* h) { return (h && h->size > 0) ? h->data[0] : NULL; }
int heap_size(Heap* h) { return h ? h->size : 0; }