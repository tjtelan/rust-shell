use std::io::{self,Write};

fn main() {
  let prompt_char = "%";
  loop {
    print!("{0} ", prompt_char);
    io::stdout().flush().unwrap();

    let mut command = String::new();
    io::stdin().read_line(&mut command)
      .expect("Failed to read in command");
    println!("{:?}", command);

    let command_split : Vec<&str> = command.split(' ').collect();
    println!("{:?}", command_split);
  }
}
