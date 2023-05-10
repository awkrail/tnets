use libc::{flockfile, funlockfile, stderr, flock, LOCK_EX};
use std::os::unix::io::AsRawFd;

pub fn debugf(file: &str, line: u32, func: &str, args: Vec<&str>) { 
    lprintf('D', file, line, func, args); 
}

fn lock_stderr() {
    let stderr_fd = std::io::stderr().as_raw_fd();
    unsafe {
        flock(stderr_fd, LOCK_EX);
    }
}

fn unlock_stderr() {
    let stderr_fd = std::io::stderr().as_raw_fd();
    unsafe {
        funlockfile(stderr_fd);
    }
}

fn lprintf(level: char, file: &str, line: u32, func: &str, args: Vec<&str>){
    lock_stderr();
    let mut output_string: String = args.join(" ");
    println!("[{}] {}: {} ({}:{})", level, func, output_string, file, line);
    unlock_stderr();
}
