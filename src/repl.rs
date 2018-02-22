pub trait Repl {
    fn read() -> Self;
    fn evaluate(&self) -> Result<String, String>;
    fn print(output: Result<String, String>) -> Option<String>;
    fn loop_interactive();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
