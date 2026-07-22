int lsearch_int(int* arr, int count, int key);
int lsearch_str(char** arr, int count, const char* key);
int lsearch(void** arr, int count, void* key,
            int (*cmp)(const void*, const void*));