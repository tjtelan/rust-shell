use std::io::{self};

// FIXME: This is should be temporary until I can make the grammars pluggable
use unix_shell::RustShellCommand;

pub fn tokenize_command(c : String) -> RustShellCommand {
  let mut command_split : Vec<String> = c.split_whitespace().map(|s| s.to_string()).collect();
  debug!("Split input: {:?}", command_split);

  match command_split.len() {
    0 => RustShellCommand { keyword : "".to_owned(), args : Vec::new()  },
    _ => RustShellCommand { keyword : command_split.remove(0), args : command_split },
  }
}

pub fn read_command() -> String {
    let mut command = String::new();
    io::stdin().read_line(&mut command)
      .expect("Failed to read in command");
    debug!("Raw input: {:?}", command);

    command
}


// TODO: Execute trait? Generic function?
// TODO: Prompt trait?

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
