#ifndef VCYPHER_VCYPHER_H
#define VCYPHER_VCYPHER_H

#endif //VCYPHER_VCYPHER_H

#include <gmp.h>
#include <wchar.h>

#define MIN_BINARY_SIZE 8

char *vcypher(const wchar_t *input);

void free_char_double_ptr(char **arr, size_t size);

char *process_character(wchar_t character, int *ones);

char *concatenate(char **binaries, size_t size);

void reverse(char *original);

unsigned int *count_zeros(const char *concatenated, size_t *size);

char *multiply_ones_together(const int *one_counts, const size_t one_counts_size);

char *cipher(char **zero_counts_string, size_t counts_size);

char **convert_arr_to_string(const unsigned int *zero_counts, size_t counts_size);