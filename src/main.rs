#[macro_use]
extern crate log;
extern crate env_logger;

pub mod repl;
pub mod unix_shell;
use repl::Repl;


fn main() {
    env_logger::init().unwrap();

    // TODO: Check if this is an interactive session before entering loop_interactive()
    unix_shell::RustShellCommand::loop_interactive();
}
