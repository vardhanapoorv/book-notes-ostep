use libc::{_exit, close, pipe, read, write};
use nix::libc::waitpid;
use nix::unistd::{fork, ForkResult};
use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::process;
use std::ptr::null_mut;

fn fork_child_parent_separate_address_space() {
    let mut x = 100;
    println!("Before fork: x = {}", x);

    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            // Child process
            println!("Child process: x = {}", x);
            x = 200; // Change x in child
            println!("Child process: x modified to {}", x);
        }
        Ok(ForkResult::Parent { .. }) => {
            // Parent process
            println!("Parent process: x = {}", x);
            x = 300; // Change x in parent
            println!("Parent process: x modified to {}", x);
        }
        Err(err) => {
            eprintln!("Fork failed: {}", err);
            process::exit(1);
        }
    }
}

fn fork_file_child_parent() {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("file_fork.txt")
        .unwrap();

    unsafe {
        let pid = libc::fork();
        if pid < 0 {
            eprintln!("Fork failed");
            process::exit(1);
        } else if pid == 0 {
            // Child process
            // Without seek, the child process will print nothing
            // because the file pointer is at the end of the file after the parent process reads
            // the file content
            // file.seek(SeekFrom::Start(0)).unwrap();
            for i in 0..10 {
                // file.write_all(b"Child process: Hello, parent!\n").unwrap();
                writeln!(file, "Child process writing").unwrap();
            }

            /*let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            println!("Child process: {}", contents);*/
        } else {
            // Parent process
            // file.write_all(b"Parent process: Hello, child!\n").unwrap();
            for i in 0..10 {
                writeln!(file, "Parent process writing").unwrap();
            }
            /*println!(
                "Continuing execution in parent process, new child has pid: {}",
                pid
            );*/
        }
    }
}

fn fork_child_print_before_parent() {
    unsafe {
        let pid = libc::fork();
        if pid < 0 {
            eprintln!("Fork failed");
            process::exit(1);
        } else if pid == 0 {
            // Child process
            println!("hello");
        } else {
            // Parent process
            println!("Will wait for child process to finish");
            waitpid(pid, null_mut(), 0);
            println!("goodbye");
        }
    }
}

fn fork_child_print_before_parent_with_pipe() {
    let mut pipefd = [0; 2]; // Create an array to hold pipe file descriptors

    // Create a pipe
    if unsafe { pipe(pipefd.as_mut_ptr()) } == -1 {
        eprintln!("Failed to create pipe");
        process::exit(1);
    }

    // Fork the process
    let pid = unsafe { libc::fork() };
    if pid < 0 {
        eprintln!("Fork failed");
        process::exit(1);
    } else if pid == 0 {
        // Child process
        unsafe {
            close(pipefd[0]); // Close the read end of the pipe

            println!("hello"); // Perform the child's task

            // Signal parent by writing to the pipe
            let message = CString::new("done").unwrap();
            write(pipefd[1], message.as_ptr() as *const _, 4);
            close(pipefd[1]); // Close the write end of the pipe
            _exit(0); // Exit the child process
        }
    } else {
        // Parent process
        unsafe {
            let mut buffer = [0; 4]; // Buffer to read from the pipe

            close(pipefd[1]); // Close the write end of the pipe

            // Block until the child writes to the pipe
            read(pipefd[0], buffer.as_mut_ptr() as *mut _, 4);
            close(pipefd[0]); // Close the read end of the pipe

            println!("goodbye"); // Perform the parent's task
        }
    }
}

fn main() {
    // fork_child_parent_separate_address_space(); // Question 1
    // fork_file_child_parent(); // Question 2
    // fork_child_print_before_parent(); // Question 3
    fork_child_print_before_parent_with_pipe(); // Question 3 with pipe
}
