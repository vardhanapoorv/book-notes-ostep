use libc::{
    mach_thread_self, pthread_self, pthread_threadid_np, sysconf, thread_policy_set,
    THREAD_AFFINITY_POLICY, THREAD_AFFINITY_POLICY_COUNT, _SC_PAGESIZE,
};
use std::env;
use std::ptr;
use std::time::Instant;

// Function to pin the thread to a specific CPU core (MacOS)
#[cfg(target_os = "macos")]
fn pin_thread_to_core(core_id: usize) {
    let core_id = core_id as i32;
    unsafe {
        let policy = [core_id];
        thread_policy_set(
            mach_thread_self(),
            THREAD_AFFINITY_POLICY.try_into().unwrap(),
            policy.as_ptr() as *mut _,
            THREAD_AFFINITY_POLICY_COUNT as u32,
        );
    }
}

// Function to get the current thread ID for migration tracking
#[cfg(target_os = "macos")]
fn get_thread_id() -> u64 {
    let mut tid: u64 = 0;
    unsafe {
        pthread_threadid_np(pthread_self(), &mut tid);
    }
    tid
}

fn main() {
    // Get initial thread ID for migration tracking
    #[cfg(target_os = "macos")]
    let initial_tid = get_thread_id();

    // Pin the thread to CPU core 0 for stable measurements
    #[cfg(target_os = "macos")]
    pin_thread_to_core(0);

    println!("Thread pinned to core 0. Monitoring CPU migrations...");

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
    /*for i in (0..array_size).step_by(jump) {
        unsafe { ptr::write_volatile(&mut array[i], 1) };
    }*/

    // Measure access time
    let start = Instant::now();
    for _ in 0..num_trials {
        for i in (0..array_size).step_by(jump) {
            unsafe { ptr::write_volatile(&mut array[i], array[i] + 1) };
        }
    }
    let duration = start.elapsed();

    // Get final thread ID for migration tracking
    #[cfg(target_os = "macos")]
    let final_tid = get_thread_id();

    if initial_tid != final_tid {
        eprintln!(
            "⚠️ WARNING: Thread migrated during execution! (Initial: {}, Final: {})",
            initial_tid, final_tid
        );
    } else {
        println!("✅ No CPU migration detected. Thread remained on core 0.");
    }

    // Output results
    let total_accesses = num_pages * num_trials;
    let avg_time_ns = (duration.as_nanos() as f64) / (total_accesses as f64);

    println!("Total time: {:?}", duration);
    println!("Average time per access: {:.2} ns", avg_time_ns);
}
