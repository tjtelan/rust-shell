use std::io::{self,Write};
use std::str;
use std::str::FromStr;
use std::process::{Command, Stdio};

pub use repl::Repl;

enum Builtin {
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

impl Repl for RustShellCommand {
    // TODO: Test for interactive / non-interactive sessions
    fn read() -> RustShellCommand {
        let mut command = String::new();
        io::stdin().read_line(&mut command)
            .expect("Failed to read in command");
        debug!("Raw input: {:?}", command);

        tokenize_command(command)
    }

    // TODO: Think about how to make this work in a state machine way.
    // Do I just want to save the exit code, and stdout/stderr into a history table?
    fn evaluate(&self) -> Result<String, String> {
        process_command(self)
    }


    // TODO: Output should be printed from the user here
    // If I rethink how evaluate works, maybe I just need to print the last command run from cache,
    // or maybe I can stream the output somehow?
    fn print(output: Result<String, String>) -> Option<String> {
        println!("{:?}", output);
        match output {
            Ok(_) => Some(String::from("Good")),
            Err(_) => Some(String::from("Bad")),
        }
    }

    fn loop_interactive() {
//        unimplemented!()
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

// FIXME: Change Builtin to something like unix shell
impl FromStr for Builtin {
    type Err = ();

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        match s {
            "echo" => Ok(Builtin::Echo),
            "history" => Ok(Builtin::History),
            "cd" => Ok(Builtin::Cd),
            "pwd" => Ok(Builtin::Pwd),
            _ => Err(()),
        }
    }
}

fn execute_binary(c : &RustShellCommand) -> Result<String, String> {
//    let output = Command::new(&c.keyword)
//        .args(&c.args)
//        .output();

    let child = Command::new(&c.keyword)
        .args(&c.args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    let output = child
        .wait_with_output()
        .expect("failed to wait on child");

    //println!("Debug: {:?}", &output);

    // if Result<Output>, then I want Ok(Output.status)?
    match output.status.success() {
        true => {
            //println!("OK!");
            //println!("After match: {:?}", String::from_utf8_lossy(&o.stdout));

            //io::stdout().flush().unwrap();
            //output.unwrap().status.code().unwrap()
//            println!("Inner debug: {:?}", o);
            Ok(String::from_utf8_lossy(&output.stdout).to_ascii_lowercase())
        },
        false => {
            //println!("Err!");
 //           println!("Inner debug: {:?}", o);
            Err(String::from_utf8_lossy(&output.stderr).to_ascii_lowercase())
        },
    }
}

fn builtin_echo(args : &Vec<String>) -> Result<String, String> {
    println!("{}", args.join(" "));
    Ok(String::from("0"))
}

fn builtin_history(_ : &Vec<String>) -> Result<String, String> {
    warn!("Not yet implemented");
    Err(String::from("Not implemented"))
}

fn builtin_cd(args : &Vec<String>) -> Result<String, String> {
    warn!("Not yet implemented");
    match args.len() {
        0 => {
            info!("Change directories to $HOME");
            Ok(String::from("0"))
        },
        _ => {
            info!("Change directories to '{}'", args[0]);
            Ok(String::from("0"))
        },
    }
}

fn builtin_pwd(_ : &Vec<String>) -> Result<String, String> {
    println!("Not yet implemented");
    Err(String::from("Not implemented"))
}

pub fn print_prompt() {
    let prompt_char = "%";

    print!("{0} ", prompt_char);
    io::stdout().flush().unwrap();
}

pub fn process_command(c : &RustShellCommand) -> Result<String, String> {
    match Builtin::from_str(&c.keyword) {
        Ok(Builtin::Echo) => builtin_echo(&c.args),
        Ok(Builtin::History) => builtin_history(&c.args),
        Ok(Builtin::Cd) => builtin_cd(&c.args),
        Ok(Builtin::Pwd) => builtin_pwd(&c.args),
        _ => {
            match *&c.keyword.is_empty() {
                true => Err(String::from("Unhandled error")),
                false => execute_binary(&c),
            }
        },
    }
}
