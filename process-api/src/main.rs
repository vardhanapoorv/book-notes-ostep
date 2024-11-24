use libc::{_exit, close, read, write};
use nix::libc::{dup2, execl, execle, execlp, execv, wait, waitpid, STDIN_FILENO, STDOUT_FILENO};
use nix::unistd::{fork, pipe, ForkResult};
use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::fd::IntoRawFd;
use std::process;
use std::ptr;
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

/*fn fork_child_print_before_parent_with_pipe() {
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
}*/

fn fork_exec_variants() {
    let path = CString::new("/bin/ls").expect("CString::new failed");
    let arg0 = CString::new("ls").expect("CString::new failed");
    let cmd = CString::new("ls").expect("CString::new failed");
    let arg = CString::new("-l").expect("CString::new failed");
    let env = CString::new("PATH=/usr/bin").expect("CString::new failed");
    let envp = [env.as_ptr(), std::ptr::null()];
    let args = [arg0.as_ptr(), ptr::null()];

    // let args_ref: Vec<&CString> = args.iter().collect();
    unsafe {
        let pid = libc::fork();
        if pid < 0 {
            eprintln!("Fork failed");
            process::exit(1);
        } else if pid == 0 {
            // Child process
            // execl(path.as_ptr(), arg0.as_ptr(), ptr::null::<i8>());
            /*execle(
                path.as_ptr(),
                arg0.as_ptr(),
                ptr::null::<i8>(),
                // envp.as_ptr(),
            );*/
            // execlp(cmd.as_ptr(), cmd.as_ptr(), arg.as_ptr(), ptr::null::<i8>());
            execv(path.as_ptr(), args.as_ptr());
        } else {
            // Parent process
            waitpid(pid, null_mut(), 0);
        }
    }
}

fn fork_child_wait() {
    unsafe {
        let pid = libc::fork();
        if pid < 0 {
            eprintln!("Fork failed");
            process::exit(1);
        } else if pid == 0 {
            // Child process
            // Wait in child process returns -1 since there is no child process
            let val = wait(null_mut());
            println!("Child process: {}", val);
            println!("Child process");
        } else {
            // Parent process
            // waitpid(pid, null_mut(), 0);
            // let val = wait(null_mut());
            println!("pid = {}", pid);
            // println!("Parent process: {}", val);
            println!("Parent process");
        }
    }
}

fn fork_child_waitpid() {
    unsafe {
        let pid = libc::fork();
        if pid < 0 {
            eprintln!("Fork failed");
            process::exit(1);
        } else if pid == 0 {
            // Child process
            // Wait in child process returns -1 since there is no child process
            // let val = waitpid(pid, null_mut(), 0);
            // println!("Child process: {}", val);
            println!("Child process");
        } else {
            // Parent process
            // waitpid(pid, null_mut(), 0);
            let status = 0;
            let val = waitpid(pid, null_mut(), 0);
            println!("pid = {}", pid);
            println!("Parent process: {}", val);
            println!("Parent process: status of child {}", status);
            println!("Parent process");
        }
    }
}

fn fork_child_close_std_out() {
    unsafe {
        let pid = libc::fork();
        if pid < 0 {
            eprintln!("Fork failed");
            process::exit(1);
        } else if pid == 0 {
            // Child process
            println!("Child process");
            let result = close(STDOUT_FILENO);
            if result == 0 {
                println!("This should not be printed, STDOUT closed successfully");
            } else {
                println!("Failed to close STDOUT");
            }
        } else {
            // Parent process
            waitpid(pid, null_mut(), 0);
            println!("Parent process");
        }
    }
}

fn fork_two_child_input_pipe_output() {
    // Create a pipe
    let (read_end, write_end) = match pipe() {
        Ok((read_end, write_end)) => (read_end.into_raw_fd(), write_end.into_raw_fd()),
        Err(err) => {
            eprintln!("Failed to create pipe: {}", err);
            process::exit(1);
        }
    };

    // Fork the process
    let pid = unsafe { libc::fork() };
    if pid < 0 {
        eprintln!("Fork failed");
        process::exit(1);
    } else if pid == 0 {
        // Child process
        unsafe {
            close(read_end);
            dup2(write_end, STDOUT_FILENO);
            close(write_end);

            println!("Hello from child 1");
        }
    } else {
        // Parent process
        unsafe {
            let pid2 = libc::fork();
            if pid2 < 0 {
                eprintln!("Fork failed");
                process::exit(1);
            } else if pid2 == 0 {
                // Child process
                close(write_end);
                dup2(read_end, STDIN_FILENO);
                close(read_end);
                let mut buffer = [0u8; 128];
                let nbytes = libc::read(
                    libc::STDIN_FILENO,
                    buffer.as_mut_ptr() as *mut libc::c_void,
                    buffer.len(),
                );

                if nbytes < 0 {
                    // Handle error case
                    eprintln!("Failed to read from standard input.");
                } else {
                    // Convert buffer to a readable string
                    let nbytes = nbytes as usize;
                    println!(
                        "Second child received: {}",
                        String::from_utf8_lossy(&buffer[..nbytes])
                    );
                }
                // Block until the child writes to the pipe
                // read(pipefd[0], buffer.as_mut_ptr() as *mut _, 4);
            }
        }
    }
}

fn main() {
    // fork_child_parent_separate_address_space(); // Question 1
    // fork_file_child_parent(); // Question 2
    // fork_child_print_before_parent(); // Question 3
    // fork_child_print_before_parent_with_pipe(); // Question 3 with pipe
    // fork_exec_variants(); // Question 4
    // fork_child_wait(); // Question 5
    // fork_child_waitpid(); // Question 6
    // fork_child_close_std_out(); // Question 7
    fork_two_child_input_pipe_output(); // Question 8
}
