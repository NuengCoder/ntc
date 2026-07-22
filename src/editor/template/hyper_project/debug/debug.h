#include <stdio.h>

#define ASSERT_NOT_NULL(p)  do { if (!(p)) { fprintf(stderr, "ASSERT FAILED: %s is NULL at %s:%d\n", #p, __FILE__, __LINE__); } } while(0)
#define LOG_ERROR(fmt, ...) fprintf(stderr, "[ERROR] %s:%d: " fmt "\n", __FILE__, __LINE__, ##__VA_ARGS__)
#define LOG_WARN(fmt, ...)  fprintf(stdout, "[WARN]  " fmt "\n", ##__VA_ARGS__)
#define LOG_INFO(fmt, ...)  fprintf(stdout, "[INFO]  " fmt "\n", ##__VA_ARGS__)

void hex_dump(const void* data, int len);
void print_dsa_stats(const char* label, int size, int capacity);