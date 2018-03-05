use super::RustShellCommand;
use super::RustShellOutput;
use std::io::{self,Write};
use unix_shell::builtin::*;
use std::str::FromStr;
use std::process::{Command};

pub fn tokenize_command(c : String) -> RustShellCommand {
    let mut command_split : Vec<String> = c.split_whitespace().map(|s| s.to_string()).collect();
    debug!("Split input: {:?}", command_split);

    match command_split.len() {
        0 => RustShellCommand { keyword : "".to_owned(), args : Vec::new()  },
        _ => RustShellCommand { keyword : command_split.remove(0), args : command_split },
    }
}

pub fn print_prompt() {
    let prompt_char = "%";

    print!("{0} ", prompt_char);
    io::stdout().flush().unwrap();
}

pub fn process_command(c : &RustShellCommand) -> Result<RustShellOutput, RustShellOutput> {
    match RustShellBuiltin::from_str(&c.keyword) {
        Ok(RustShellBuiltin::Echo) => builtin_echo(&c.args),
        Ok(RustShellBuiltin::History) => builtin_history(&c.args),
        Ok(RustShellBuiltin::Cd) => builtin_cd(&c.args),
        Ok(RustShellBuiltin::Pwd) => builtin_pwd(&c.args),

        _ => {
            match *&c.keyword.is_empty() {
                true => {
                    debug!("Empty command. Possibly Ctrl+D pressed");
                    Err(RustShellOutput {
                        code: Some(1),
                        stdout: String::from("").into_bytes(),
                        stderr: String::from("").into_bytes(),
                    })
                },
                false => execute_binary(&c),
            }
        },
    }
}

fn execute_binary(c : &RustShellCommand) -> Result<RustShellOutput, RustShellOutput> {
    // TODO: Maybe pipe stdout and stderr so print() will handle all i/o
    // Figure out how to make vim continue working
    //  .stdout(Stdio::piped())
    //  .stderr(Stdio::piped())
    let child = Command::new(&c.keyword)
        .args(&c.args)
        .spawn();

    match child {
        Ok(process) => {
          let output = process.wait_with_output();

            match output {
                Ok(x) => {
                    Ok(RustShellOutput {
                        code: x.status.code(),
                        stdout: x.stdout,
                        stderr: x.stderr,
                    })
                },

                // TODO: Need to write a test that hits this condition, but I don't know how...
                Err(_x) => {
                    Err(RustShellOutput {
                        code: Some(1),
                        stdout: String::from("").into_bytes(),
                        stderr: String::from("DEBUG: Command error").into_bytes(),
                    })
                },
            }

        },

            // TODO: Less hardcoding of binary name here
        Err(e) => {
            eprintln!("DEBUG: Exit code: {:?}", e.raw_os_error());
            Err(RustShellOutput {
                code: e.raw_os_error(),
                stdout: String::from("").into_bytes(),
                stderr: String::from(format!("rush: {}: command not found", &c.keyword)).into_bytes(),
            })
        },
    }
}

