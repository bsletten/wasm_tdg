#include "nanolibc/libc.h"
#include "nanolibc/libc_extra.h"

#define WASM_EXPORT __attribute__((visibility("default"))) extern "C"
 
WASM_EXPORT int* get_memory_for_int_array(int size) {
  return new int[size];
}

WASM_EXPORT void free_memory_for_int_array(int* arr) {
	delete[] arr;
}

WASM_EXPORT void debug_dump_memory() {
	dump_memory();
}

WASM_EXPORT void mergeSort(char *p, int length) {
  int c, d, swap;

  for(c = 0; c < length - 1; c++ ) {
    for( d = 0; d < length - c - 1; d++) {
      if(p[d] > p[d+1]) {
	swap = p[d];
	p[d] = p[d+1];
	p[d+1] = swap;
      }
    }
  }
}

WASM_EXPORT void reverse(unsigned char* p, int len) {
   for( int i = 0; i < len / 2; i++ ) {
     unsigned char temp = p[i];
     p[i] = p[len - i - 1];
     p[len - i - 1] = temp;
   }
}

WASM_EXPORT void helloWorld() {
  printf("Hello, World!\n");
}
