typedef struct BTNode {
    void* data;
    struct BTNode* left;
    struct BTNode* right;
} BTNode;

typedef struct BinaryTree {
    BTNode* root;
    int (*cmp)(const void*, const void*);
    int size;
} BinaryTree;

BinaryTree* binarytree_create(int (*cmp)(const void*, const void*));
void        binarytree_destroy(BinaryTree* tree, void (*free_elem)(void*));
int         binarytree_insert(BinaryTree* tree, void* data);
BTNode*     binarytree_search(BinaryTree* tree, void* data);
int         binarytree_delete(BinaryTree* tree, void* data, void (*free_elem)(void*));
void        binarytree_inorder(BinaryTree* tree, void (*visit)(void*));
void        binarytree_preorder(BinaryTree* tree, void (*visit)(void*));
void        binarytree_postorder(BinaryTree* tree, void (*visit)(void*));
int         binarytree_size(BinaryTree* tree);