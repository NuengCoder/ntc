#include "os.h"
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#ifdef _WIN32
#include <windows.h>
#else
#include <sys/time.h>
#endif

void* os_malloc(size_t size) {
void* p = malloc(size);
if (!p) { fprintf(stderr, "os_malloc(%zu) failed\n", size); exit(1); }
return p;
}

void* os_calloc(size_t count, size_t size) {
void* p = calloc(count, size);
if (!p) { fprintf(stderr, "os_calloc(%zu, %zu) failed\n", count, size); exit(1); }
return p;
}

void* os_realloc(void* ptr, size_t size) {
void* p = realloc(ptr, size);
if (!p) { fprintf(stderr, "os_realloc(%zu) failed\n", size); exit(1); }
return p;
}

void os_free(void* ptr) { free(ptr); }

char* os_file_read(const char* path) {
FILE* f = fopen(path, "rb");
if (!f) return NULL;
fseek(f, 0, SEEK_END);
long sz = ftell(f);
rewind(f);
char* buf = (char*)malloc(sz + 1);
if (!buf) { fclose(f); return NULL; }
fread(buf, 1, sz, f);
buf[sz] = '\0';
fclose(f);
return buf;
}

int os_file_write(const char* path, const char* content) {
FILE* f = fopen(path, "w");
if (!f) return -1;
if (content) fputs(content, f);
fclose(f);
return 0;
}

double os_timer_ms(void) {
#ifdef _WIN32
LARGE_INTEGER freq, count;
QueryPerformanceFrequency(&freq);
QueryPerformanceCounter(&count);
return (double)count.QuadPart / (double)freq.QuadPart * 1000.0;
#else
struct timeval tv;
gettimeofday(&tv, NULL);
return (double)tv.tv_sec * 1000.0 + (double)tv.tv_usec / 1000.0;
#endif
}