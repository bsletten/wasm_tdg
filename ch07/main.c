#include <stdio.h>

extern void sayHello(char *message);

int main() {
  printf("Hello, world.\n");
  sayHello("How are you?");
  return 0;
}
