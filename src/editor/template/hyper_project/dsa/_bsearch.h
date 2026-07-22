int bsearch_int(int* arr, int count, int key);
int bsearch_str(char** arr, int count, const char* key);
int bsearch_custom(void** arr, int count, void* key,
                   int (*cmp)(const void*, const void*));