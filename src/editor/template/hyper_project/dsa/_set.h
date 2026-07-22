typedef struct SetNode {
    char* key;
    struct SetNode* left;
    struct SetNode* right;
} SetNode;

typedef struct Set {
    SetNode* root;
    int (*cmp)(const char*, const char*);
    int size;
} Set;

Set* set_create(int (*cmp)(const char*, const char*));
void set_destroy(Set* s);
int  set_add(Set* s, const char* key);
int  set_contains(Set* s, const char* key);
int  set_remove(Set* s, const char* key);
int  set_size(Set* s);