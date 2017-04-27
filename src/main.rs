use std::io::{self,Write};

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

fn main() {
  loop {
    print_prompt();

    let command = tokenize_command(read_command());

    println!("DEBUG: keyword : {:?}", command.keyword );
    println!("DEBUG: args : {:?}", command.args );
    
  }
}
