#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <time.h>

// Function to simulate memory usage
void use_memory(int megabytes, int seconds) {
  size_t array_size = (size_t)megabytes * 1024 * 1024 /
                      sizeof(int); // Convert MB to number of integers
  int *array = malloc(array_size * sizeof(int));

  if (array == NULL) {
    fprintf(stderr, "Memory allocation failed!\n");
    exit(EXIT_FAILURE);
  }

  printf("Allocated %d MB of memory.\n", megabytes);

  // Stream through the array indefinitely or for a set time
  time_t start_time = time(NULL);
  while (1) {
    for (size_t i = 0; i < array_size; i++) {
      array[i] = (int)i; // Write to memory
    }

    // Exit if the specified time has elapsed
    if (seconds > 0 && (time(NULL) - start_time) >= seconds) {
      break;
    }
  }

  printf("Memory usage completed.\n");
  free(array);
}

int main(int argc, char *argv[]) {
  if (argc < 2 || argc > 3) {
    fprintf(stderr, "Usage: %s <megabytes> [seconds]\n", argv[0]);
    return EXIT_FAILURE;
  }

  int megabytes = atoi(argv[1]);
  if (megabytes <= 0) {
    fprintf(stderr, "Please specify a valid positive number of megabytes.\n");
    return EXIT_FAILURE;
  }

  int seconds = 0;
  if (argc == 3) {
    seconds = atoi(argv[2]);
    if (seconds < 0) {
      fprintf(stderr,
              "Please specify a valid non-negative number of seconds.\n");
      return EXIT_FAILURE;
    }
  }

  use_memory(megabytes, seconds);
  return EXIT_SUCCESS;
}
