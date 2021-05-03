#include "vcypher.h"

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <wchar.h>

int main(int argc, char **argv) {
  for (int i = 1; i < argc; i++) {
    wchar_t *input = calloc(strlen(argv[i]) + 1, sizeof(*input));
    swprintf(input, strlen(argv[i]) + 1, L"%hs", argv[i]);
    char *result = vcypher(input);
    printf("%s\n", result);
    free(result);
    free(input);
  }
  return 0;
}
