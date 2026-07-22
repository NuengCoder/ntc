#include <stdlib.h>
#include <stdio.h>

void* os_malloc(size_t size);
void* os_calloc(size_t count, size_t size);
void* os_realloc(void* ptr, size_t size);
void  os_free(void* ptr);
char* os_file_read(const char* path);
int   os_file_write(const char* path, const char* content);
double os_timer_ms(void);