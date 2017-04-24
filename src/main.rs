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

    return command;
}

struct Command <'a> {
  keyword : String,
  arguments : &'a [&'a str],
}

//fn tokenize_command<'a>(c : String) -> &'a Command<'a> {
//    let command_split : Vec<&str> = c.split(' ').collect();
//    println!("DEBUG: Split input: {:?}", command_split);
//
//    let command = Command {
//      keyword : command_split[0],
//      arguments : command_split[1..].to_vec(),
//    };
//
//    println!("DEBUG: Keyword: {0}", command.keyword);
//    println!("DEBUG: Number of arguments: {0:?}", command.arguments.len());
//    println!("DEBUG: Arguments: {0:?}", command.arguments);
//
//    return command;
//}

fn main() {
  loop {
    print_prompt();

    //let command = tokenize_command(read_command());
    let raw_command = read_command();

    let mut command_split : Vec<&str> = raw_command.split(' ').collect();
    println!("DEBUG: Split input: {:?}", command_split);

    let keyword = command_split.remove(0);
    let args = command_split;

    println!("DEBUG: keyword : {:?}", keyword );
    println!("DEBUG: args : {:?}", args );

    let command = Command {
      keyword : keyword.to_string(),
      //arguments : args.to_owned(),
      arguments : args.as_slice(),
    };

    
  }
}
