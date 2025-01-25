#include <stdio.h>
#include <stdlib.h>

int main() {
  int *ptr = NULL; // Create a pointer to an integer and set it to NULL

  printf("Pointer is set to NULL.\n");

  // Attempt to dereference the NULL pointer
  printf("Dereferencing NULL pointer...\n");
  printf("Value: %d\n", *ptr); // This will cause a segmentation fault

  return 0;
}
