int addArray() {
  int retValue = 0;
  int array[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};

  for(int i = 0; i < 10; i++) {
    retValue += array[i];
  }

  return retValue;
}
