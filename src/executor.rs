use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub struct DefaultExecutor {}
impl DefaultExecutor {
    pub fn run(&self, _command: Vec<String>) -> Result<(), Error> {

    fn main() -> Result<(), Error> {
        let stdout = Command::new("journalctl")
            .stdout(Stdio::piped())
            .spawn()?
            .stdout
            .ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output."))?;

        let reader = BufReader::new(stdout);

        reader
            .lines()
            .filter_map(|line| line.ok())
            .filter(|line| line.find("usb").is_some())
            .for_each(|line| println!("{}", line));

         Ok(())
    }
}
