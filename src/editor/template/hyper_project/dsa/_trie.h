#define TRIE_ALPHABET 26

typedef struct TrieNode {
    struct TrieNode* children[TRIE_ALPHABET];
    int is_end;
    int count;
} TrieNode;

typedef struct Trie {
    TrieNode* root;
    int size;
} Trie;

Trie*    trie_create(void);
void     trie_destroy(Trie* t);
int      trie_insert(Trie* t, const char* word);
int      trie_search(Trie* t, const char* word);
int      trie_starts_with(Trie* t, const char* prefix);
int      trie_delete(Trie* t, const char* word);
int      trie_size(Trie* t);