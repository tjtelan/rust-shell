use super::RustShellOutput;
use std::str::FromStr;

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

pub fn builtin_history(_ : &Vec<String>) -> Result<RustShellOutput, RustShellOutput> {
    warn!("Not yet implemented");
    Err(RustShellOutput {
        code: Some(1),
        stdout: String::from("").into_bytes(),
        stderr: String::from("Not yet implemented").into_bytes(),
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
