use anyhow::Result;
use nix::{
    sys::wait::wait,
    unistd::{execvp, fork, ForkResult},
};
use std::ffi::CString;

use crate::parser::ASTToken;

pub fn execute<'a, T: Iterator<Item = &'a ASTToken>>(iter: T) -> Result<()> {
    for token in iter {
        match token {
            ASTToken::Command { name, args } => run_cmd(name, args)?,
        }
    }
    Ok(())
}

fn run_cmd(name: &str, args: &Vec<String>) -> Result<()> {
    match fork()? {
        ForkResult::Child => {
            let mut full_args = vec![name.to_string()];
            full_args.extend(args.iter().cloned());
            let args_cstring = full_args
                .iter()
                .map(|s| CString::new(s.as_bytes()).unwrap())
                .collect::<Vec<_>>();

            let args_cstr = args_cstring
                .iter()
                .map(|s| s.as_c_str())
                .collect::<Vec<_>>();

            execvp(&CString::new(name.as_bytes())?, &args_cstr)?;
        }
        ForkResult::Parent { child: _ } => {
            wait()?;
        }
    }

    Ok(())
}
