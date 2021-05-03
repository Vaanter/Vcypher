#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <gmp.h>
#include <wchar.h>

#include "vcypher.h"

char *vcypher(const wchar_t *input) {
  if (input == NULL || wcslen(input) == 0) {
    return "";
  }
  const size_t size = wcslen(input);
  char *binaries[size];
  size_t binaries_size = 0;
  int one_counts[size];
  size_t one_counts_size = 0;

  for (int i = 0; i < size; i++) {
    wchar_t character = input[i];
    int ones = 0;
    char *binary = process_character(character, &ones);
    reverse(binary);
    binaries[binaries_size] = binary;
    one_counts[one_counts_size] = ones;
    one_counts_size += 1;
    binaries_size += 1;
  }

  char *all_bin = concatenate(binaries, binaries_size);
  free_array_contents(binaries, binaries_size);
  size_t counts_size = 0;
  unsigned int *zero_counts = count_zeros(all_bin, &counts_size);
  free(all_bin);

  char **zero_counts_string = convert_arr_to_string(zero_counts, counts_size);
  free(zero_counts);

  if (one_counts_size > 0) {
    char *one_counts_product_str = multiply_ones_together(one_counts, one_counts_size);
    counts_size += 1;
    zero_counts_string[counts_size - 1] = one_counts_product_str;
  }

  char *buffer = cipher(zero_counts_string, counts_size);
  free_array_contents(zero_counts_string, counts_size);
  free(zero_counts_string);

  return buffer;
}

char *process_character(wchar_t character, int *ones) {
  int capacity = MIN_BINARY_SIZE;
  char *binary = calloc(capacity + 1, sizeof(char));
  int binary_size = 0;
  while (character != '\0' || binary_size < MIN_BINARY_SIZE) {
    if (binary_size == capacity) {
      capacity *= 2;
      char *binary_tmp = realloc(binary, capacity + 1);
      binary = binary_tmp;
    }
    int result = (character % 2);
    character = (wchar_t) (character / 2);
    binary[binary_size] = result == 1 ? '1' : '0';
    binary_size += 1;
    *ones += result;
  }
  return binary;
}

char *concatenate(char **binaries, size_t size) {
  size_t capacity = MIN_BINARY_SIZE;
  char *concatenated = calloc(capacity + 1, sizeof(char));
  size_t concatenated_size = 0;
  for (int i = 0; i < size; i++) {
    if (concatenated_size >= capacity) {
      capacity *= 2;
      char *concatenated_tmp = realloc(concatenated, capacity + 1);
      concatenated = concatenated_tmp;
    }
    strcat(concatenated, binaries[i]);
    concatenated_size += strlen(binaries[i]);
  }
  return concatenated;
}

void reverse(char *original) {
  size_t size = strlen(original);
  for (int i = 0; i < size / 2; i++) {
    char tmp = original[i];
    original[i] = original[size - i - 1];
    original[size - i - 1] = tmp;
  }
}

unsigned int *count_zeros(const char *concatenated, size_t *size) {
  size_t capacity = 2;
  unsigned int *counts = calloc(capacity + 1, sizeof(unsigned int));
  size_t count = 0;
  for (int i = 0; i < strlen(concatenated); i++) {
    if (*size >= capacity) {
      capacity *= 2;
      unsigned int *counts_tmp = realloc(counts, (capacity + 1) * sizeof(unsigned int));
      counts = counts_tmp;
    }
    if (concatenated[i] == '0') {
      count += 1;
    } else {
      if (count > 0) {
        if (*size > 0) {
          count += counts[*size - 1];
        }
        counts[*size] = count;
        *size += 1;
      }
      count = 0;
    }
  }
  if (count > 0) {
    if (*size > 0) {
      count += counts[*size - 1];
    }
    counts[*size] = count;
    *size += 1;
  }
  return counts;
}

char *multiply_ones_together(const int *one_counts, const size_t one_counts_size) {
  mpz_t mul;
  mpz_init(mul);
  mpz_set_si(mul, one_counts[0]);
  for (int i = 1; i < one_counts_size; i++) {
    mpz_t n;
    mpz_init_set_d(n, one_counts[i]);
    mpz_mul(mul, mul, n);
    mpz_clear(n);
  }
  char *one_counts_product_str = mpz_get_str(NULL, 10, mul);
  mpz_clear(mul);
  reverse(one_counts_product_str);
  return one_counts_product_str;
}

char **convert_arr_to_string(const unsigned int *zero_counts, const size_t counts_size) {
  char **zero_counts_string = calloc(counts_size + 1, sizeof(char *));
  for (int i = 0; i < counts_size; i++) {
    size_t num_as_string_size = (size_t) floor(log10(zero_counts[i])) + 1;
    char *buffer = calloc(num_as_string_size + 1, sizeof(char));
    snprintf(buffer, num_as_string_size + 1, "%d", zero_counts[i]);
    reverse(buffer);
    zero_counts_string[i] = buffer;
  }
  return zero_counts_string;
}

char *cipher(char **zero_counts_string, size_t counts_size) {
  int position_back = (int) counts_size - 2;
  int position_front = 0;
  size_t buffer_size = 0;
  for (int i = 0; i < counts_size; i++) {
    buffer_size += strlen(zero_counts_string[i]);
  }
  char *buffer = calloc(buffer_size + 1, sizeof(char));
  strcat(buffer, zero_counts_string[counts_size - 1]);
  for (int i = 0; i < ceil( (double)(counts_size - 1) / 2.0); i++) {
    for (int j = 0; j < 2 && strlen(buffer) < buffer_size; j++) {
      if (i % 2 == 0) {
        strcat(buffer, zero_counts_string[position_front]);
        position_front += 1;
      } else {
        strcat(buffer, zero_counts_string[position_back]);
        position_back -= 1;
      }
    }
  }
  return buffer;
}

void free_array_contents(char **arr, size_t size) {
  for (int i = 0; i < size; i++) {
    free(arr[i]);
  }
}
