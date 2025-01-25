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

  // Out-of-bounds access (this is unsafe)
  data[100] = 0;
  printf("data[100] = %d\n", data[100]);

  return 0; // Success
}
