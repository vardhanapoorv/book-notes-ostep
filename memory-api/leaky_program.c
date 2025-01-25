#include <stdlib.h>

void leak_memory() {
  int *ptr = malloc(sizeof(int)); // Allocate memory
  // The allocated memory is not freed, creating a memory leak
}

int main() {
  for (int i = 0; i < 10; i++) {
    leak_memory();
  }
  return 0;
}
