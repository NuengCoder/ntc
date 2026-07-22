#include "_heap.h"
typedef Heap PriorityQueue;
#define priorityqueue_create(cap, cmp)  heap_create(cap, cmp)
#define priorityqueue_destroy(pq, fe)   heap_destroy(pq, fe)
#define priorityqueue_enqueue(pq, e)    heap_push(pq, e)
#define priorityqueue_dequeue(pq)       heap_pop(pq)
#define priorityqueue_peek(pq)          heap_peek(pq)
#define priorityqueue_size(pq)          heap_size(pq)