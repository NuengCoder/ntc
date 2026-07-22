typedef struct RopeNode {
    char* str;
    int   len;
    struct RopeNode* left;
    struct RopeNode* right;
} RopeNode;

typedef struct Rope {
    RopeNode* root;
    int total_len;
} Rope;

Rope*    rope_create(const char* initial);
void     rope_destroy(Rope* r);
int      rope_insert(Rope* r, int index, const char* s);
int      rope_delete(Rope* r, int start, int end);
char*    rope_substr(Rope* r, int start, int end);
char     rope_char_at(Rope* r, int index);
int      rope_len(Rope* r);