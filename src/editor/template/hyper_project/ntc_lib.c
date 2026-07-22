#include "header/ntc.h"
#include <stdio.h>
#include <string.h>

int main(void) {
printf("=== NtcLib Demo ===\n\n");

// ntc_split / ntc_join
const char* csv = "apple,banana,cherry,date";
int count = 0;
char** parts = ntc_split(csv, ",", &count);
printf("Split '%s' into %d parts:\n", csv, count);
for (int i = 0; i < count; i++)
printf("  [%d] %s\n", i, parts[i]);

char* joined = ntc_join(parts, count, " <-> ");
printf("Joined: %s\n\n", joined);

// ntc_trim
char messy[] = "  hello world  	\n";
printf("Trim: '%s' -> '%s'\n\n", messy, ntc_trim(messy));

// ntc_sort (int)
int arr[] = {42, 7, 19, 3, 88, 55, 12};
int n = sizeof(arr) / sizeof(arr[0]);
void* ptrs[7];
for (int i = 0; i < n; i++) ptrs[i] = &arr[i];
ntc_sort(ptrs, n, ntc_compare_int);
printf("Sorted ints: ");
for (int i = 0; i < n; i++) printf("%d ", *(int*)ptrs[i]);
printf("\n\n");

// ntc_sort (strings)
const char* words[] = {"zebra", "apple", "monkey", "banana", "coral"};
n = 5;
void* str_ptrs[5];
for (int i = 0; i < n; i++) str_ptrs[i] = (void*)words[i];
ntc_sort(str_ptrs, n, ntc_compare_str);
printf("Sorted words: ");
for (int i = 0; i < n; i++) printf("%s ", (const char*)str_ptrs[i]);
printf("\n\n");

// Memory & file I/O
int* mem = (int*)os_malloc(10 * sizeof(int));
for (int i = 0; i < 10; i++) mem[i] = i * i;
printf("os_malloc: last element = %d\n", mem[9]);
os_free(mem);

os_file_write("demo_output.txt", "Hello from ntc_lib!\n");
char* content = os_file_read("demo_output.txt");
if (content) { printf("Read file: %s", content); free(content); }

// Cleanup
for (int i = 0; i < count; i++) free(parts[i]);
free(parts);
free(joined);

printf("\n=== NtcLib Demo Complete ===\n");
return 0;
}