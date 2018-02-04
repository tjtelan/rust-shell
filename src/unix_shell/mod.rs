use std::io::{self,Write};
use std::str::FromStr;
use std::process::Command;

enum Builtin {
  Echo,
  History,
  Cd,
  Pwd
}

// FIXME: Should this actually be public?
pub struct RustShellCommand {
  pub keyword : String,
  pub args : Vec<String>,
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

fn execute_binary(c : &RustShellCommand) -> i32 {
  let output = Command::new(&c.keyword)
                .args(&c.args)
                .output();

  println!("{:?}", &output);
  match output {
    Ok( o ) => {
      println!("OK!");
      println!("After match: {:?}", String::from_utf8_lossy(&o.stdout));
      
      io::stdout().flush().unwrap();
      //output.unwrap().status.code().unwrap()
      0
    },
    Err(_) => {
      println!("Err!");
      1
    },
  }
}

fn builtin_echo(args : &Vec<String>) -> i32 {
  println!("{}", args.join(" "));
  0
}

fn builtin_history(_ : &Vec<String>) -> i32 {
  warn!("Not yet implemented");
  1
}

fn builtin_cd(args : &Vec<String>) -> i32 {
  warn!("Not yet implemented");
  match args.len() {
    0 => {
      info!("Change directories to $HOME");
      0
    },
    _ => {
      info!("Change directories to '{}'", args[0]);
      0
    },
  }
}

fn builtin_pwd(_ : &Vec<String>) -> i32 {
  println!("Not yet implemented");
  1
}

pub fn print_prompt() {
  let prompt_char = "%";

  print!("{0} ", prompt_char);
  io::stdout().flush().unwrap();
}

pub fn process_command(c : RustShellCommand) -> i32 {
  match Builtin::from_str(&c.keyword) {
    Ok(Builtin::Echo) => builtin_echo(&c.args),
    Ok(Builtin::History) => builtin_history(&c.args),
    Ok(Builtin::Cd) => builtin_cd(&c.args),
    Ok(Builtin::Pwd) => builtin_pwd(&c.args),
    _ => {
      match *&c.keyword.is_empty() {
        true => 0,
        false => execute_binary(&c),
      }
    },
  }
}

