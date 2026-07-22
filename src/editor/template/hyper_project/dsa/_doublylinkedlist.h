typedef struct DLNode {
    void* data;
    struct DLNode* prev;
    struct DLNode* next;
} DLNode;

typedef struct DoublyLinkedList {
    DLNode* head;
    DLNode* tail;
    int size;
} DoublyLinkedList;

DoublyLinkedList* doublylinkedlist_create(void);
void              doublylinkedlist_destroy(DoublyLinkedList* list, void (*free_elem)(void*));
int               doublylinkedlist_push_front(DoublyLinkedList* list, void* data);
int               doublylinkedlist_push_back(DoublyLinkedList* list, void* data);
void*             doublylinkedlist_pop_front(DoublyLinkedList* list);
void*             doublylinkedlist_pop_back(DoublyLinkedList* list);
void*             doublylinkedlist_peek_front(DoublyLinkedList* list);
void*             doublylinkedlist_peek_back(DoublyLinkedList* list);
int               doublylinkedlist_size(DoublyLinkedList* list);