typedef struct TreeNode {
    void* data;
    struct TreeNode** children;
    int child_count;
    int child_capacity;
} TreeNode;

typedef struct Tree {
    TreeNode* root;
    int size;
} Tree;

Tree*     tree_create(void);
void      tree_destroy(Tree* tree, void (*free_elem)(void*));
TreeNode* tree_add_child(Tree* tree, TreeNode* parent, void* data);
void      tree_traverse_dfs(Tree* tree, void (*visit)(void*));
void      tree_traverse_bfs(Tree* tree, void (*visit)(void*));
int       tree_size(Tree* tree);