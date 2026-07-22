#include "ntc_lib.h"
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

char* ntc_strdup(const char* s) {
if (!s) return NULL;
size_t len = strlen(s);
char* d = (char*)malloc(len + 1);
if (!d) return NULL;
memcpy(d, s, len + 1);
return d;
}

char* ntc_trim(char* s) {
if (!s) return NULL;
char* end;
while (isspace((unsigned char)*s)) s++;
if (*s == 0) return s;
end = s + strlen(s) - 1;
while (end > s && isspace((unsigned char)*end)) end--;
*(end + 1) = '\0';
return s;
}

char** ntc_split(const char* s, const char* delim, int* count) {
if (!s || !delim || !count) return NULL;
int cap = 16, idx = 0;
char** res = (char**)malloc(sizeof(char*) * cap);
if (!res) return NULL;
char* copy = ntc_strdup(s);
if (!copy) { free(res); return NULL; }
char* token = strtok(copy, delim);
while (token) {
if (idx >= cap) {
cap *= 2;
char** nr = (char**)realloc(res, sizeof(char*) * cap);
if (!nr) { free(copy); free(res); return NULL; }
res = nr;
}
res[idx++] = ntc_strdup(token);
token = strtok(NULL, delim);
}
*count = idx;
free(copy);
return res;
}

char* ntc_join(char** parts, int count, const char* glue) {
if (!parts || count <= 0) return NULL;
size_t total = 0;
for (int i = 0; i < count; i++) total += strlen(parts[i]);
total += (count - 1) * strlen(glue) + 1;
char* res = (char*)malloc(total);
if (!res) return NULL;
res[0] = '\0';
for (int i = 0; i < count; i++) {
if (i > 0) strcat(res, glue);
strcat(res, parts[i]);
}
return res;
}

// ── Timsort (insertion sort + merge sort hybrid) ──────────────────────────────

#define TIMSORT_MIN_MERGE 32

static int timsort_min_run(int n) {
int r = 0;
while (n >= TIMSORT_MIN_MERGE) { r |= n & 1; n >>= 1; }
return n + r;
}

static void timsort_insertion_sort(void** arr, int left, int right,
int (*cmp)(const void*, const void*)) {
for (int i = left + 1; i <= right; i++) {
void* key = arr[i];
int j = i - 1;
while (j >= left && cmp(arr[j], key) > 0) { arr[j + 1] = arr[j]; j--; }
arr[j + 1] = key;
}
}

static void timsort_merge(void** arr, int left, int mid, int right,
int (*cmp)(const void*, const void*)) {
int len1 = mid - left + 1, len2 = right - mid;
void** left_arr = (void**)malloc(len1 * sizeof(void*));
void** right_arr = (void**)malloc(len2 * sizeof(void*));
if (!left_arr || !right_arr) { free(left_arr); free(right_arr); return; }
for (int i = 0; i < len1; i++) left_arr[i] = arr[left + i];
for (int i = 0; i < len2; i++) right_arr[i] = arr[mid + 1 + i];
int i = 0, j = 0, k = left;
while (i < len1 && j < len2)
arr[k++] = cmp(left_arr[i], right_arr[j]) <= 0 ? left_arr[i++] : right_arr[j++];
while (i < len1) arr[k++] = left_arr[i++];
while (j < len2) arr[k++] = right_arr[j++];
free(left_arr);
free(right_arr);
}

void ntc_sort(void** arr, int count, int (*cmp)(const void*, const void*)) {
if (!arr || count <= 1 || !cmp) return;
int min_run = timsort_min_run(count);
for (int i = 0; i < count; i += min_run)
timsort_insertion_sort(arr, i, (i + min_run - 1 < count - 1) ? i + min_run - 1 : count - 1, cmp);
for (int size = min_run; size < count; size *= 2) {
for (int left = 0; left < count; left += 2 * size) {
int mid = left + size - 1;
int right = (left + 2 * size - 1 < count - 1) ? left + 2 * size - 1 : count - 1;
if (mid < right) timsort_merge(arr, left, mid, right, cmp);
}
}
}

int ntc_compare_int(const void* a, const void* b) {
int ia = *(const int*)a, ib = *(const int*)b;
return (ia > ib) - (ia < ib);
}

int ntc_compare_str(const void* a, const void* b) {
return strcmp(*(const char**)a, *(const char**)b);
}