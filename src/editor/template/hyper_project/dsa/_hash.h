unsigned long long hash_djb2(const unsigned char* str);
unsigned long long hash_sdbm(const unsigned char* str);
unsigned long long hash_fnv1a(const unsigned char* str);
unsigned int       hash_murmur3_32(const void* key, int len);