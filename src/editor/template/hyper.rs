use std::path::Path;
use std::fs;

use anyhow::{Result, bail};

use super::simple::generate_header_guard;

fn guard(name: &str) -> String {
    generate_header_guard(name, "h")
}

fn write_file(path: &Path, content: &str) -> Result<()> {
    fs::write(path, content)?;
    Ok(())
}

pub fn init_hyper_project(name: &str, lang: &str) -> Result<bool> {
    if lang != "c" {
        bail!("Unsupported language '{}'. Only 'c' is supported.", lang);
    }
    if name.contains('.') {
        bail!("Project name '{}' contains an extension. Use a name without dots.", name);
    }

    let root = Path::new(name);
    if root.exists() {
        bail!("Directory '{}' already exists.", name);
    }

    println!("Creating hyper project '{}' (C)...", name);

    for dir in &["", "dsa", "debug", "os", "header", "ntclib"] {
        fs::create_dir_all(root.join(dir))?;
    }

    let r = |p: &str| root.join(p);

    // DSA headers + sources
    write_file(&r("dsa/_arraylist.h"), &arraylist_h())?;
    write_file(&r("dsa/_arraylist.c"), arraylist_c())?;
    write_file(&r("dsa/_linkedlist.h"), &linkedlist_h())?;
    write_file(&r("dsa/_linkedlist.c"), linkedlist_c())?;
    write_file(&r("dsa/_doublylinkedlist.h"), &doublylinkedlist_h())?;
    write_file(&r("dsa/_doublylinkedlist.c"), doublylinkedlist_c())?;
    write_file(&r("dsa/_circularlinkedlist.h"), &circularlinkedlist_h())?;
    write_file(&r("dsa/_circularlinkedlist.c"), circularlinkedlist_c())?;
    write_file(&r("dsa/_stack.h"), &stack_h())?;
    write_file(&r("dsa/_stack.c"), stack_c())?;
    write_file(&r("dsa/_queue.h"), &queue_h())?;
    write_file(&r("dsa/_queue.c"), queue_c())?;
    write_file(&r("dsa/_binarytree.h"), &binarytree_h())?;
    write_file(&r("dsa/_binarytree.c"), binarytree_c())?;
    write_file(&r("dsa/_tree.h"), &tree_h())?;
    write_file(&r("dsa/_tree.c"), tree_c())?;
    write_file(&r("dsa/_hashmap.h"), &hashmap_h())?;
    write_file(&r("dsa/_hashmap.c"), hashmap_c())?;
    write_file(&r("dsa/_hashtable.h"), &hashtable_h())?;
    write_file(&r("dsa/_hashtable.c"), hashtable_c())?;
    write_file(&r("dsa/_hashset.h"), &hashset_h())?;
    write_file(&r("dsa/_hashset.c"), hashset_c())?;
    write_file(&r("dsa/_hash.h"), &hash_h())?;
    write_file(&r("dsa/_hash.c"), hash_c())?;
    write_file(&r("dsa/_heap.h"), &heap_h())?;
    write_file(&r("dsa/_heap.c"), heap_c())?;
    write_file(&r("dsa/_priorityqueue.h"), &priorityqueue_h())?;
    write_file(&r("dsa/_priorityqueue.c"), priorityqueue_c())?;
    write_file(&r("dsa/_graph.h"), &graph_h())?;
    write_file(&r("dsa/_graph.c"), graph_c())?;
    write_file(&r("dsa/_map.h"), &map_h())?;
    write_file(&r("dsa/_map.c"), map_c())?;
    write_file(&r("dsa/_set.h"), &set_h())?;
    write_file(&r("dsa/_set.c"), set_c())?;
    write_file(&r("dsa/_rope.h"), &rope_h())?;
    write_file(&r("dsa/_rope.c"), rope_c())?;
    write_file(&r("dsa/_trie.h"), &trie_h())?;
    write_file(&r("dsa/_trie.c"), trie_c())?;
    write_file(&r("dsa/_lsearch.h"), &lsearch_h())?;
    write_file(&r("dsa/_lsearch.c"), lsearch_c())?;
    write_file(&r("dsa/_bsearch.h"), &bsearch_h())?;
    write_file(&r("dsa/_bsearch.c"), bsearch_c())?;

    // Debug, OS, header, ntclib
    write_file(&r("debug/debug.h"), &debug_h())?;
    write_file(&r("debug/debug.c"), debug_c())?;
    write_file(&r("os/os.h"), &os_h())?;
    write_file(&r("os/os.c"), os_c())?;
    write_file(&r("header/ntc.h"), &ntc_h())?;
    write_file(&r("ntclib/ntc_lib.h"), &ntclib_h())?;
    write_file(&r("ntclib/ntc_lib.c"), ntclib_c())?;

    // Example files
    write_file(&r("ntc_main.c"), &ntc_main_c(name))?;
    write_file(&r("ntc_lib.c"), ntc_lib_c())?;

    // NTCRANFILE.toml
    write_file(&r("NTCRANFILE.toml"), &ntc_ranfile_toml(name))?;

    println!("Hyper project '{}' created successfully!", name);
    Ok(true)
}

// ── ArrayList ─────────────────────────────────────────────────────────────────

fn arraylist_h() -> String {
    let g = guard("_arraylist_h");
    format!("\
#ifndef {g}
#define {g}

typedef struct ArrayList {{
    void** data;
    int capacity;
    int size;
}} ArrayList;

ArrayList* arraylist_create(int initial_capacity);
void      arraylist_destroy(ArrayList* list, void (*free_elem)(void*));
int       arraylist_append(ArrayList* list, void* elem);
int       arraylist_insert(ArrayList* list, int index, void* elem);
void*     arraylist_get(ArrayList* list, int index);
int       arraylist_set(ArrayList* list, int index, void* elem);
void*     arraylist_remove(ArrayList* list, int index);
int       arraylist_size(ArrayList* list);
int       arraylist_capacity(ArrayList* list);

#endif /* {g} */
")
}

fn arraylist_c() -> &'static str {
    "\
#include \"_arraylist.h\"
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
"
}

// ── LinkedList ────────────────────────────────────────────────────────────────

fn linkedlist_h() -> String {
    let g = guard("_linkedlist_h");
    format!("\
#ifndef {g}
#define {g}

typedef struct LLNode {{
    void* data;
    struct LLNode* next;
}} LLNode;

typedef struct LinkedList {{
    LLNode* head;
    int size;
}} LinkedList;

LinkedList* linkedlist_create(void);
void        linkedlist_destroy(LinkedList* list, void (*free_elem)(void*));
int         linkedlist_push_front(LinkedList* list, void* data);
int         linkedlist_push_back(LinkedList* list, void* data);
void*       linkedlist_pop_front(LinkedList* list);
void*       linkedlist_pop_back(LinkedList* list);
void*       linkedlist_peek_front(LinkedList* list);
void*       linkedlist_peek_back(LinkedList* list);
LLNode*     linkedlist_find(LinkedList* list, void* data,
                            int (*cmp)(const void*, const void*));
int         linkedlist_remove(LinkedList* list, void* data,
                              int (*cmp)(const void*, const void*),
                              void (*free_elem)(void*));
int         linkedlist_size(LinkedList* list);

#endif /* {g} */
")
}

fn linkedlist_c() -> &'static str {
    "\
#include \"_linkedlist.h\"
#include <stdlib.h>

LinkedList* linkedlist_create(void) {
    LinkedList* list = (LinkedList*)malloc(sizeof(LinkedList));
    if (!list) return NULL;
    list->head = NULL;
    list->size = 0;
    return list;
}

void linkedlist_destroy(LinkedList* list, void (*free_elem)(void*)) {
    if (!list) return;
    LLNode* cur = list->head;
    while (cur) {
        LLNode* next = cur->next;
        if (free_elem) free_elem(cur->data);
        free(cur);
        cur = next;
    }
    free(list);
}

static LLNode* node_create(void* data) {
    LLNode* n = (LLNode*)malloc(sizeof(LLNode));
    if (!n) return NULL;
    n->data = data;
    n->next = NULL;
    return n;
}

int linkedlist_push_front(LinkedList* list, void* data) {
    if (!list) return -1;
    LLNode* n = node_create(data);
    if (!n) return -1;
    n->next = list->head;
    list->head = n;
    list->size++;
    return 0;
}

int linkedlist_push_back(LinkedList* list, void* data) {
    if (!list) return -1;
    LLNode* n = node_create(data);
    if (!n) return -1;
    if (!list->head) {
        list->head = n;
    } else {
        LLNode* cur = list->head;
        while (cur->next) cur = cur->next;
        cur->next = n;
    }
    list->size++;
    return 0;
}

void* linkedlist_pop_front(LinkedList* list) {
    if (!list || !list->head) return NULL;
    LLNode* old = list->head;
    void* data = old->data;
    list->head = old->next;
    free(old);
    list->size--;
    return data;
}

void* linkedlist_pop_back(LinkedList* list) {
    if (!list || !list->head) return NULL;
    if (!list->head->next) return linkedlist_pop_front(list);
    LLNode* cur = list->head;
    while (cur->next->next) cur = cur->next;
    void* data = cur->next->data;
    free(cur->next);
    cur->next = NULL;
    list->size--;
    return data;
}

void* linkedlist_peek_front(LinkedList* list) {
    return (list && list->head) ? list->head->data : NULL;
}

void* linkedlist_peek_back(LinkedList* list) {
    if (!list || !list->head) return NULL;
    LLNode* cur = list->head;
    while (cur->next) cur = cur->next;
    return cur->data;
}

LLNode* linkedlist_find(LinkedList* list, void* data,
                        int (*cmp)(const void*, const void*)) {
    if (!list || !cmp) return NULL;
    LLNode* cur = list->head;
    while (cur) {
        if (cmp(cur->data, data) == 0) return cur;
        cur = cur->next;
    }
    return NULL;
}

int linkedlist_remove(LinkedList* list, void* data,
                      int (*cmp)(const void*, const void*),
                      void (*free_elem)(void*)) {
    if (!list || !cmp || !list->head) return -1;
    LLNode* cur = list->head;
    LLNode* prev = NULL;
    while (cur) {
        if (cmp(cur->data, data) == 0) {
            if (prev) prev->next = cur->next;
            else list->head = cur->next;
            if (free_elem) free_elem(cur->data);
            free(cur);
            list->size--;
            return 0;
        }
        prev = cur;
        cur = cur->next;
    }
    return -1;
}

int linkedlist_size(LinkedList* list) { return list ? list->size : 0; }
"
}

