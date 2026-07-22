#include "_tree.h"
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