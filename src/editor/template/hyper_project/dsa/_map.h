typedef struct MapNode {
    char* key;
    void* value;
    struct MapNode* left;
    struct MapNode* right;
} MapNode;

typedef struct Map {
    MapNode* root;
    int (*cmp)(const char*, const char*);
    int size;
} Map;

Map*   map_create(int (*cmp)(const char*, const char*));
void   map_destroy(Map* m, void (*free_val)(void*));
int    map_put(Map* m, const char* key, void* value);
void*  map_get(Map* m, const char* key);
void*  map_remove(Map* m, const char* key);
int    map_has_key(Map* m, const char* key);
int    map_size(Map* m);