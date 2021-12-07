#include <stdio.h>

extern int addArray();

int main() {
  int sum = addArray();

  printf("The array sum is: %d\n", sum);
}
