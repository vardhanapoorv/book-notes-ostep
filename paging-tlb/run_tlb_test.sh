#!/bin/bash

# Ensure the Rust program is built
cargo build --release

# Define parameters
EXECUTABLE="./target/release/paging-tlb"
TRIALS=1000000  # Number of trials per run
START_PAGES=1    # Start with 1 page
MAX_PAGES=8192   # Go up to 8192 pages (adjust based on machine)

# Output file
OUTPUT_FILE="tlb_results-on-same-cpu-without-initialization.txt"
echo "Pages Accessed, Avg Time per Access (ns)" > $OUTPUT_FILE

# Run the test while doubling the number of pages each iteration
PAGES=$START_PAGES
while [ $PAGES -le $MAX_PAGES ]; do
    echo "Running with $PAGES pages..."
    
    # Run the Rust program and extract the timing result
    RESULT=$($EXECUTABLE $PAGES $TRIALS | grep "Average time per access" | awk '{print $5}')
    
    # Log the result
    echo "$PAGES, $RESULT" | tee -a $OUTPUT_FILE

    # Double the number of pages for the next iteration
    PAGES=$((PAGES * 2))
done

echo "Done! Results saved in $OUTPUT_FILE."

