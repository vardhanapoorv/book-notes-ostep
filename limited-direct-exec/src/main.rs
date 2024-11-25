use libc::{getpid, gettimeofday, open, timeval, O_RDONLY, O_RDWR, O_WRONLY};
use std::{ffi::CString, mem::MaybeUninit};

fn gettimeofday_precision() {
    const NUM_MEASUREMENTS: usize = 100;

    let mut deltas = Vec::new();

    for _ in 0..NUM_MEASUREMENTS {
        // Capture two consecutive timestamps
        let mut start = MaybeUninit::<timeval>::uninit();
        let mut end = MaybeUninit::<timeval>::uninit();

        unsafe {
            gettimeofday(start.as_mut_ptr(), std::ptr::null_mut());
            gettimeofday(end.as_mut_ptr(), std::ptr::null_mut());

            let start = start.assume_init();
            let end = end.assume_init();

            // Calculate the delta in microseconds
            let delta =
                (end.tv_sec - start.tv_sec) * 1_000_000 + (end.tv_usec - start.tv_usec) as i64;
            deltas.push(delta);
        }
    }

    // Analyze the results
    let min_delta = deltas.iter().min().unwrap_or(&0);
    let max_delta = deltas.iter().max().unwrap_or(&0);
    let avg_delta: f64 = deltas.iter().map(|&x| x as f64).sum::<f64>() / deltas.len() as f64;

    println!("Precision of gettimeofday():");
    println!("  Minimum delta: {} µs", min_delta);
    println!("  Maximum delta: {} µs", max_delta);
    println!("  Average delta: {:.2} µs", avg_delta);
}

fn measure_system_call_cost() {
    const NUM_ITERATIONS: usize = 100_000;

    let mut total_time = 0;

    for _ in 0..NUM_ITERATIONS {
        // Initialize timeval structs
        let mut start = MaybeUninit::<timeval>::uninit();
        let mut end = MaybeUninit::<timeval>::uninit();

        unsafe {
            // Get the start time
            gettimeofday(start.as_mut_ptr(), std::ptr::null_mut());
            let start = start.assume_init();

            // Perform the system call (e.g., getpid())
            // getpid();
            let file_path = CString::new("measure-test-open.txt").unwrap();
            open(file_path.as_ptr(), O_RDONLY);
            // open(file_path.as_ptr(), O_RDWR); // As expected takes more time than O_RDONLY

            // Get the end time
            gettimeofday(end.as_mut_ptr(), std::ptr::null_mut());
            let end = end.assume_init();

            // Calculate the time difference in microseconds
            let elapsed =
                (end.tv_sec - start.tv_sec) * 1_000_000 + (end.tv_usec - start.tv_usec) as i64;
            total_time += elapsed;
        }
    }

    // Calculate the average system call cost
    let avg_time = total_time as f64 / NUM_ITERATIONS as f64;
    println!("Average system call cost: {:.3} µs", avg_time);
}

fn main() {
    // gettimeofday_precision();
    measure_system_call_cost();
}
