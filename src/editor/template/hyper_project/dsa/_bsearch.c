#include "_bsearch.h"
#include <string.h>

int bsearch_int(int* arr, int count, int key) {
if (!arr) return -1;
int lo = 0, hi = count - 1;
while (lo <= hi) {
int mid = lo + (hi - lo) / 2;
if (arr[mid] == key) return mid;
if (arr[mid] < key) lo = mid + 1;
else hi = mid - 1;
}
return -1;
}

int bsearch_str(char** arr, int count, const char* key) {
if (!arr || !key) return -1;
int lo = 0, hi = count - 1;
while (lo <= hi) {
int mid = lo + (hi - lo) / 2;
int c = strcmp(arr[mid], key);
if (c == 0) return mid;
if (c < 0) lo = mid + 1;
else hi = mid - 1;
}
return -1;
}

int bsearch_custom(void** arr, int count, void* key,
int (*cmp)(const void*, const void*)) {
if (!arr || !key || !cmp) return -1;
int lo = 0, hi = count - 1;
while (lo <= hi) {
int mid = lo + (hi - lo) / 2;
int c = cmp(arr[mid], key);
if (c == 0) return mid;
if (c < 0) lo = mid + 1;
else hi = mid - 1;
}
return -1;
}