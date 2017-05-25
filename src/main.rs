use std::io::{self,Write};
use std::str::FromStr;
#[macro_use]
extern crate log;
extern crate env_logger;

fn print_prompt() {
  let prompt_char = "%";

  print!("{0} ", prompt_char);
  io::stdout().flush().unwrap();
}

fn read_command() -> String {
    let mut command = String::new();
    io::stdin().read_line(&mut command)
      .expect("Failed to read in command");
    debug!("Raw input: {:?}", command);

    command
}

struct Command {
  keyword : String,
  args : Vec<String>,
}

fn tokenize_command(c : String) -> Command {
  let mut command_split : Vec<String> = c.split_whitespace().map(|s| s.to_string()).collect();
  debug!("Split input: {:?}", command_split);

  match command_split.len() {
    0 => Command { keyword : "".to_owned(), args : Vec::new()  },
    _ => Command { keyword : command_split.remove(0), args : command_split },
  }
}

enum Builtin {
  Echo,
  History,
  Cd,
  Pwd
}

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

fn process_command(c : Command) -> i32 {
  match Builtin::from_str(&c.keyword) {
    Ok(Builtin::Echo) => builtin_echo(&c.args),
    Ok(Builtin::History) => builtin_history(&c.args),
    Ok(Builtin::Cd) => builtin_cd(&c.args),
    Ok(Builtin::Pwd) => builtin_pwd(&c.args),
    _ => {
      match *&c.keyword.is_empty() {
        true => 0,
        false => {
          println!("{}: command not found", &c.keyword);
          1
        },
      }
    },
  }
}

fn main() {
  env_logger::init().unwrap();

  loop {
    print_prompt();

    let exit_code = process_command(tokenize_command(read_command()));

    debug!("Exit code : {:?}", exit_code );
  }
}

#[cfg(test)]
mod unittest_tokenize_command {
    use super::*;

    #[test]
    fn empty_command() {
      assert_eq!("", tokenize_command("".to_string()).keyword)
    }

    #[test]
    fn test_keyword() {
      assert_eq!("test", tokenize_command("test".to_string()).keyword)
    }

    #[test]
    fn no_arg() {
      assert_eq!(0, tokenize_command("test".to_string()).args.len())
    }

    #[test]
    fn one_arg() {
      assert_eq!(1, tokenize_command("test one".to_string()).args.len())
    }

    #[test]
    fn multi_args() {
      assert_eq!(3, tokenize_command("test one two three".to_string()).args.len())
    }

    #[test]
    #[ignore]
    fn quotes() {
      assert_eq!(2, tokenize_command("test \"one two\" three".to_string()).args.len())
    }
}
