typedef struct Stack {
    void** data;
    int capacity;
    int top;
} Stack;

Stack* stack_create(int initial_capacity);
void   stack_destroy(Stack* s, void (*free_elem)(void*));
int    stack_push(Stack* s, void* elem);
void*  stack_pop(Stack* s);
void*  stack_peek(Stack* s);
int    stack_is_empty(Stack* s);
int    stack_size(Stack* s);