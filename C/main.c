#include <stdlib.h>
#include <stdio.h>

#include "vcypher.h"

int main() {
  char in[4] = "BFC";
  char *result = vcypher(in);
  printf("%s", result);
  free(result);
  return 0;
}
