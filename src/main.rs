use std::io::{self,Write};

fn main() {
  let prompt_char = "%";
  loop {
    print!("{0} ", prompt_char);
    io::stdout().flush().unwrap();

    let mut command = String::new();
    io::stdin().read_line(&mut command)
      .expect("Failed to read in command");
    println!("DEBUG: Raw input: {:?}", command);

    let command_split : Vec<&str> = command.split(' ').collect();
    println!("DEBUG: Split input: {:?}", command_split);

    let keyword = command_split[0];
    let arguments = &command_split[1..];

    println!("DEBUG: Keyword: {0}", keyword);
    println!("DEBUG: Number of arguments: {0:?}\nDEBUG: Arguments: {1:?}", arguments.len(), arguments);
  }
}
