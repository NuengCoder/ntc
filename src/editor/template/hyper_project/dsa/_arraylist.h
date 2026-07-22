typedef struct ArrayList {
    void** data;
    int capacity;
    int size;
} ArrayList;

ArrayList* arraylist_create(int initial_capacity);
void      arraylist_destroy(ArrayList* list, void (*free_elem)(void*));
int       arraylist_append(ArrayList* list, void* elem);
int       arraylist_insert(ArrayList* list, int index, void* elem);
void*     arraylist_get(ArrayList* list, int index);
int       arraylist_set(ArrayList* list, int index, void* elem);
void*     arraylist_remove(ArrayList* list, int index);
int       arraylist_size(ArrayList* list);
int       arraylist_capacity(ArrayList* list);