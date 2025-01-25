#include <stdio.h>
#include <stdlib.h>

typedef struct {
  int *data;       // Pointer to the array
  size_t size;     // Number of elements currently in the vector
  size_t capacity; // Current capacity of the vector
} Vector;

// Function to initialize the vector
void vector_init(Vector *vector) {
  vector->data = NULL;
  vector->size = 0;
  vector->capacity = 0;
}

// Function to add an element to the vector
void vector_add(Vector *vector, int value) {
  // Check if we need to resize the array
  if (vector->size == vector->capacity) {
    // Double the capacity (or start with 1 if it's 0)
    size_t new_capacity = vector->capacity == 0 ? 1 : vector->capacity * 2;
    int *new_data = realloc(vector->data, new_capacity * sizeof(int));
    if (!new_data) {
      fprintf(stderr, "Memory allocation failed\n");
      exit(1); // Exit on allocation failure
    }
    vector->data = new_data;
    vector->capacity = new_capacity;
    printf("Resized the vector to %zu\n", vector->capacity);
  }

  // Add the new element
  vector->data[vector->size++] = value;
}

// Function to print the vector
void vector_print(Vector *vector) {
  printf("Vector elements: ");
  for (size_t i = 0; i < vector->size; i++) {
    printf("%d ", vector->data[i]);
  }
  printf("\n");
}

// Function to free the vector
void vector_free(Vector *vector) {
  free(vector->data);
  vector->data = NULL;
  vector->size = 0;
  vector->capacity = 0;
}

// Main function to test the vector
int main() {
  Vector vector;
  vector_init(&vector);

  // Add elements to the vector
  for (int i = 0; i < 2; i++) {
    vector_add(&vector, i);
  }

  // Print the vector
  vector_print(&vector);

  // Free the vector
  // vector_free(&vector);

  // Use `leaks` here to ensure no memory leaks
  return 0;
}