// ── DoublyLinkedList ──────────────────────────────────────────────────────────

fn doublylinkedlist_h() -> String {
    let g = guard("_doublylinkedlist_h");
    format!("\
#ifndef {g}
#define {g}

typedef struct DLNode {{
    void* data;
    struct DLNode* prev;
    struct DLNode* next;
}} DLNode;

typedef struct DoublyLinkedList {{
    DLNode* head;
    DLNode* tail;
    int size;
}} DoublyLinkedList;

DoublyLinkedList* doublylinkedlist_create(void);
void              doublylinkedlist_destroy(DoublyLinkedList* list, void (*free_elem)(void*));
int               doublylinkedlist_push_front(DoublyLinkedList* list, void* data);
int               doublylinkedlist_push_back(DoublyLinkedList* list, void* data);
void*             doublylinkedlist_pop_front(DoublyLinkedList* list);
void*             doublylinkedlist_pop_back(DoublyLinkedList* list);
void*             doublylinkedlist_peek_front(DoublyLinkedList* list);
void*             doublylinkedlist_peek_back(DoublyLinkedList* list);
int               doublylinkedlist_size(DoublyLinkedList* list);

#endif /* {g} */
")
}

fn doublylinkedlist_c() -> &'static str {
    "\
#include \"_doublylinkedlist.h\"
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
"
}

// ── CircularLinkedList ────────────────────────────────────────────────────────

fn circularlinkedlist_h() -> String {
    let g = guard("_circularlinkedlist_h");
    format!("\
#ifndef {g}
#define {g}

typedef struct CLNode {{
    void* data;
    struct CLNode* next;
}} CLNode;

typedef struct CircularLinkedList {{
    CLNode* tail;
    int size;
}} CircularLinkedList;

CircularLinkedList* circularlinkedlist_create(void);
void                circularlinkedlist_destroy(CircularLinkedList* list, void (*free_elem)(void*));
int                 circularlinkedlist_push_front(CircularLinkedList* list, void* data);
int                 circularlinkedlist_push_back(CircularLinkedList* list, void* data);
void*               circularlinkedlist_pop_front(CircularLinkedList* list);
void*               circularlinkedlist_pop_back(CircularLinkedList* list);
void*               circularlinkedlist_peek_front(CircularLinkedList* list);
void*               circularlinkedlist_peek_back(CircularLinkedList* list);
int                 circularlinkedlist_size(CircularLinkedList* list);

#endif /* {g} */
")
}

fn circularlinkedlist_c() -> &'static str {
    "\
#include \"_circularlinkedlist.h\"
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
"
}

// ── Stack ─────────────────────────────────────────────────────────────────────

fn stack_h() -> String {
    let g = guard("_stack_h");
    format!("\
#ifndef {g}
#define {g}

typedef struct Stack {{
    void** data;
    int capacity;
    int top;
}} Stack;

Stack* stack_create(int initial_capacity);
void   stack_destroy(Stack* s, void (*free_elem)(void*));
int    stack_push(Stack* s, void* elem);
void*  stack_pop(Stack* s);
void*  stack_peek(Stack* s);
int    stack_is_empty(Stack* s);
int    stack_size(Stack* s);

#endif /* {g} */
")
}

fn stack_c() -> &'static str {
    "\
#include \"_stack.h\"
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
"
}

// ── Queue ─────────────────────────────────────────────────────────────────────

fn queue_h() -> String {
    let g = guard("_queue_h");
    format!("\
#ifndef {g}
#define {g}

typedef struct Queue {{
    void** data;
    int capacity;
    int head;
    int tail;
    int count;
}} Queue;

Queue* queue_create(int initial_capacity);
void   queue_destroy(Queue* q, void (*free_elem)(void*));
int    queue_enqueue(Queue* q, void* elem);
void*  queue_dequeue(Queue* q);
void*  queue_peek(Queue* q);
int    queue_is_empty(Queue* q);
int    queue_size(Queue* q);

#endif /* {g} */
")
}

fn queue_c() -> &'static str {
    "\
#include \"_queue.h\"
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
"
}

// ── BinaryTree ────────────────────────────────────────────────────────────────

fn binarytree_h() -> String {
    let g = guard("_binarytree_h");
    format!("\
#ifndef {g}
#define {g}

typedef struct BTNode {{
    void* data;
    struct BTNode* left;
    struct BTNode* right;
}} BTNode;

typedef struct BinaryTree {{
    BTNode* root;
    int (*cmp)(const void*, const void*);
    int size;
}} BinaryTree;

BinaryTree* binarytree_create(int (*cmp)(const void*, const void*));
void        binarytree_destroy(BinaryTree* tree, void (*free_elem)(void*));
int         binarytree_insert(BinaryTree* tree, void* data);
BTNode*     binarytree_search(BinaryTree* tree, void* data);
int         binarytree_delete(BinaryTree* tree, void* data, void (*free_elem)(void*));
void        binarytree_inorder(BinaryTree* tree, void (*visit)(void*));
void        binarytree_preorder(BinaryTree* tree, void (*visit)(void*));
void        binarytree_postorder(BinaryTree* tree, void (*visit)(void*));
int         binarytree_size(BinaryTree* tree);

#endif /* {g} */
")
}

fn binarytree_c() -> &'static str {
    "\
#include \"_binarytree.h\"
#include <stdlib.h>

static BTNode* btn_create(void* data) {
    BTNode* n = (BTNode*)malloc(sizeof(BTNode));
    if (!n) return NULL;
    n->data = data;
    n->left = n->right = NULL;
    return n;
}

BinaryTree* binarytree_create(int (*cmp)(const void*, const void*)) {
    BinaryTree* t = (BinaryTree*)malloc(sizeof(BinaryTree));
    if (!t) return NULL;
    t->root = NULL;
    t->cmp = cmp;
    t->size = 0;
    return t;
}

static void destroy_sub(BTNode* n, void (*fe)(void*)) {
    if (!n) return;
    destroy_sub(n->left, fe);
    destroy_sub(n->right, fe);
    if (fe) fe(n->data);
    free(n);
}

void binarytree_destroy(BinaryTree* tree, void (*free_elem)(void*)) {
    if (!tree) return;
    destroy_sub(tree->root, free_elem);
    free(tree);
}

int binarytree_insert(BinaryTree* tree, void* data) {
    if (!tree) return -1;
    BTNode* n = btn_create(data);
    if (!n) return -1;
    if (!tree->root) { tree->root = n; tree->size++; return 0; }
    BTNode* cur = tree->root;
    while (1) {
        int c = tree->cmp(data, cur->data);
        if (c < 0) {
            if (!cur->left) { cur->left = n; break; }
            cur = cur->left;
        } else {
            if (!cur->right) { cur->right = n; break; }
            cur = cur->right;
        }
    }
    tree->size++;
    return 0;
}

BTNode* binarytree_search(BinaryTree* tree, void* data) {
    if (!tree || !tree->cmp) return NULL;
    BTNode* cur = tree->root;
    while (cur) {
        int c = tree->cmp(data, cur->data);
        if (c == 0) return cur;
        cur = c < 0 ? cur->left : cur->right;
    }
    return NULL;
}

static BTNode* min_node(BTNode* n) {
    while (n && n->left) n = n->left;
    return n;
}

static BTNode* delete_sub(BTNode* n, void* data, int (*cmp)(const void*, const void*),
                          void (*fe)(void*), int* removed) {
    if (!n) return NULL;
    int c = cmp(data, n->data);
    if (c < 0) n->left = delete_sub(n->left, data, cmp, fe, removed);
    else if (c > 0) n->right = delete_sub(n->right, data, cmp, fe, removed);
    else {
        *removed = 1;
        if (fe) fe(n->data);
        if (!n->left) { BTNode* r = n->right; free(n); return r; }
        if (!n->right) { BTNode* l = n->left; free(n); return l; }
        BTNode* m = min_node(n->right);
        n->data = m->data;
        n->right = delete_sub(n->right, m->data, cmp, NULL, &(int){0});
    }
    return n;
}

int binarytree_delete(BinaryTree* tree, void* data, void (*free_elem)(void*)) {
    if (!tree) return -1;
    int removed = 0;
    tree->root = delete_sub(tree->root, data, tree->cmp, free_elem, &removed);
    if (removed) tree->size--;
    return removed ? 0 : -1;
}

static void inorder_sub(BTNode* n, void (*visit)(void*)) {
    if (!n) return;
    inorder_sub(n->left, visit);
    visit(n->data);
    inorder_sub(n->right, visit);
}

static void preorder_sub(BTNode* n, void (*visit)(void*)) {
    if (!n) return;
    visit(n->data);
    preorder_sub(n->left, visit);
    preorder_sub(n->right, visit);
}

static void postorder_sub(BTNode* n, void (*visit)(void*)) {
    if (!n) return;
    postorder_sub(n->left, visit);
    postorder_sub(n->right, visit);
    visit(n->data);
}

void binarytree_inorder(BinaryTree* tree, void (*visit)(void*)) {
    if (tree) inorder_sub(tree->root, visit);
}

void binarytree_preorder(BinaryTree* tree, void (*visit)(void*)) {
    if (tree) preorder_sub(tree->root, visit);
}

void binarytree_postorder(BinaryTree* tree, void (*visit)(void*)) {
    if (tree) postorder_sub(tree->root, visit);
}

int binarytree_size(BinaryTree* tree) { return tree ? tree->size : 0; }
"
}

// ── Tree (n-ary) ──────────────────────────────────────────────────────────────

