mod repl;
use repl::*;
mod unix_shell;
use unix_shell::*;

#[macro_use]
extern crate log;
extern crate env_logger;

fn main() {
  env_logger::init().unwrap();

  loop {
    print_prompt();

    let exit_code = process_command(tokenize_command(read_command()));

    debug!("Exit code : {:?}", exit_code );
  }
}

