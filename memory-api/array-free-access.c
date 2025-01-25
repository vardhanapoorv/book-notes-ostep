#include <stdio.h>
#include <stdlib.h>

int main() {
  // Allocate memory for an array of 100 integers
  int *data = malloc(100 * sizeof(int));

  // Check if malloc failed
  if (data == NULL) {
    printf("Memory allocation failed\n");
    return 1; // Exit with error code
  }

  // Initialize the array with some values
  data[0] = 42;
  data[1] = 24;

  // Free the allocated memory
  free(data);

  // Attempt to print a value after freeing memory (undefined behavior)
  printf("data[0] = %d\n", data[0]); // This is undefined behavior!

  return 0; // Success
}
