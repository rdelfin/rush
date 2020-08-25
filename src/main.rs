use nix::{
    sys::wait::wait,
    unistd::{execvp, fork, ForkResult},
};
use std::env;
use std::ffi::{CStr, CString};

mod parser;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match fork() {
        Ok(ForkResult::Parent { child }) => {
            println!("From the parent, child has PID {}", child);
            wait().unwrap();
            println!("Child finished!")
        }
        Ok(ForkResult::Child) => {
            let args_cstr: Vec<_> = args
                .iter()
                .skip(1)
                .map(|a| CString::new(a.as_bytes()).unwrap())
                .collect();

            let args_cstring: Vec<&CStr> = args_cstr.iter().map(|c| c.as_c_str()).collect();
            println!(
                "Running execvp on program \"{}\" with args {:?}",
                args[0], args_cstring,
            );
            execvp(&CString::new(args[0].as_bytes()).unwrap(), &args_cstring).unwrap();
        }
        Err(_) => println!("Fork failed."),
    }
}
