use libc::{sysconf, _SC_PAGESIZE};
use std::env;
use std::ptr;
use std::time::Instant;

fn main() {
    // Read command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <num_pages> <num_trials>", args[0]);
        return;
    }

    let num_pages: usize = args[1].parse().expect("Invalid number of pages");
    let num_trials: usize = args[2].parse().expect("Invalid number of trials");

    // Get system page size
    let page_size = unsafe { sysconf(_SC_PAGESIZE) as usize };
    let jump = page_size / std::mem::size_of::<usize>();

    // Allocate memory for the array
    let array_size = num_pages * jump;
    let mut array = vec![0usize; array_size];

    // Prefault pages to ensure they are in memory
    for i in (0..array_size).step_by(jump) {
        unsafe { ptr::write_volatile(&mut array[i], 1) };
        // array[i] = 1;
    }

    // Measure access time
    let start = Instant::now();
    for _ in 0..num_trials {
        for i in (0..array_size).step_by(jump) {
            unsafe { ptr::write_volatile(&mut array[i], array[i] + 1) };
            // array[i] += 1;
        }
    }
    let duration = start.elapsed();

    // Output results
    let total_accesses = num_pages * num_trials;
    let avg_time_ns = (duration.as_nanos() as f64) / (total_accesses as f64);

    println!("Total time: {:?}", duration);
    println!("Average time per access: {:.2} ns", avg_time_ns);
}
