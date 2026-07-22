#include "_stack.h"
#include <stdlib.h>

Stack* stack_create(int initial_capacity) {
Stack* s = (Stack*)malloc(sizeof(Stack));
if (!s) return NULL;
if (initial_capacity <= 0) initial_capacity = 16;
s->data = (void**)malloc(sizeof(void*) * initial_capacity);
if (!s->data) { free(s); return NULL; }
s->capacity = initial_capacity;
s->top = 0;
return s;
}

void stack_destroy(Stack* s, void (*free_elem)(void*)) {
if (!s) return;
if (free_elem)
for (int i = 0; i < s->top; i++) free_elem(s->data[i]);
free(s->data);
free(s);
}

int stack_push(Stack* s, void* elem) {
if (!s) return -1;
if (s->top >= s->capacity) {
int nc = s->capacity * 2;
void** nd = (void**)realloc(s->data, sizeof(void*) * nc);
if (!nd) return -1;
s->data = nd;
s->capacity = nc;
}
s->data[s->top++] = elem;
return 0;
}

void* stack_pop(Stack* s) {
if (!s || s->top == 0) return NULL;
return s->data[--s->top];
}

void* stack_peek(Stack* s) {
if (!s || s->top == 0) return NULL;
return s->data[s->top - 1];
}

int stack_is_empty(Stack* s) { return !s || s->top == 0; }
int stack_size(Stack* s) { return s ? s->top : 0; }