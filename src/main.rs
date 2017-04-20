use std::io;

fn main() {
  loop {
    let mut command = String::new();
    io::stdin().read_line(&mut command)
      .expect("Failed to read in command");
    println!("{0}", command);
  }
}
