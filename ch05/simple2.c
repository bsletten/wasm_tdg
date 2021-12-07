#include <stdio.h>

void generateArray() {
  int array[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
  int * a = array;
  printf("The first value is: %d\n", *a);
}

int main() {
  generateArray();
}
