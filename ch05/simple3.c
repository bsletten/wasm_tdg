#include <stdio.h>

void generateArray() {
  int array[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
  int * a = array;
  printf("a is %p\n", a);
  printf("The first value is: %d\n", *a);
  printf("The second value is: %d\n", *(a + 1));
  printf("The third value is: %d\n", *(a + 2));
}

int main() {
  generateArray();
}
