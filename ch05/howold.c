#include <stdio.h>

int howOld(int currentYear, int yearBorn) {

  int retValue = -1;
  
  if(yearBorn <= currentYear) {
    retValue = currentYear - yearBorn;
  }

  return retValue;
}

int main() {
  int age = howOld(2021, 2000);

  if(age >= 0) {
    printf("You are %d!\n", age);
  } else {
    printf("You haven't been born yet.");
  }
}
