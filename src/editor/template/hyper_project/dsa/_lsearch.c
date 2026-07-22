#include "_lsearch.h"
#include <string.h>

int lsearch_int(int* arr, int count, int key) {
if (!arr) return -1;
for (int i = 0; i < count; i++)
if (arr[i] == key) return i;
return -1;
}

int lsearch_str(char** arr, int count, const char* key) {
if (!arr || !key) return -1;
for (int i = 0; i < count; i++)
if (arr[i] && strcmp(arr[i], key) == 0) return i;
return -1;
}

int lsearch(void** arr, int count, void* key,
int (*cmp)(const void*, const void*)) {
if (!arr || !key || !cmp) return -1;
for (int i = 0; i < count; i++)
if (cmp(arr[i], key) == 0) return i;
return -1;
}