use std::error::Error;
use std::process::{Child, Command};
use which::which;

pub fn verify_binary(binary: &str) -> Result<(), Box<dyn Error>> {
    match which(binary) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Binary file '{}' does not exists: ({})", binary, err).into())
    }
}

pub struct DefaultExecutor {}

impl DefaultExecutor {
    pub fn run(&self, command: Vec<String>) -> Result<Child, Box<dyn Error>> {
        if let Err(err) = verify_binary(&command[0].as_str()) {
            return Err(format!("(executor::run) {}", err).into());
        }

        let child = Command::new(command[0].clone())
            .args(&command[1..])
            .spawn()
            .expect("(executor::run) run command");

        return Ok(child);
    }
}
