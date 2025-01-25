#include <stdio.h>
#include <stdlib.h>
int main() {
  int *ptr = malloc(sizeof(int));  // Allocate memory
  int *ptr2 = malloc(sizeof(int)); // Allocate memory
  printf("size of int: %lu\n", sizeof(int));
  // With one malloc on macOS that is somehow cleanup
  // Hence the leaks tool says no leaks
  // But with two mallocs, it says one leak
  // Apparently with valgrind on linux would show the leak with one malloc
  // int *ptr3 = malloc(sizeof(int)); // Allocate memory
  /*for (int i = 0; i < 10; i++) {
    ptr = malloc(sizeof(int)); // Allocate memory
  }*/
  return 0; // Forgot to free(ptr);
}