fn tree_h() -> String {
    let g = guard("_tree_h");
    format!("\
#ifndef {g}
#define {g}

typedef struct TreeNode {{
    void* data;
    struct TreeNode** children;
    int child_count;
    int child_capacity;
}} TreeNode;

typedef struct Tree {{
    TreeNode* root;
    int size;
}} Tree;

Tree*     tree_create(void);
void      tree_destroy(Tree* tree, void (*free_elem)(void*));
TreeNode* tree_add_child(Tree* tree, TreeNode* parent, void* data);
void      tree_traverse_dfs(Tree* tree, void (*visit)(void*));
void      tree_traverse_bfs(Tree* tree, void (*visit)(void*));
int       tree_size(Tree* tree);

#endif /* {g} */
")
}

fn tree_c() -> &'static str {
    "\
#include \"_tree.h\"
#include <stdlib.h>

static TreeNode* tn_create(void* data) {
    TreeNode* n = (TreeNode*)malloc(sizeof(TreeNode));
    if (!n) return NULL;
    n->data = data;
    n->children = NULL;
    n->child_count = 0;
    n->child_capacity = 0;
    return n;
}

Tree* tree_create(void) {
    Tree* t = (Tree*)malloc(sizeof(Tree));
    if (!t) return NULL;
    t->root = NULL;
    t->size = 0;
    return t;
}

static void destroy_tree_node(TreeNode* n, void (*fe)(void*)) {
    if (!n) return;
    for (int i = 0; i < n->child_count; i++)
        destroy_tree_node(n->children[i], fe);
    if (fe) fe(n->data);
    free(n->children);
    free(n);
}

void tree_destroy(Tree* tree, void (*free_elem)(void*)) {
    if (!tree) return;
    destroy_tree_node(tree->root, free_elem);
    free(tree);
}

TreeNode* tree_add_child(Tree* tree, TreeNode* parent, void* data) {
    if (!tree) return NULL;
    TreeNode* n = tn_create(data);
    if (!n) return NULL;
    if (!parent) {
        if (tree->root) {
            free(n);
            return NULL;
        }
        tree->root = n;
        tree->size++;
        return n;
    }
    if (parent->child_count >= parent->child_capacity) {
        int nc = parent->child_capacity == 0 ? 4 : parent->child_capacity * 2;
        TreeNode** nd = (TreeNode**)realloc(parent->children, sizeof(TreeNode*) * nc);
        if (!nd) { free(n); return NULL; }
        parent->children = nd;
        parent->child_capacity = nc;
    }
    parent->children[parent->child_count++] = n;
    tree->size++;
    return n;
}

static void dfs_sub(TreeNode* n, void (*visit)(void*)) {
    if (!n) return;
    visit(n->data);
    for (int i = 0; i < n->child_count; i++)
        dfs_sub(n->children[i], visit);
}

void tree_traverse_dfs(Tree* tree, void (*visit)(void*)) {
    if (tree) dfs_sub(tree->root, visit);
}

void tree_traverse_bfs(Tree* tree, void (*visit)(void*)) {
    if (!tree || !tree->root || !visit) return;
    TreeNode** q = (TreeNode**)malloc(sizeof(TreeNode*) * tree->size);
    if (!q) return;
    int front = 0, back = 0;
    q[back++] = tree->root;
    while (front < back) {
        TreeNode* n = q[front++];
        visit(n->data);
        for (int i = 0; i < n->child_count; i++)
            q[back++] = n->children[i];
    }
    free(q);
}

int tree_size(Tree* tree) { return tree ? tree->size : 0; }
"
}

// ── HashMap ───────────────────────────────────────────────────────────────────

fn hashmap_h() -> String {
    let g = guard("_hashmap_h");
    format!("\
#ifndef {g}
#define {g}

typedef struct HMEntry {{
    char* key;
    void* value;
    int  occupied;
}} HMEntry;

typedef struct HashMap {{
    HMEntry* buckets;
    int capacity;
    int count;
}} HashMap;

HashMap* hashmap_create(int initial_capacity);
void     hashmap_destroy(HashMap* map, void (*free_val)(void*));
int      hashmap_put(HashMap* map, const char* key, void* value);
void*    hashmap_get(HashMap* map, const char* key);
int      hashmap_has_key(HashMap* map, const char* key);
void*    hashmap_remove(HashMap* map, const char* key);
char**   hashmap_keys(HashMap* map, int* count);
int      hashmap_size(HashMap* map);

#endif /* {g} */
")
}

fn hashmap_c() -> &'static str {
    "\
#include \"_hashmap.h\"
#include <stdlib.h>
#include <string.h>

static unsigned long hash_str(const char* s) {
    unsigned long h = 5381;
    int c;
    while ((c = *s++)) h = ((h << 5) + h) + (unsigned char)c;
    return h;
}

HashMap* hashmap_create(int initial_capacity) {
    HashMap* m = (HashMap*)malloc(sizeof(HashMap));
    if (!m) return NULL;
    if (initial_capacity <= 0) initial_capacity = 32;
    m->buckets = (HMEntry*)calloc(initial_capacity, sizeof(HMEntry));
    if (!m->buckets) { free(m); return NULL; }
    m->capacity = initial_capacity;
    m->count = 0;
    return m;
}

void hashmap_destroy(HashMap* map, void (*free_val)(void*)) {
    if (!map) return;
    for (int i = 0; i < map->capacity; i++) {
        if (map->buckets[i].occupied) {
            free(map->buckets[i].key);
            if (free_val) free_val(map->buckets[i].value);
        }
    }
    free(map->buckets);
    free(map);
}

static int find_slot(HashMap* map, const char* key) {
    unsigned long h = hash_str(key);
    int idx = (int)(h % (unsigned long)map->capacity);
    for (int i = 0; i < map->capacity; i++) {
        int si = (idx + i) % map->capacity;
        if (!map->buckets[si].occupied) return si;
        if (strcmp(map->buckets[si].key, key) == 0) return si;
    }
    return -1;
}

static int resize(HashMap* map) {
    int oc = map->capacity;
    HMEntry* ob = map->buckets;
    map->capacity = oc * 2;
    map->buckets = (HMEntry*)calloc(map->capacity, sizeof(HMEntry));
    if (!map->buckets) { map->buckets = ob; map->capacity = oc; return -1; }
    map->count = 0;
    for (int i = 0; i < oc; i++) {
        if (ob[i].occupied) {
            hashmap_put(map, ob[i].key, ob[i].value);
            free(ob[i].key);
        }
    }
    free(ob);
    return 0;
}

int hashmap_put(HashMap* map, const char* key, void* value) {
    if (!map || !key) return -1;
    if (map->count * 2 >= map->capacity && resize(map) != 0) return -1;
    int si = find_slot(map, key);
    if (si < 0) return -1;
    if (!map->buckets[si].occupied) {
        map->buckets[si].key = strdup(key);
        if (!map->buckets[si].key) return -1;
        map->buckets[si].occupied = 1;
        map->count++;
    }
    map->buckets[si].value = value;
    return 0;
}

void* hashmap_get(HashMap* map, const char* key) {
    if (!map || !key) return NULL;
    unsigned long h = hash_str(key);
    int idx = (int)(h % (unsigned long)map->capacity);
    for (int i = 0; i < map->capacity; i++) {
        int si = (idx + i) % map->capacity;
        if (!map->buckets[si].occupied) return NULL;
        if (strcmp(map->buckets[si].key, key) == 0)
            return map->buckets[si].value;
    }
    return NULL;
}

int hashmap_has_key(HashMap* map, const char* key) {
    return hashmap_get(map, key) != NULL;
}

void* hashmap_remove(HashMap* map, const char* key) {
    if (!map || !key) return NULL;
    unsigned long h = hash_str(key);
    int idx = (int)(h % (unsigned long)map->capacity);
    for (int i = 0; i < map->capacity; i++) {
        int si = (idx + i) % map->capacity;
        if (!map->buckets[si].occupied) return NULL;
        if (strcmp(map->buckets[si].key, key) == 0) {
            void* v = map->buckets[si].value;
            free(map->buckets[si].key);
            map->buckets[si].occupied = 0;
            map->count--;
            return v;
        }
    }
    return NULL;
}

char** hashmap_keys(HashMap* map, int* count) {
    if (!map || !count) return NULL;
    *count = map->count;
    char** ks = (char**)malloc(sizeof(char*) * map->count);
    if (!ks) { *count = 0; return NULL; }
    int j = 0;
    for (int i = 0; i < map->capacity; i++)
        if (map->buckets[i].occupied)
            ks[j++] = strdup(map->buckets[i].key);
    return ks;
}

int hashmap_size(HashMap* map) { return map ? map->count : 0; }
"
}

// ── HashTable (shim over HashMap) ─────────────────────────────────────────────

fn hashtable_h() -> String {
    let g = guard("_hashtable_h");
    format!("\
#ifndef {g}
#define {g}
#include \"_hashmap.h\"
typedef HashMap HashTable;
#define hashtable_create(cap)        hashmap_create(cap)
#define hashtable_destroy(m, fv)     hashmap_destroy(m, fv)
#define hashtable_put(m, k, v)       hashmap_put(m, k, v)
#define hashtable_get(m, k)          hashmap_get(m, k)
#define hashtable_has_key(m, k)      hashmap_has_key(m, k)
#define hashtable_remove(m, k)       hashmap_remove(m, k)
#define hashtable_keys(m, c)         hashmap_keys(m, c)
#define hashtable_size(m)            hashmap_size(m)
#endif /* {g} */
")
}

fn hashtable_c() -> &'static str {
    "\
#include \"_hashtable.h\"
/* HashTable is a typedef alias for HashMap — all operations are inherited. */
"
}

// ── HashSet ───────────────────────────────────────────────────────────────────

