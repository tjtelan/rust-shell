use super::RustShellOutput;
use std::str::FromStr;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub enum RustShellBuiltin {
    Echo,
    History,
    Cd,
    Pwd
}

impl FromStr for RustShellBuiltin {
    type Err = ();

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        match s {
            "echo" => Ok(RustShellBuiltin::Echo),
            "history" => Ok(RustShellBuiltin::History),
            "cd" => Ok(RustShellBuiltin::Cd),
            "pwd" => Ok(RustShellBuiltin::Pwd),
            _ => Err(()),
        }
    }
}

pub fn builtin_echo(args : &Vec<String>) -> Result<RustShellOutput, RustShellOutput> {
    Ok(RustShellOutput {
        code: Some(0),
        stdout: String::from(args.join(" ")).into_bytes(),
        stderr: String::from("").into_bytes(),
    })
}

// BUG: If rush_history doesn't exist yet, runtime panic.
pub fn builtin_history(_ : &Vec<String>) -> Result<RustShellOutput, RustShellOutput> {

    let f = File::open("rush_history").expect("rush_history not found");
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let o = lines
        .enumerate()
        .map(|x| format!("{:3} {}", (x.0 + 1), x.1.unwrap()));

    println!("{:?}", o);

    Ok(RustShellOutput {
        code: Some(0),
        stdout: String::from("").into_bytes(),
        stderr: String::from("").into_bytes(),
    })
}

// FIXME: This doesn't actually do anything. Lol.
pub fn builtin_cd(args : &Vec<String>) -> Result<RustShellOutput, RustShellOutput> {
    warn!("Not yet implemented");
    match args.len() {
        0 => {
            info!("Change directories to $HOME");
            Ok(RustShellOutput {
                code: Some(0),
                stdout: String::from(args.join(" ")).into_bytes(),
                stderr: String::from("").into_bytes(),
            })
        },
        _ => {
            info!("Change directories to '{}'", args[0]);
            Ok(RustShellOutput {
                code: Some(0),
                stdout: String::from(args.join(" ")).into_bytes(),
                stderr: String::from("").into_bytes(),
            })
        },
    }
}

pub fn builtin_pwd(_ : &Vec<String>) -> Result<RustShellOutput, RustShellOutput> {
    Err(RustShellOutput {
        code: Some(1),
        stdout: String::from("").into_bytes(),
        stderr: String::from("Not yet implemented").into_bytes(),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
