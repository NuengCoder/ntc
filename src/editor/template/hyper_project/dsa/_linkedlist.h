typedef struct LLNode {
    void* data;
    struct LLNode* next;
} LLNode;

typedef struct LinkedList {
    LLNode* head;
    int size;
} LinkedList;

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