fn hashset_h() -> String {
    let g = guard("_hashset_h");
    format!("\
#ifndef {g}
#define {g}

typedef struct HSEntry {{
    char* key;
    int   occupied;
}} HSEntry;

typedef struct HashSet {{
    HSEntry* buckets;
    int capacity;
    int count;
}} HashSet;

HashSet* hashset_create(int initial_capacity);
void     hashset_destroy(HashSet* set);
int      hashset_add(HashSet* set, const char* key);
int      hashset_contains(HashSet* set, const char* key);
int      hashset_remove(HashSet* set, const char* key);
int      hashset_size(HashSet* set);

#endif /* {g} */
")
}

fn hashset_c() -> &'static str {
    "\
#include \"_hashset.h\"
#include <stdlib.h>
#include <string.h>

static unsigned long hs_hash(const char* s) {
    unsigned long h = 5381;
    int c;
    while ((c = *s++)) h = ((h << 5) + h) + (unsigned char)c;
    return h;
}

HashSet* hashset_create(int initial_capacity) {
    HashSet* s = (HashSet*)malloc(sizeof(HashSet));
    if (!s) return NULL;
    if (initial_capacity <= 0) initial_capacity = 32;
    s->buckets = (HSEntry*)calloc(initial_capacity, sizeof(HSEntry));
    if (!s->buckets) { free(s); return NULL; }
    s->capacity = initial_capacity;
    s->count = 0;
    return s;
}

void hashset_destroy(HashSet* set) {
    if (!set) return;
    for (int i = 0; i < set->capacity; i++)
        if (set->buckets[i].occupied) free(set->buckets[i].key);
    free(set->buckets);
    free(set);
}

static int hs_find_slot(HashSet* set, const char* key) {
    unsigned long h = hs_hash(key);
    int idx = (int)(h % (unsigned long)set->capacity);
    for (int i = 0; i < set->capacity; i++) {
        int si = (idx + i) % set->capacity;
        if (!set->buckets[si].occupied) return si;
        if (strcmp(set->buckets[si].key, key) == 0) return si;
    }
    return -1;
}

static int hs_resize(HashSet* set) {
    int oc = set->capacity;
    HSEntry* ob = set->buckets;
    set->capacity = oc * 2;
    set->buckets = (HSEntry*)calloc(set->capacity, sizeof(HSEntry));
    if (!set->buckets) { set->buckets = ob; set->capacity = oc; return -1; }
    int old_count = set->count;
    set->count = 0;
    for (int i = 0; i < oc; i++) {
        if (ob[i].occupied) {
            hashset_add(set, ob[i].key);
            free(ob[i].key);
        }
    }
    free(ob);
    return old_count != set->count ? -1 : 0;
}

int hashset_add(HashSet* set, const char* key) {
    if (!set || !key) return -1;
    if (set->count * 2 >= set->capacity && hs_resize(set) != 0) return -1;
    int si = hs_find_slot(set, key);
    if (si < 0) return -1;
    if (!set->buckets[si].occupied) {
        set->buckets[si].key = strdup(key);
        if (!set->buckets[si].key) return -1;
        set->buckets[si].occupied = 1;
        set->count++;
    }
    return 0;
}

int hashset_contains(HashSet* set, const char* key) {
    if (!set || !key) return 0;
    unsigned long h = hs_hash(key);
    int idx = (int)(h % (unsigned long)set->capacity);
    for (int i = 0; i < set->capacity; i++) {
        int si = (idx + i) % set->capacity;
        if (!set->buckets[si].occupied) return 0;
        if (strcmp(set->buckets[si].key, key) == 0) return 1;
    }
    return 0;
}

int hashset_remove(HashSet* set, const char* key) {
    if (!set || !key) return -1;
    unsigned long h = hs_hash(key);
    int idx = (int)(h % (unsigned long)set->capacity);
    for (int i = 0; i < set->capacity; i++) {
        int si = (idx + i) % set->capacity;
        if (!set->buckets[si].occupied) return -1;
        if (strcmp(set->buckets[si].key, key) == 0) {
            free(set->buckets[si].key);
            set->buckets[si].occupied = 0;
            set->count--;
            return 0;
        }
    }
    return -1;
}

int hashset_size(HashSet* set) { return set ? set->count : 0; }
"
}

// ── Hash ──────────────────────────────────────────────────────────────────────

fn hash_h() -> String {
    let g = guard("_hash_h");
    format!("\
#ifndef {g}
#define {g}

unsigned long long hash_djb2(const unsigned char* str);
unsigned long long hash_sdbm(const unsigned char* str);
unsigned long long hash_fnv1a(const unsigned char* str);
unsigned int       hash_murmur3_32(const void* key, int len);

#endif /* {g} */
")
}

fn hash_c() -> &'static str {
    "\
#include \"_hash.h\"

unsigned long long hash_djb2(const unsigned char* str) {
    unsigned long long hash = 5381;
    int c;
    while ((c = *str++)) hash = ((hash << 5) + hash) + (unsigned char)c;
    return hash;
}

unsigned long long hash_sdbm(const unsigned char* str) {
    unsigned long long hash = 0;
    int c;
    while ((c = *str++)) hash = (unsigned char)c + (hash << 6) + (hash << 16) - hash;
    return hash;
}

unsigned long long hash_fnv1a(const unsigned char* str) {
    unsigned long long hash = 14695981039346656037ULL;
    int c;
    while ((c = *str++)) { hash ^= (unsigned char)c; hash *= 1099511628211ULL; }
    return hash;
}

unsigned int hash_murmur3_32(const void* key, int len) {
    const unsigned char* data = (const unsigned char*)key;
    unsigned int h = 0x971e137b;
    for (int i = 0; i < len; i++) {
        h ^= data[i];
        h *= 0x5bd1e995;
        h ^= h >> 15;
    }
    return h;
}
"
}

// ── Heap ──────────────────────────────────────────────────────────────────────

fn heap_h() -> String {
    let g = guard("_heap_h");
    format!("\
#ifndef {g}
#define {g}

typedef struct Heap {{
    void** data;
    int capacity;
    int size;
    int (*cmp)(const void*, const void*);
}} Heap;

Heap* heap_create(int initial_capacity, int (*cmp)(const void*, const void*));
void  heap_destroy(Heap* h, void (*free_elem)(void*));
int   heap_push(Heap* h, void* elem);
void* heap_pop(Heap* h);
void* heap_peek(Heap* h);
int   heap_size(Heap* h);

#endif /* {g} */
")
}

fn heap_c() -> &'static str {
    "\
#include \"_heap.h\"
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
"
}

// ── PriorityQueue ─────────────────────────────────────────────────────────────

fn priorityqueue_h() -> String {
    let g = guard("_priorityqueue_h");
    format!("\
#ifndef {g}
#define {g}
#include \"_heap.h\"
typedef Heap PriorityQueue;
#define priorityqueue_create(cap, cmp)  heap_create(cap, cmp)
#define priorityqueue_destroy(pq, fe)   heap_destroy(pq, fe)
#define priorityqueue_enqueue(pq, e)    heap_push(pq, e)
#define priorityqueue_dequeue(pq)       heap_pop(pq)
#define priorityqueue_peek(pq)          heap_peek(pq)
#define priorityqueue_size(pq)          heap_size(pq)
#endif /* {g} */
")
}

fn priorityqueue_c() -> &'static str {
    "\
#include \"_priorityqueue.h\"
/* PriorityQueue is a typedef alias for Heap — all operations are inherited. */
"
}

// ── Graph ─────────────────────────────────────────────────────────────────────

fn graph_h() -> String {
    let g = guard("_graph_h");
    format!("\
#ifndef {g}
#define {g}

typedef struct GVertex {{
    int id;
    void* data;
    struct GEdge* edges;
    int edge_count;
    int edge_capacity;
}} GVertex;

typedef struct GEdge {{
    int dest_id;
    int weight;
}} GEdge;

typedef struct Graph {{
    GVertex* vertices;
    int vertex_count;
    int vertex_capacity;
    int directed;
}} Graph;

Graph* graph_create(int initial_capacity, int directed);
void   graph_destroy(Graph* g, void (*free_data)(void*));
int    graph_add_vertex(Graph* g, void* data);
int    graph_add_edge(Graph* g, int from, int to, int weight);
void   graph_bfs(Graph* g, int start, void (*visit)(int, void*));
void   graph_dfs(Graph* g, int start, void (*visit)(int, void*));
int    graph_vertex_count(Graph* g);

#endif /* {g} */
")
}

fn graph_c() -> &'static str {
    "\
#include \"_graph.h\"
#include <stdlib.h>
#include <string.h>

Graph* graph_create(int initial_capacity, int directed) {
    Graph* g = (Graph*)malloc(sizeof(Graph));
    if (!g) return NULL;
    if (initial_capacity <= 0) initial_capacity = 16;
    g->vertices = (GVertex*)calloc(initial_capacity, sizeof(GVertex));
    if (!g->vertices) { free(g); return NULL; }
    g->vertex_count = 0;
    g->vertex_capacity = initial_capacity;
    g->directed = directed;
    return g;
}

void graph_destroy(Graph* g, void (*free_data)(void*)) {
    if (!g) return;
    for (int i = 0; i < g->vertex_count; i++) {
        free(g->vertices[i].edges);
        if (free_data) free_data(g->vertices[i].data);
    }
    free(g->vertices);
    free(g);
}

int graph_add_vertex(Graph* g, void* data) {
    if (!g) return -1;
    if (g->vertex_count >= g->vertex_capacity) {
        int nc = g->vertex_capacity * 2;
        GVertex* nv = (GVertex*)realloc(g->vertices, sizeof(GVertex) * nc);
        if (!nv) return -1;
        memset(&nv[g->vertex_count], 0, sizeof(GVertex) * (nc - g->vertex_count));
        g->vertices = nv;
        g->vertex_capacity = nc;
    }
    int id = g->vertex_count++;
    g->vertices[id].id = id;
    g->vertices[id].data = data;
    g->vertices[id].edges = NULL;
    g->vertices[id].edge_count = 0;
    g->vertices[id].edge_capacity = 0;
    return id;
}

int graph_add_edge(Graph* g, int from, int to, int weight) {
    if (!g || from < 0 || from >= g->vertex_count || to < 0 || to >= g->vertex_count)
        return -1;
    GVertex* v = &g->vertices[from];
    if (v->edge_count >= v->edge_capacity) {
        int nc = v->edge_capacity == 0 ? 4 : v->edge_capacity * 2;
        GEdge* ne = (GEdge*)realloc(v->edges, sizeof(GEdge) * nc);
        if (!ne) return -1;
        v->edges = ne;
        v->edge_capacity = nc;
    }
    v->edges[v->edge_count].dest_id = to;
    v->edges[v->edge_count].weight = weight;
    v->edge_count++;
    if (!g->directed && from != to)
        return graph_add_edge(g, to, from, weight);
    return 0;
}

static void dfs_rec(Graph* g, int v, int* visited, void (*visit)(int, void*)) {
    visited[v] = 1;
    visit(v, g->vertices[v].data);
    for (int i = 0; i < g->vertices[v].edge_count; i++) {
        int d = g->vertices[v].edges[i].dest_id;
        if (!visited[d]) dfs_rec(g, d, visited, visit);
    }
}

void graph_bfs(Graph* g, int start, void (*visit)(int, void*)) {
    if (!g || start < 0 || start >= g->vertex_count || !visit) return;
    int* visited = (int*)calloc(g->vertex_count, sizeof(int));
    if (!visited) return;
    int* q = (int*)malloc(sizeof(int) * g->vertex_count);
    if (!q) { free(visited); return; }
    int f = 0, b = 0;
    visited[start] = 1;
    q[b++] = start;
    while (f < b) {
        int v = q[f++];
        visit(v, g->vertices[v].data);
        for (int i = 0; i < g->vertices[v].edge_count; i++) {
            int d = g->vertices[v].edges[i].dest_id;
            if (!visited[d]) { visited[d] = 1; q[b++] = d; }
        }
    }
    free(q);
    free(visited);
}

void graph_dfs(Graph* g, int start, void (*visit)(int, void*)) {
    if (!g || start < 0 || start >= g->vertex_count || !visit) return;
    int* visited = (int*)calloc(g->vertex_count, sizeof(int));
    if (!visited) return;
    dfs_rec(g, start, visited, visit);
    free(visited);
}

int graph_vertex_count(Graph* g) { return g ? g->vertex_count : 0; }
"
}

