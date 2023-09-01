use std::io::{Error};
use std::process::{Command, Child};
use which::which;


pub fn verify_binary(binary: &str, error_context: &str) {
    which(binary).expect(
        format!(
            "{} Binary file '{}' does not exists",
            error_context, binary
        )
        .as_str(),
    );
}


pub type CallbackType = fn(String) -> String;
fn default_callback(line: String) -> String {
    return line;
}

#[derive(Clone, Copy)]
pub struct DefaultExecutor {
    pub callback: CallbackType,
}

impl Default for DefaultExecutor {
    fn default() -> Self {
        DefaultExecutor {
            callback: default_callback,
        }
    }
}

impl DefaultExecutor {
    pub fn run(&self, command: Vec<String>) -> Result<Child, Error> {
        verify_binary(&command[0].as_str(), "(executor::run)");

        let child = Command::new(command[0].clone())
            .args(&command[1..])
            .spawn();

        return child;
    }
}
