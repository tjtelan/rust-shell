use std::io::{self,Write};
use std::str::FromStr;

fn print_prompt() {
  let prompt_char = "%";

  print!("{0} ", prompt_char);
  io::stdout().flush().unwrap();
}

fn read_command() -> String {
    let mut command = String::new();
    io::stdin().read_line(&mut command)
      .expect("Failed to read in command");
    println!("DEBUG: Raw input: {:?}", command);

    command
}

struct Command {
  keyword : String,
  args : Vec<String>,
}

fn tokenize_command(c : String) -> Command {
  let mut command_split : Vec<String> = c.split_whitespace().map(|s| s.to_string()).collect();
  println!("DEBUG: Split input: {:?}", command_split);

  let command = Command {
    keyword : command_split.remove(0),
    args : command_split,
  };

  command
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

fn builtin_history(args : &Vec<String>) -> i32 {
  println!("Not yet implemented");
  1
}

fn builtin_cd(args : &Vec<String>) -> i32 {
  println!("Not yet implemented");
  1
}

fn builtin_pwd(args : &Vec<String>) -> i32 {
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
        println!("{}: command not found", &c.keyword);
        1
    },
  }
}

fn main() {
  loop {
    print_prompt();

    let command = tokenize_command(read_command());

    println!("DEBUG: keyword : {:?}", command.keyword );
    println!("DEBUG: args : {:?}", command.args );
    
    let exit_code = process_command(command);
    println!("DEBUG: Exit code : {:?}", exit_code );
  }
}

#[cfg(test)]
mod unittest_tokenize_command {
    use super::*;

    #[test]
    #[ignore]
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
