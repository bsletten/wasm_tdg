#include <emscripten.h>
#include <stdio.h>

EMSCRIPTEN_KEEPALIVE int add(int x, int y) {
  return x + y;
}

/*int main() {
  printf("The sum of 2 and 3 is: %d\n", add(2,3));
  return 0;
}*/
