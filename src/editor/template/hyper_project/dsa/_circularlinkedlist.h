typedef struct CLNode {
    void* data;
    struct CLNode* next;
} CLNode;

typedef struct CircularLinkedList {
    CLNode* tail;
    int size;
} CircularLinkedList;

CircularLinkedList* circularlinkedlist_create(void);
void                circularlinkedlist_destroy(CircularLinkedList* list, void (*free_elem)(void*));
int                 circularlinkedlist_push_front(CircularLinkedList* list, void* data);
int                 circularlinkedlist_push_back(CircularLinkedList* list, void* data);
void*               circularlinkedlist_pop_front(CircularLinkedList* list);
void*               circularlinkedlist_pop_back(CircularLinkedList* list);
void*               circularlinkedlist_peek_front(CircularLinkedList* list);
void*               circularlinkedlist_peek_back(CircularLinkedList* list);
int                 circularlinkedlist_size(CircularLinkedList* list);