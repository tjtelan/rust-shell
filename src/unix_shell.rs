use std::io::{self,Write};
use std::str;
use std::str::FromStr;
use std::process::{Command};

pub use repl::Repl;

enum RustShellBuiltin {
    Echo,
    History,
    Cd,
    Pwd
}

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
                println!("{}", String::from_utf8(o.stdout).unwrap());
                eprintln!("{}", String::from_utf8(o.stderr).unwrap());
            },
            Err(e) => {
                println!("{}", String::from_utf8(e.stdout).unwrap());
                eprintln!("{}", String::from_utf8(e.stderr).unwrap());
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

fn tokenize_command(c : String) -> RustShellCommand {
    let mut command_split : Vec<String> = c.split_whitespace().map(|s| s.to_string()).collect();
    debug!("Split input: {:?}", command_split);

    match command_split.len() {
        0 => RustShellCommand { keyword : "".to_owned(), args : Vec::new()  },
        _ => RustShellCommand { keyword : command_split.remove(0), args : command_split },
    }
}

impl FromStr for RustShellBuiltin {
    type Err = ();

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        match s {
            "echo" => Ok(RustShellBuiltin::Echo),
            "history" => Ok(RustShellBuiltin::History),
            "cd" => Ok(RustShellBuiltin::Cd),
            "pwd" => Ok(RustShellBuiltin::Pwd),
            _ => Err(()),
        }
    }
}

fn execute_binary(c : &RustShellCommand) -> Result<RustShellOutput, RustShellOutput> {
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

fn builtin_echo(args : &Vec<String>) -> Result<RustShellOutput, RustShellOutput> {
    Ok(RustShellOutput {
        code: Some(0),
        stdout: String::from(args.join(" ")).into_bytes(),
        stderr: String::from("").into_bytes(),
    })
}

fn builtin_history(_ : &Vec<String>) -> Result<RustShellOutput, RustShellOutput> {
    warn!("Not yet implemented");
    Err(RustShellOutput {
        code: Some(1),
        stdout: String::from("").into_bytes(),
        stderr: String::from("Not yet implemented").into_bytes(),
    })
}

fn builtin_cd(args : &Vec<String>) -> Result<RustShellOutput, RustShellOutput> {
    warn!("Not yet implemented");
    match args.len() {
        0 => {
            info!("Change directories to $HOME");
            Ok(RustShellOutput {
                code: Some(0),
                stdout: String::from(args.join(" ")).into_bytes(),
                stderr: String::from("").into_bytes(),
            })
        },
        _ => {
            info!("Change directories to '{}'", args[0]);
            Ok(RustShellOutput {
                code: Some(0),
                stdout: String::from(args.join(" ")).into_bytes(),
                stderr: String::from("").into_bytes(),
            })
        },
    }
}

fn builtin_pwd(_ : &Vec<String>) -> Result<RustShellOutput, RustShellOutput> {
    Err(RustShellOutput {
        code: Some(1),
        stdout: String::from("").into_bytes(),
        stderr: String::from("Not yet implemented").into_bytes(),
    })
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
