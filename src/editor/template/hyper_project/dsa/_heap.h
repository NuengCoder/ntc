typedef struct Heap {
    void** data;
    int capacity;
    int size;
    int (*cmp)(const void*, const void*);
} Heap;

Heap* heap_create(int initial_capacity, int (*cmp)(const void*, const void*));
void  heap_destroy(Heap* h, void (*free_elem)(void*));
int   heap_push(Heap* h, void* elem);
void* heap_pop(Heap* h);
void* heap_peek(Heap* h);
int   heap_size(Heap* h);