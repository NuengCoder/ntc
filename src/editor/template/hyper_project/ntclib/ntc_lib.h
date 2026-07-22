char*  ntc_strdup(const char* s);
char*  ntc_trim(char* s);
char** ntc_split(const char* s, const char* delim, int* count);
char*  ntc_join(char** parts, int count, const char* glue);
void   ntc_sort(void** arr, int count, int (*cmp)(const void*, const void*));
int    ntc_compare_int(const void* a, const void* b);
int    ntc_compare_str(const void* a, const void* b);