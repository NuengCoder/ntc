#include "_binarytree.h"
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