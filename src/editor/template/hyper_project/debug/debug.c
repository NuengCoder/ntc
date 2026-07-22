#include "debug.h"
#include <stdio.h>
#include <ctype.h>

void hex_dump(const void* data, int len) {
const unsigned char* buf = (const unsigned char*)data;
for (int i = 0; i < len; i += 16) {
printf("%08x  ", i);
for (int j = 0; j < 16; j++) {
if (i + j < len) printf("%02x ", buf[i + j]);
else printf("   ");
if (j == 7) putchar(' ');
}
printf(" ");
for (int j = 0; j < 16 && i + j < len; j++)
putchar(isprint(buf[i + j]) ? buf[i + j] : '.');
putchar('\n');
}
}

void print_dsa_stats(const char* label, int size, int capacity) {
printf("[DSA] %s: size=%d, capacity=%d\n", label, size, capacity);
}