// ── Map (ordered BST) ────────────────────────────────────────────────────────

fn map_h() -> String {
    let g = guard("_map_h");
    format!("\
#ifndef {g}
#define {g}

typedef struct MapNode {{
    char* key;
    void* value;
    struct MapNode* left;
    struct MapNode* right;
}} MapNode;

typedef struct Map {{
    MapNode* root;
    int (*cmp)(const char*, const char*);
    int size;
}} Map;

Map*   map_create(int (*cmp)(const char*, const char*));
void   map_destroy(Map* m, void (*free_val)(void*));
int    map_put(Map* m, const char* key, void* value);
void*  map_get(Map* m, const char* key);
void*  map_remove(Map* m, const char* key);
int    map_has_key(Map* m, const char* key);
int    map_size(Map* m);

#endif /* {g} */
")
}

fn map_c() -> &'static str {
    "\
#include \"_map.h\"
#include <stdlib.h>
#include <string.h>

static MapNode* mn_create(const char* key, void* value) {
    MapNode* n = (MapNode*)malloc(sizeof(MapNode));
    if (!n) return NULL;
    n->key = strdup(key);
    n->value = value;
    n->left = n->right = NULL;
    return n;
}

Map* map_create(int (*cmp)(const char*, const char*)) {
    Map* m = (Map*)malloc(sizeof(Map));
    if (!m) return NULL;
    m->root = NULL;
    m->cmp = cmp ? cmp : (int (*)(const char*, const char*))strcmp;
    m->size = 0;
    return m;
}

static void map_destroy_sub(MapNode* n, void (*fv)(void*)) {
    if (!n) return;
    map_destroy_sub(n->left, fv);
    map_destroy_sub(n->right, fv);
    free(n->key);
    if (fv) fv(n->value);
    free(n);
}

void map_destroy(Map* m, void (*free_val)(void*)) {
    if (!m) return;
    map_destroy_sub(m->root, free_val);
    free(m);
}

int map_put(Map* m, const char* key, void* value) {
    if (!m || !key) return -1;
    MapNode* n = mn_create(key, value);
    if (!n) return -1;
    if (!m->root) { m->root = n; m->size++; return 0; }
    MapNode* cur = m->root;
    while (1) {
        int c = m->cmp(key, cur->key);
        if (c == 0) { free(n->key); free(n); cur->value = value; return 0; }
        if (c < 0) { if (!cur->left) { cur->left = n; break; } cur = cur->left; }
        else { if (!cur->right) { cur->right = n; break; } cur = cur->right; }
    }
    m->size++;
    return 0;
}

void* map_get(Map* m, const char* key) {
    if (!m || !key) return NULL;
    MapNode* cur = m->root;
    while (cur) {
        int c = m->cmp(key, cur->key);
        if (c == 0) return cur->value;
        cur = c < 0 ? cur->left : cur->right;
    }
    return NULL;
}

static MapNode* map_min(MapNode* n) {
    while (n && n->left) n = n->left;
    return n;
}

static MapNode* map_remove_sub(MapNode* n, const char* key, int (*cmp)(const char*, const char*),
                               void (*fv)(void*), int* removed) {
    if (!n) return NULL;
    int c = cmp(key, n->key);
    if (c < 0) n->left = map_remove_sub(n->left, key, cmp, fv, removed);
    else if (c > 0) n->right = map_remove_sub(n->right, key, cmp, fv, removed);
    else {
        *removed = 1;
        free(n->key);
        if (fv) fv(n->value);
        if (!n->left) { MapNode* r = n->right; free(n); return r; }
        if (!n->right) { MapNode* l = n->left; free(n); return l; }
        MapNode* m = map_min(n->right);
        n->key = strdup(m->key);
        n->value = m->value;
        n->right = map_remove_sub(n->right, m->key, cmp, NULL, &(int){0});
    }
    return n;
}

void* map_remove(Map* m, const char* key) {
    if (!m || !key) return NULL;
    void* old = map_get(m, key);
    if (!old) return NULL;
    int removed = 0;
    m->root = map_remove_sub(m->root, key, m->cmp, NULL, &removed);
    if (removed) m->size--;
    return old;
}

int map_has_key(Map* m, const char* key) { return map_get(m, key) != NULL; }
int map_size(Map* m) { return m ? m->size : 0; }
"
}

// ── Set (ordered BST) ────────────────────────────────────────────────────────

fn set_h() -> String {
    let g = guard("_set_h");
    format!("\
#ifndef {g}
#define {g}

typedef struct SetNode {{
    char* key;
    struct SetNode* left;
    struct SetNode* right;
}} SetNode;

typedef struct Set {{
    SetNode* root;
    int (*cmp)(const char*, const char*);
    int size;
}} Set;

Set* set_create(int (*cmp)(const char*, const char*));
void set_destroy(Set* s);
int  set_add(Set* s, const char* key);
int  set_contains(Set* s, const char* key);
int  set_remove(Set* s, const char* key);
int  set_size(Set* s);

#endif /* {g} */
")
}

fn set_c() -> &'static str {
    "\
#include \"_set.h\"
#include <stdlib.h>
#include <string.h>

static SetNode* sn_create(const char* key) {
    SetNode* n = (SetNode*)malloc(sizeof(SetNode));
    if (!n) return NULL;
    n->key = strdup(key);
    n->left = n->right = NULL;
    return n;
}

Set* set_create(int (*cmp)(const char*, const char*)) {
    Set* s = (Set*)malloc(sizeof(Set));
    if (!s) return NULL;
    s->root = NULL;
    s->cmp = cmp ? cmp : (int (*)(const char*, const char*))strcmp;
    s->size = 0;
    return s;
}

static void set_destroy_sub(SetNode* n) {
    if (!n) return;
    set_destroy_sub(n->left);
    set_destroy_sub(n->right);
    free(n->key);
    free(n);
}

void set_destroy(Set* s) {
    if (!s) return;
    set_destroy_sub(s->root);
    free(s);
}

int set_add(Set* s, const char* key) {
    if (!s || !key) return -1;
    SetNode* n = sn_create(key);
    if (!n) return -1;
    if (!s->root) { s->root = n; s->size++; return 0; }
    SetNode* cur = s->root;
    while (1) {
        int c = s->cmp(key, cur->key);
        if (c == 0) { free(n->key); free(n); return 0; }
        if (c < 0) { if (!cur->left) { cur->left = n; break; } cur = cur->left; }
        else { if (!cur->right) { cur->right = n; break; } cur = cur->right; }
    }
    s->size++;
    return 0;
}

int set_contains(Set* s, const char* key) {
    if (!s || !key) return 0;
    SetNode* cur = s->root;
    while (cur) {
        int c = s->cmp(key, cur->key);
        if (c == 0) return 1;
        cur = c < 0 ? cur->left : cur->right;
    }
    return 0;
}

static SetNode* set_min(SetNode* n) {
    while (n && n->left) n = n->left;
    return n;
}

static SetNode* set_remove_sub(SetNode* n, const char* key, int (*cmp)(const char*, const char*),
                               int* removed) {
    if (!n) return NULL;
    int c = cmp(key, n->key);
    if (c < 0) n->left = set_remove_sub(n->left, key, cmp, removed);
    else if (c > 0) n->right = set_remove_sub(n->right, key, cmp, removed);
    else {
        *removed = 1;
        free(n->key);
        if (!n->left) { SetNode* r = n->right; free(n); return r; }
        if (!n->right) { SetNode* l = n->left; free(n); return l; }
        SetNode* m = set_min(n->right);
        n->key = strdup(m->key);
        n->right = set_remove_sub(n->right, m->key, cmp, &(int){0});
    }
    return n;
}

int set_remove(Set* s, const char* key) {
    if (!s || !key || !set_contains(s, key)) return -1;
    int removed = 0;
    s->root = set_remove_sub(s->root, key, s->cmp, &removed);
    if (removed) s->size--;
    return 0;
}

int set_size(Set* s) { return s ? s->size : 0; }
"
}

// ── Rope ──────────────────────────────────────────────────────────────────────

