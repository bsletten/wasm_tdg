int howOld(int currentYear, int yearBorn) {

  int retValue = -1;
  
  if(yearBorn <= currentYear) {
    retValue = currentYear - yearBorn;
  }

  return retValue;
}
