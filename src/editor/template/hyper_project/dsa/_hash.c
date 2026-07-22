#include "_hash.h"

unsigned long long hash_djb2(const unsigned char* str) {
unsigned long long hash = 5381;
int c;
while ((c = *str++)) hash = ((hash << 5) + hash) + (unsigned char)c;
return hash;
}

unsigned long long hash_sdbm(const unsigned char* str) {
unsigned long long hash = 0;
int c;
while ((c = *str++)) hash = (unsigned char)c + (hash << 6) + (hash << 16) - hash;
return hash;
}

unsigned long long hash_fnv1a(const unsigned char* str) {
unsigned long long hash = 14695981039346656037ULL;
int c;
while ((c = *str++)) { hash ^= (unsigned char)c; hash *= 1099511628211ULL; }
return hash;
}

unsigned int hash_murmur3_32(const void* key, int len) {
const unsigned char* data = (const unsigned char*)key;
unsigned int h = 0x971e137b;
for (int i = 0; i < len; i++) {
h ^= data[i];
h *= 0x5bd1e995;
h ^= h >> 15;
}
return h;
}