fn rope_h() -> String {
    let g = guard("_rope_h");
    format!("\
#ifndef {g}
#define {g}

typedef struct RopeNode {{
    char* str;
    int   len;
    struct RopeNode* left;
    struct RopeNode* right;
}} RopeNode;

typedef struct Rope {{
    RopeNode* root;
    int total_len;
}} Rope;

Rope*    rope_create(const char* initial);
void     rope_destroy(Rope* r);
int      rope_insert(Rope* r, int index, const char* s);
int      rope_delete(Rope* r, int start, int end);
char*    rope_substr(Rope* r, int start, int end);
char     rope_char_at(Rope* r, int index);
int      rope_len(Rope* r);

#endif /* {g} */
")
}

fn rope_c() -> &'static str {
    "\
#include \"_rope.h\"
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

static RopeNode* rn_create(const char* s) {
    RopeNode* n = (RopeNode*)malloc(sizeof(RopeNode));
    if (!n) return NULL;
    n->str = s ? strdup(s) : strdup(\"\");
    n->len = s ? (int)strlen(s) : 0;
    n->left = n->right = NULL;
    return n;
}

Rope* rope_create(const char* initial) {
    Rope* r = (Rope*)malloc(sizeof(Rope));
    if (!r) return NULL;
    r->root = rn_create(initial ? initial : \"\");
    r->total_len = r->root ? r->root->len : 0;
    return r;
}

static void rn_destroy(RopeNode* n) {
    if (!n) return;
    rn_destroy(n->left);
    rn_destroy(n->right);
    free(n->str);
    free(n);
}

void rope_destroy(Rope* r) {
    if (!r) return;
    rn_destroy(r->root);
    free(r);
}

static char* rope_flatten(RopeNode* n) {
    if (!n) return strdup(\"\");
    if (!n->left && !n->right) return strdup(n->str);
    char* l = rope_flatten(n->left);
    char* r = rope_flatten(n->right);
    char* res = (char*)malloc(strlen(l) + strlen(r) + 1);
    if (res) { sprintf(res, \"%s%s\", l, r); }
    free(l); free(r);
    return res;
}

int rope_insert(Rope* r, int index, const char* s) {
    if (!r || !s || index < 0 || index > r->total_len) return -1;
    char* flat = rope_flatten(r->root);
    if (!flat) return -1;
    int flen = (int)strlen(flat);
    char* before = (char*)malloc(index + 1);
    char* after  = (char*)malloc(flen - index + 1);
    if (!before || !after) { free(flat); free(before); free(after); return -1; }
    strncpy(before, flat, index); before[index] = '\\0';
    strcpy(after, flat + index);
    char* new_flat = (char*)malloc(strlen(before) + strlen(s) + strlen(after) + 1);
    if (!new_flat) { free(flat); free(before); free(after); return -1; }
    strcpy(new_flat, before);
    strcat(new_flat, s);
    strcat(new_flat, after);
    rn_destroy(r->root);
    r->root = rn_create(new_flat);
    r->total_len = (int)strlen(new_flat);
    free(new_flat); free(flat); free(before); free(after);
    return 0;
}

int rope_delete(Rope* r, int start, int end) {
    if (!r || start < 0 || end > r->total_len || start >= end) return -1;
    char* flat = rope_flatten(r->root);
    if (!flat) return -1;
    int flen = (int)strlen(flat);
    char* before = (char*)malloc(start + 1);
    if (!before) { free(flat); return -1; }
    strncpy(before, flat, start); before[start] = '\\0';
    char* new_flat = (char*)malloc(strlen(before) + (flen - end) + 1);
    if (!new_flat) { free(flat); free(before); return -1; }
    strcpy(new_flat, before);
    strcat(new_flat, flat + end);
    rn_destroy(r->root);
    r->root = rn_create(new_flat);
    r->total_len = (int)strlen(new_flat);
    free(new_flat); free(flat); free(before);
    return 0;
}

char* rope_substr(Rope* r, int start, int end) {
    if (!r || start < 0 || end > r->total_len || start >= end) return NULL;
    char* flat = rope_flatten(r->root);
    if (!flat) return NULL;
    char* sub = (char*)malloc(end - start + 1);
    if (!sub) { free(flat); return NULL; }
    strncpy(sub, flat + start, end - start);
    sub[end - start] = '\\0';
    free(flat);
    return sub;
}

char rope_char_at(Rope* r, int index) {
    if (!r || index < 0 || index >= r->total_len) return '\\0';
    char* flat = rope_flatten(r->root);
    if (!flat) return '\\0';
    char c = flat[index];
    free(flat);
    return c;
}

int rope_len(Rope* r) { return r ? r->total_len : 0; }
"
}

// ── Trie ──────────────────────────────────────────────────────────────────────

fn trie_h() -> String {
    let g = guard("_trie_h");
    format!("\
#ifndef {g}
#define {g}

#define TRIE_ALPHABET 26

typedef struct TrieNode {{
    struct TrieNode* children[TRIE_ALPHABET];
    int is_end;
    int count;
}} TrieNode;

typedef struct Trie {{
    TrieNode* root;
    int size;
}} Trie;

Trie*    trie_create(void);
void     trie_destroy(Trie* t);
int      trie_insert(Trie* t, const char* word);
int      trie_search(Trie* t, const char* word);
int      trie_starts_with(Trie* t, const char* prefix);
int      trie_delete(Trie* t, const char* word);
int      trie_size(Trie* t);

#endif /* {g} */
")
}

fn trie_c() -> &'static str {
    "\
#include \"_trie.h\"
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

static TrieNode* tn_create(void) {
    TrieNode* n = (TrieNode*)calloc(1, sizeof(TrieNode));
    if (!n) return NULL;
    n->is_end = 0;
    n->count = 0;
    for (int i = 0; i < TRIE_ALPHABET; i++) n->children[i] = NULL;
    return n;
}

Trie* trie_create(void) {
    Trie* t = (Trie*)malloc(sizeof(Trie));
    if (!t) return NULL;
    t->root = tn_create();
    if (!t->root) { free(t); return NULL; }
    t->size = 0;
    return t;
}

static void tn_destroy(TrieNode* n) {
    if (!n) return;
    for (int i = 0; i < TRIE_ALPHABET; i++)
        tn_destroy(n->children[i]);
    free(n);
}

void trie_destroy(Trie* t) {
    if (!t) return;
    tn_destroy(t->root);
    free(t);
}

int trie_insert(Trie* t, const char* word) {
    if (!t || !word) return -1;
    TrieNode* cur = t->root;
    for (int i = 0; word[i]; i++) {
        int idx = tolower((unsigned char)word[i]) - 'a';
        if (idx < 0 || idx >= TRIE_ALPHABET) continue;
        if (!cur->children[idx]) {
            cur->children[idx] = tn_create();
            if (!cur->children[idx]) return -1;
        }
        cur = cur->children[idx];
        cur->count++;
    }
    if (!cur->is_end) { cur->is_end = 1; t->size++; }
    return 0;
}

int trie_search(Trie* t, const char* word) {
    if (!t || !word) return 0;
    TrieNode* cur = t->root;
    for (int i = 0; word[i]; i++) {
        int idx = tolower((unsigned char)word[i]) - 'a';
        if (idx < 0 || idx >= TRIE_ALPHABET || !cur->children[idx]) return 0;
        cur = cur->children[idx];
    }
    return cur->is_end;
}

int trie_starts_with(Trie* t, const char* prefix) {
    if (!t || !prefix) return 0;
    TrieNode* cur = t->root;
    for (int i = 0; prefix[i]; i++) {
        int idx = tolower((unsigned char)prefix[i]) - 'a';
        if (idx < 0 || idx >= TRIE_ALPHABET || !cur->children[idx]) return 0;
        cur = cur->children[idx];
    }
    return 1;
}

static int has_children(TrieNode* n) {
    for (int i = 0; i < TRIE_ALPHABET; i++)
        if (n->children[i]) return 1;
    return 0;
}

static int tn_delete(TrieNode* n, const char* word, int depth) {
    if (!n) return 0;
    if (word[depth] == '\\0') {
        if (n->is_end) { n->is_end = 0; return 1; }
        return 0;
    }
    int idx = tolower((unsigned char)word[depth]) - 'a';
    if (idx < 0 || idx >= TRIE_ALPHABET) return 0;
    if (tn_delete(n->children[idx], word, depth + 1)) {
        n->children[idx]->count--;
        if (n->children[idx]->count == 0 && !n->children[idx]->is_end && !has_children(n->children[idx])) {
            tn_destroy(n->children[idx]);
            n->children[idx] = NULL;
        }
        return 1;
    }
    return 0;
}

int trie_delete(Trie* t, const char* word) {
    if (!t || !word || !trie_search(t, word)) return -1;
    t->size--;
    tn_delete(t->root, word, 0);
    return 0;
}

int trie_size(Trie* t) { return t ? t->size : 0; }
"
}

// ── Linear Search ─────────────────────────────────────────────────────────────

fn lsearch_h() -> String {
    let g = guard("_lsearch_h");
    format!("\
#ifndef {g}
#define {g}

int lsearch_int(int* arr, int count, int key);
int lsearch_str(char** arr, int count, const char* key);
int lsearch(void** arr, int count, void* key,
            int (*cmp)(const void*, const void*));

#endif /* {g} */
")
}

fn lsearch_c() -> &'static str {
    "\
#include \"_lsearch.h\"
#include <string.h>

int lsearch_int(int* arr, int count, int key) {
    if (!arr) return -1;
    for (int i = 0; i < count; i++)
        if (arr[i] == key) return i;
    return -1;
}

int lsearch_str(char** arr, int count, const char* key) {
    if (!arr || !key) return -1;
    for (int i = 0; i < count; i++)
        if (arr[i] && strcmp(arr[i], key) == 0) return i;
    return -1;
}

int lsearch(void** arr, int count, void* key,
            int (*cmp)(const void*, const void*)) {
    if (!arr || !key || !cmp) return -1;
    for (int i = 0; i < count; i++)
        if (cmp(arr[i], key) == 0) return i;
    return -1;
}
"
}

// ── Binary Search ─────────────────────────────────────────────────────────────

fn bsearch_h() -> String {
    let g = guard("_bsearch_h");
    format!("\
#ifndef {g}
#define {g}

int bsearch_int(int* arr, int count, int key);
int bsearch_str(char** arr, int count, const char* key);
int bsearch_custom(void** arr, int count, void* key,
                   int (*cmp)(const void*, const void*));

#endif /* {g} */
")
}

fn bsearch_c() -> &'static str {
    "\
#include \"_bsearch.h\"
#include <string.h>

int bsearch_int(int* arr, int count, int key) {
    if (!arr) return -1;
    int lo = 0, hi = count - 1;
    while (lo <= hi) {
        int mid = lo + (hi - lo) / 2;
        if (arr[mid] == key) return mid;
        if (arr[mid] < key) lo = mid + 1;
        else hi = mid - 1;
    }
    return -1;
}

int bsearch_str(char** arr, int count, const char* key) {
    if (!arr || !key) return -1;
    int lo = 0, hi = count - 1;
    while (lo <= hi) {
        int mid = lo + (hi - lo) / 2;
        int c = strcmp(arr[mid], key);
        if (c == 0) return mid;
        if (c < 0) lo = mid + 1;
        else hi = mid - 1;
    }
    return -1;
}

int bsearch_custom(void** arr, int count, void* key,
                   int (*cmp)(const void*, const void*)) {
    if (!arr || !key || !cmp) return -1;
    int lo = 0, hi = count - 1;
    while (lo <= hi) {
        int mid = lo + (hi - lo) / 2;
        int c = cmp(arr[mid], key);
        if (c == 0) return mid;
        if (c < 0) lo = mid + 1;
        else hi = mid - 1;
    }
    return -1;
}
"
}

// ── Debug ─────────────────────────────────────────────────────────────────────

fn debug_h() -> String {
    let g = guard("debug_h");
    format!("\
#ifndef {g}
#define {g}

#include <stdio.h>

#define ASSERT_NOT_NULL(p)  do {{ if (!(p)) {{ fprintf(stderr, \"ASSERT FAILED: %s is NULL at %s:%d\\n\", #p, __FILE__, __LINE__); }} }} while(0)
#define LOG_ERROR(fmt, ...) fprintf(stderr, \"[ERROR] %s:%d: \" fmt \"\\n\", __FILE__, __LINE__, ##__VA_ARGS__)
#define LOG_WARN(fmt, ...)  fprintf(stdout, \"[WARN]  \" fmt \"\\n\", ##__VA_ARGS__)
#define LOG_INFO(fmt, ...)  fprintf(stdout, \"[INFO]  \" fmt \"\\n\", ##__VA_ARGS__)

void hex_dump(const void* data, int len);
void print_dsa_stats(const char* label, int size, int capacity);

#endif /* {g} */
")
}

fn debug_c() -> &'static str {
    "\
#include \"debug.h\"
#include <stdio.h>
#include <ctype.h>

void hex_dump(const void* data, int len) {
    const unsigned char* buf = (const unsigned char*)data;
    for (int i = 0; i < len; i += 16) {
        printf(\"%08x  \", i);
        for (int j = 0; j < 16; j++) {
            if (i + j < len) printf(\"%02x \", buf[i + j]);
            else printf(\"   \");
            if (j == 7) putchar(' ');
        }
        printf(\" \");
        for (int j = 0; j < 16 && i + j < len; j++)
            putchar(isprint(buf[i + j]) ? buf[i + j] : '.');
        putchar('\\n');
    }
}

void print_dsa_stats(const char* label, int size, int capacity) {
    printf(\"[DSA] %s: size=%d, capacity=%d\\n\", label, size, capacity);
}
"
}

// ── OS ────────────────────────────────────────────────────────────────────────

fn os_h() -> String {
    let g = guard("os_h");
    format!("\
#ifndef {g}
#define {g}

#include <stdlib.h>
#include <stdio.h>

void* os_malloc(size_t size);
void* os_calloc(size_t count, size_t size);
void* os_realloc(void* ptr, size_t size);
void  os_free(void* ptr);
char* os_file_read(const char* path);
int   os_file_write(const char* path, const char* content);
double os_timer_ms(void);

#endif /* {g} */
")
}

fn os_c() -> &'static str {
    "\
#include \"os.h\"
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#ifdef _WIN32
#include <windows.h>
#else
#include <sys/time.h>
#endif

void* os_malloc(size_t size) {
    void* p = malloc(size);
    if (!p) { fprintf(stderr, \"os_malloc(%zu) failed\\n\", size); exit(1); }
    return p;
}

void* os_calloc(size_t count, size_t size) {
    void* p = calloc(count, size);
    if (!p) { fprintf(stderr, \"os_calloc(%zu, %zu) failed\\n\", count, size); exit(1); }
    return p;
}

void* os_realloc(void* ptr, size_t size) {
    void* p = realloc(ptr, size);
    if (!p) { fprintf(stderr, \"os_realloc(%zu) failed\\n\", size); exit(1); }
    return p;
}

void os_free(void* ptr) { free(ptr); }

char* os_file_read(const char* path) {
    FILE* f = fopen(path, \"rb\");
    if (!f) return NULL;
    fseek(f, 0, SEEK_END);
    long sz = ftell(f);
    rewind(f);
    char* buf = (char*)malloc(sz + 1);
    if (!buf) { fclose(f); return NULL; }
    fread(buf, 1, sz, f);
    buf[sz] = '\\0';
    fclose(f);
    return buf;
}

int os_file_write(const char* path, const char* content) {
    FILE* f = fopen(path, \"w\");
    if (!f) return -1;
    if (content) fputs(content, f);
    fclose(f);
    return 0;
}

double os_timer_ms(void) {
#ifdef _WIN32
    LARGE_INTEGER freq, count;
    QueryPerformanceFrequency(&freq);
    QueryPerformanceCounter(&count);
    return (double)count.QuadPart / (double)freq.QuadPart * 1000.0;
#else
    struct timeval tv;
    gettimeofday(&tv, NULL);
    return (double)tv.tv_sec * 1000.0 + (double)tv.tv_usec / 1000.0;
#endif
}
"
}

// ── umbrella header/ntc.h ────────────────────────────────────────────────────

fn ntc_h() -> String {
    let g = guard("ntc_h");
    format!("\
#ifndef {g}
#define {g}

#define NTC_VERSION \"1.0.0\"

#include \"../dsa/_arraylist.h\"
#include \"../dsa/_linkedlist.h\"
#include \"../dsa/_doublylinkedlist.h\"
#include \"../dsa/_circularlinkedlist.h\"
#include \"../dsa/_stack.h\"
#include \"../dsa/_queue.h\"
#include \"../dsa/_binarytree.h\"
#include \"../dsa/_tree.h\"
#include \"../dsa/_hashmap.h\"
#include \"../dsa/_hashtable.h\"
#include \"../dsa/_hashset.h\"
#include \"../dsa/_hash.h\"
#include \"../dsa/_heap.h\"
#include \"../dsa/_priorityqueue.h\"
#include \"../dsa/_graph.h\"
#include \"../dsa/_map.h\"
#include \"../dsa/_set.h\"
#include \"../dsa/_rope.h\"
#include \"../dsa/_trie.h\"
#include \"../dsa/_lsearch.h\"
#include \"../dsa/_bsearch.h\"
#include \"../debug/debug.h\"
#include \"../os/os.h\"
#include \"../ntclib/ntc_lib.h\"

#endif /* {g} */
")
}

// ── NtcLib ────────────────────────────────────────────────────────────────────

fn ntclib_h() -> String {
    let g = guard("ntc_lib_h");
    format!("\
#ifndef {g}
#define {g}

char*  ntc_strdup(const char* s);
char*  ntc_trim(char* s);
char** ntc_split(const char* s, const char* delim, int* count);
char*  ntc_join(char** parts, int count, const char* glue);
void   ntc_sort(void** arr, int count, int (*cmp)(const void*, const void*));
int    ntc_compare_int(const void* a, const void* b);
int    ntc_compare_str(const void* a, const void* b);

#endif /* {g} */
")
}

fn ntclib_c() -> &'static str {
    "\
#include \"ntc_lib.h\"
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

char* ntc_strdup(const char* s) {
    if (!s) return NULL;
    size_t len = strlen(s);
    char* d = (char*)malloc(len + 1);
    if (!d) return NULL;
    memcpy(d, s, len + 1);
    return d;
}

char* ntc_trim(char* s) {
    if (!s) return NULL;
    char* end;
    while (isspace((unsigned char)*s)) s++;
    if (*s == 0) return s;
    end = s + strlen(s) - 1;
    while (end > s && isspace((unsigned char)*end)) end--;
    *(end + 1) = '\\0';
    return s;
}

char** ntc_split(const char* s, const char* delim, int* count) {
    if (!s || !delim || !count) return NULL;
    int cap = 16, idx = 0;
    char** res = (char**)malloc(sizeof(char*) * cap);
    if (!res) return NULL;
    char* copy = ntc_strdup(s);
    if (!copy) { free(res); return NULL; }
    char* token = strtok(copy, delim);
    while (token) {
        if (idx >= cap) {
            cap *= 2;
            char** nr = (char**)realloc(res, sizeof(char*) * cap);
            if (!nr) { free(copy); free(res); return NULL; }
            res = nr;
        }
        res[idx++] = ntc_strdup(token);
        token = strtok(NULL, delim);
    }
    *count = idx;
    free(copy);
    return res;
}

char* ntc_join(char** parts, int count, const char* glue) {
    if (!parts || count <= 0) return NULL;
    size_t total = 0;
    for (int i = 0; i < count; i++) total += strlen(parts[i]);
    total += (count - 1) * strlen(glue) + 1;
    char* res = (char*)malloc(total);
    if (!res) return NULL;
    res[0] = '\\0';
    for (int i = 0; i < count; i++) {
        if (i > 0) strcat(res, glue);
        strcat(res, parts[i]);
    }
    return res;
}

// ── Timsort (insertion sort + merge sort hybrid) ──────────────────────────────

#define TIMSORT_MIN_MERGE 32

static int timsort_min_run(int n) {
    int r = 0;
    while (n >= TIMSORT_MIN_MERGE) { r |= n & 1; n >>= 1; }
    return n + r;
}

static void timsort_insertion_sort(void** arr, int left, int right,
                                    int (*cmp)(const void*, const void*)) {
    for (int i = left + 1; i <= right; i++) {
        void* key = arr[i];
        int j = i - 1;
        while (j >= left && cmp(arr[j], key) > 0) { arr[j + 1] = arr[j]; j--; }
        arr[j + 1] = key;
    }
}

static void timsort_merge(void** arr, int left, int mid, int right,
                          int (*cmp)(const void*, const void*)) {
    int len1 = mid - left + 1, len2 = right - mid;
    void** left_arr = (void**)malloc(len1 * sizeof(void*));
    void** right_arr = (void**)malloc(len2 * sizeof(void*));
    if (!left_arr || !right_arr) { free(left_arr); free(right_arr); return; }
    for (int i = 0; i < len1; i++) left_arr[i] = arr[left + i];
    for (int i = 0; i < len2; i++) right_arr[i] = arr[mid + 1 + i];
    int i = 0, j = 0, k = left;
    while (i < len1 && j < len2)
        arr[k++] = cmp(left_arr[i], right_arr[j]) <= 0 ? left_arr[i++] : right_arr[j++];
    while (i < len1) arr[k++] = left_arr[i++];
    while (j < len2) arr[k++] = right_arr[j++];
    free(left_arr);
    free(right_arr);
}

void ntc_sort(void** arr, int count, int (*cmp)(const void*, const void*)) {
    if (!arr || count <= 1 || !cmp) return;
    int min_run = timsort_min_run(count);
    for (int i = 0; i < count; i += min_run)
        timsort_insertion_sort(arr, i, (i + min_run - 1 < count - 1) ? i + min_run - 1 : count - 1, cmp);
    for (int size = min_run; size < count; size *= 2) {
        for (int left = 0; left < count; left += 2 * size) {
            int mid = left + size - 1;
            int right = (left + 2 * size - 1 < count - 1) ? left + 2 * size - 1 : count - 1;
            if (mid < right) timsort_merge(arr, left, mid, right, cmp);
        }
    }
}

int ntc_compare_int(const void* a, const void* b) {
    int ia = *(const int*)a, ib = *(const int*)b;
    return (ia > ib) - (ia < ib);
}

int ntc_compare_str(const void* a, const void* b) {
    return strcmp(*(const char**)a, *(const char**)b);
}
"
}

// ── ntc_main.c ────────────────────────────────────────────────────────────────

fn ntc_main_c(name: &str) -> String {
    format!("\
#include \"header/ntc.h\"
#include <stdio.h>
#include <string.h>

static void print_int_data(void* data) {{
    printf(\"%d \", *(int*)data);
}}

static void print_graph_vertex(int id, void* data) {{
    (void)id;
    printf(\"%s \", (char*)data);
}}

int main(void) {{
    printf(\"=== {name} — Hyper Project Demo ===\\n\\n\");

    // ── ArrayList Demo ──
    printf(\"1. ArrayList:\\n\");
    ArrayList* list = arraylist_create(4);
    int nums[] = {{10, 20, 30, 40, 50}};
    for (int i = 0; i < 5; i++)
        arraylist_append(list, &nums[i]);
    printf(\"   Size: %d, Capacity: %d\\n\", arraylist_size(list), arraylist_capacity(list));
    for (int i = 0; i < arraylist_size(list); i++)
        printf(\"   list[%d] = %d\\n\", i, *(int*)arraylist_get(list, i));
    arraylist_destroy(list, NULL);

    // ── LinkedList Demo ──
    printf(\"\\n2. LinkedList:\\n\");
    LinkedList* ll = linkedlist_create();
    linkedlist_push_front(ll, &nums[0]);
    linkedlist_push_back(ll, &nums[1]);
    linkedlist_push_back(ll, &nums[2]);
    printf(\"   Front: %d, Size: %d\\n\", *(int*)linkedlist_peek_front(ll), linkedlist_size(ll));
    // Traverse and print
    LLNode* cur = ll->head;
    printf(\"   Elements: \");
    while (cur) {{ printf(\"%d \", *(int*)cur->data); cur = cur->next; }}
    printf(\"\\n\");
    linkedlist_destroy(ll, NULL);

    // ── Stack Demo ──
    printf(\"\\n3. Stack:\\n\");
    Stack* st = stack_create(4);
    for (int i = 0; i < 4; i++) stack_push(st, &nums[i]);
    printf(\"   Popped: %d %d %d\\n\", *(int*)stack_pop(st), *(int*)stack_pop(st), *(int*)stack_pop(st));
    stack_destroy(st, NULL);

    // ── Queue Demo ──
    printf(\"\\n4. Queue:\\n\");
    Queue* q = queue_create(4);
    for (int i = 0; i < 4; i++) queue_enqueue(q, &nums[i]);
    printf(\"   Dequeued: %d %d %d\\n\", *(int*)queue_dequeue(q), *(int*)queue_dequeue(q), *(int*)queue_dequeue(q));
    queue_destroy(q, NULL);

    // ── BinaryTree Demo ──
    printf(\"\\n5. BinaryTree:\\n\");
    BinaryTree* bt = binarytree_create(ntc_compare_int);
    int btv[] = {{50, 30, 70, 20, 40, 60, 80}};
    for (int i = 0; i < 7; i++) binarytree_insert(bt, &btv[i]);
    printf(\"   Inorder: \");
    binarytree_inorder(bt, print_int_data);
    printf(\"\\n   Size: %d\\n\", binarytree_size(bt));
    binarytree_destroy(bt, NULL);

    // ── HashMap Demo ──
    printf(\"\\n6. HashMap:\\n\");
    HashMap* hm = hashmap_create(16);
    int h1 = 100, h2 = 200, h3 = 300;
    hashmap_put(hm, \"alpha\", &h1);
    hashmap_put(hm, \"beta\", &h2);
    hashmap_put(hm, \"gamma\", &h3);
    printf(\"   alpha = %d\\n\", *(int*)hashmap_get(hm, \"alpha\"));
    printf(\"   Has 'beta': %s\\n\", hashmap_has_key(hm, \"beta\") ? \"yes\" : \"no\");
    printf(\"   Size: %d\\n\", hashmap_size(hm));
    int kc = 0;
    char** ks = hashmap_keys(hm, &kc);
    printf(\"   Keys: \");
    for (int i = 0; i < kc; i++) {{ printf(\"%s \", ks[i]); free(ks[i]); }}
    printf(\"\\n\");
    free(ks);
    hashmap_destroy(hm, NULL);

    // ── Graph Demo ──
    printf(\"\\n7. Graph (BFS):\\n\");
    Graph* g = graph_create(8, 1);
    char* labels[] = {{ (char*)\"A\", (char*)\"B\", (char*)\"C\", (char*)\"D\", (char*)\"E\" }};
    int gids[5];
    for (int i = 0; i < 5; i++) gids[i] = graph_add_vertex(g, labels[i]);
    graph_add_edge(g, gids[0], gids[1], 1);
    graph_add_edge(g, gids[0], gids[2], 1);
    graph_add_edge(g, gids[1], gids[3], 1);
    graph_add_edge(g, gids[2], gids[4], 1);
    printf(\"   BFS from A: \");
    graph_bfs(g, gids[0], print_graph_vertex);
    printf(\"\\n   Vertices: %d\\n\", graph_vertex_count(g));
    graph_destroy(g, NULL);

    // ── OS timer Demo ──
    printf(\"\\n8. OS Timer:\\n\");
    double t1 = os_timer_ms();
    int s = 0;
    for (int i = 0; i < 1000000; i++) s += i;
    double t2 = os_timer_ms();
    printf(\"   Loop took: %.2f ms (sum=%d)\\n\", t2 - t1, s);

    // ── Linear Search + Binary Search Demo ──
    printf(\"\\n9. Search Algorithms:\\n\");
    int search_arr[] = {{7, 12, 19, 3, 88, 55, 42, 31, 64, 28}};
    int search_n = 10;
    int lkey = 55;
    int li = lsearch_int(search_arr, search_n, lkey);
    printf(\"   lsearch_int(%d) = %s\\n\", lkey, li >= 0 ? \"found\" : \"not found\");
    if (li >= 0) printf(\"     at index %d\\n\", li);

    // Sort for binary search (using timsort)
    void* sort_ptrs[10];
    for (int i = 0; i < search_n; i++) sort_ptrs[i] = &search_arr[i];
    ntc_sort(sort_ptrs, search_n, ntc_compare_int);
    int sorted[10];
    for (int i = 0; i < search_n; i++) sorted[i] = *(int*)sort_ptrs[i];
    printf(\"   Sorted: \");
    for (int i = 0; i < search_n; i++) printf(\"%d \", sorted[i]);
    printf(\"\\n\");
    int bkey = 42;
    int bi = bsearch_int(sorted, search_n, bkey);
    printf(\"   bsearch_int(%d) = %s\\n\", bkey, bi >= 0 ? \"found\" : \"not found\");
    if (bi >= 0) printf(\"     at index %d\\n\", bi);

    // String search demo
    char* words[] = {{ (char*)\"cherry\", (char*)\"apple\", (char*)\"banana\", (char*)\"date\", (char*)\"elderberry\" }};
    int wn = 5;
    int wli = lsearch_str(words, wn, \"banana\");
    printf(\"   lsearch_str('banana') = %s\\n\", wli >= 0 ? \"found\" : \"not found\");
    void* wptrs[5];
    for (int i = 0; i < wn; i++) wptrs[i] = words[i];
    ntc_sort(wptrs, wn, ntc_compare_str);
    char* sorted_words[5];
    for (int i = 0; i < wn; i++) sorted_words[i] = (char*)wptrs[i];
    int wbi = bsearch_str(sorted_words, wn, \"date\");
    printf(\"   bsearch_str('date') = %s\\n\", wbi >= 0 ? \"found\" : \"not found\");

    printf(\"\\n=== Demo Complete ===\\n\");
    return 0;
}}
")
}

// ── ntc_lib.c ─────────────────────────────────────────────────────────────────

fn ntc_lib_c() -> &'static str {
    "\
#include \"header/ntc.h\"
#include <stdio.h>
#include <string.h>

int main(void) {
    printf(\"=== NtcLib Demo ===\\n\\n\");

    // ntc_split / ntc_join
    const char* csv = \"apple,banana,cherry,date\";
    int count = 0;
    char** parts = ntc_split(csv, \",\", &count);
    printf(\"Split '%s' into %d parts:\\n\", csv, count);
    for (int i = 0; i < count; i++)
        printf(\"  [%d] %s\\n\", i, parts[i]);

    char* joined = ntc_join(parts, count, \" <-> \");
    printf(\"Joined: %s\\n\\n\", joined);

    // ntc_trim
    char messy[] = \"  hello world  \\t\\n\";
    printf(\"Trim: '%s' -> '%s'\\n\\n\", messy, ntc_trim(messy));

    // ntc_sort (int)
    int arr[] = {42, 7, 19, 3, 88, 55, 12};
    int n = sizeof(arr) / sizeof(arr[0]);
    void* ptrs[7];
    for (int i = 0; i < n; i++) ptrs[i] = &arr[i];
    ntc_sort(ptrs, n, ntc_compare_int);
    printf(\"Sorted ints: \");
    for (int i = 0; i < n; i++) printf(\"%d \", *(int*)ptrs[i]);
    printf(\"\\n\\n\");

    // ntc_sort (strings)
    const char* words[] = {\"zebra\", \"apple\", \"monkey\", \"banana\", \"coral\"};
    n = 5;
    void* str_ptrs[5];
    for (int i = 0; i < n; i++) str_ptrs[i] = (void*)words[i];
    ntc_sort(str_ptrs, n, ntc_compare_str);
    printf(\"Sorted words: \");
    for (int i = 0; i < n; i++) printf(\"%s \", (const char*)str_ptrs[i]);
    printf(\"\\n\\n\");

    // Memory & file I/O
    int* mem = (int*)os_malloc(10 * sizeof(int));
    for (int i = 0; i < 10; i++) mem[i] = i * i;
    printf(\"os_malloc: last element = %d\\n\", mem[9]);
    os_free(mem);

    os_file_write(\"demo_output.txt\", \"Hello from ntc_lib!\\n\");
    char* content = os_file_read(\"demo_output.txt\");
    if (content) { printf(\"Read file: %s\", content); free(content); }

    // Cleanup
    for (int i = 0; i < count; i++) free(parts[i]);
    free(parts);
    free(joined);

    printf(\"\\n=== NtcLib Demo Complete ===\\n\");
    return 0;
}
"
}

// ── NTCRANFILE.toml ──────────────────────────────────────────────────────────

fn ntc_ranfile_toml(name: &str) -> String {
    format!("\
# NTCRANFILE — ntc task runner configuration for {name}
#   ran build          compile main demo
#   ran run            build + run main demo
#   ran build_lib      compile the ntc_lib example separately
#   ran run_lib        build + run ntc_lib example
#   ran clean          remove build artifacts

[vars]
CC    = \"gcc\"
FLAGS = \"-Wall -Wextra -O2 -g\"

[targets.build]
deps = []
cmd = \"$(CC) $(FLAGS) -o {name} ntc_main.c dsa/*.c debug/debug.c os/os.c ntclib/ntc_lib.c\"

[targets.run]
deps = [\"build\"]
cmd = \"./{name}\"

[targets.build_lib]
deps = []
cmd = \"$(CC) $(FLAGS) -o {name}_lib ntc_lib.c dsa/*.c debug/debug.c os/os.c ntclib/ntc_lib.c\"

[targets.run_lib]
deps = [\"build_lib\"]
cmd = \"./{name}_lib\"

[targets.clean]
deps = []
cmd = \"rm -f {name} {name}_lib *.o demo_output.txt\"
")
}
