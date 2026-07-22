typedef struct Queue {
    void** data;
    int capacity;
    int head;
    int tail;
    int count;
} Queue;

Queue* queue_create(int initial_capacity);
void   queue_destroy(Queue* q, void (*free_elem)(void*));
int    queue_enqueue(Queue* q, void* elem);
void*  queue_dequeue(Queue* q);
void*  queue_peek(Queue* q);
int    queue_is_empty(Queue* q);
int    queue_size(Queue* q);