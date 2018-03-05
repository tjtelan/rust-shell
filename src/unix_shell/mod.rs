use std::io::{self};

pub use repl::Repl;

pub mod builtin;

pub mod util;
use unix_shell::util::*;

#[derive(Debug)]
pub struct RustShellCommand {
    pub keyword : String,
    pub args : Vec<String>,
}

#[derive(Debug)]
pub struct RustShellOutput {
    code: Option<i32>,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

impl Repl for RustShellCommand {
    type ReplResult = RustShellOutput;
    type ReplError = RustShellOutput;

    // TODO: Test for interactive / non-interactive sessions
    fn read() -> RustShellCommand {
        let mut command = String::new();
        io::stdin().read_line(&mut command)
            .expect("Failed to read in command");
        debug!("Raw input: {:?}", command);

        tokenize_command(command)
    }

    fn evaluate(&self) -> Result<RustShellOutput, RustShellOutput> {
        process_command(self)
    }

    fn print(output: Result<RustShellOutput, RustShellOutput>) {
        //println!("Print {:?}", output);
        match output {
            Ok(o) => {
                if !o.stderr.is_empty() { eprintln!("{}", String::from_utf8(o.stderr).unwrap()); }
                if !o.stdout.is_empty() { println!("{}", String::from_utf8(o.stdout).unwrap()); }
            },
            Err(e) => {
                if !e.stderr.is_empty() { eprintln!("{}", String::from_utf8(e.stderr).unwrap()); }
                if !e.stdout.is_empty() { println!("{}", String::from_utf8(e.stdout).unwrap()); }
            },
        }
    }

    fn loop_interactive() {
        loop {
            print_prompt();
            Self::print(Self::evaluate(&Self::read()));
        }
    }
}
