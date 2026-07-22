#include "_queue.h"
#include <stdlib.h>

Queue* queue_create(int initial_capacity) {
Queue* q = (Queue*)malloc(sizeof(Queue));
if (!q) return NULL;
if (initial_capacity <= 0) initial_capacity = 16;
q->data = (void**)malloc(sizeof(void*) * initial_capacity);
if (!q->data) { free(q); return NULL; }
q->capacity = initial_capacity;
q->head = q->tail = q->count = 0;
return q;
}

void queue_destroy(Queue* q, void (*free_elem)(void*)) {
if (!q) return;
if (free_elem) {
for (int i = 0; i < q->count; i++) {
int idx = (q->head + i) % q->capacity;
free_elem(q->data[idx]);
}
}
free(q->data);
free(q);
}

static int queue_grow(Queue* q) {
int nc = q->capacity * 2;
void** nd = (void**)malloc(sizeof(void*) * nc);
if (!nd) return -1;
for (int i = 0; i < q->count; i++)
nd[i] = q->data[(q->head + i) % q->capacity];
free(q->data);
q->data = nd;
q->capacity = nc;
q->head = 0;
q->tail = q->count;
return 0;
}

int queue_enqueue(Queue* q, void* elem) {
if (!q) return -1;
if (q->count >= q->capacity && queue_grow(q) != 0) return -1;
q->data[q->tail] = elem;
q->tail = (q->tail + 1) % q->capacity;
q->count++;
return 0;
}

void* queue_dequeue(Queue* q) {
if (!q || q->count == 0) return NULL;
void* elem = q->data[q->head];
q->head = (q->head + 1) % q->capacity;
q->count--;
return elem;
}

void* queue_peek(Queue* q) {
if (!q || q->count == 0) return NULL;
return q->data[q->head];
}

int queue_is_empty(Queue* q) { return !q || q->count == 0; }
int queue_size(Queue* q) { return q ? q->count : 0; }