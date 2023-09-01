#[cfg(test)]
mod tests {
    use crate::test_utils::*;
    use rs_ansible::executor::*;
    use std::fs;

    #[test]
    #[should_panic]
    #[allow(unused_must_use)]
    fn should_fail_() {
        let executor = DefaultExecutor {};
        let command = vec!["non-existing-binary", "-i", "127.0.0.1,"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        executor.run(command);
    }

    #[test]
    #[allow(unused_must_use)]
    fn run_command() {
        let executor = DefaultExecutor {};

        let file = random_file();
        let command = vec!["touch", file.as_str()]
            .iter()
            .map(|s| s.to_string())
            .collect();

        match executor.run(command) {
            Ok(mut child) => {
                assert!(child.wait().expect("Command was not running").success());
                match fs::metadata(file.clone()) {
                    Ok(_) => fs::remove_file(file),
                    Err(err) => panic!("{}", err),
                };
            }
            Err(err) => panic!("{}", err),
        }
    }
}

#[cfg(test)]
pub mod test_utils {
    use rand::{thread_rng, Rng};

    /// Generate a random filename in /tmp
    /// eg. `/tmp/rs-ansible-test-123456789-01234567`
    pub fn random_file() -> String {
        let mut uuid = [0i32; 3];
        thread_rng().fill(&mut uuid[..]);
        return format!(
            "/tmp/rs-ansible-test-{}",
            Vec::from(uuid)
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("")
        );
    }

    #[test]
    fn random_file_generation() {
        let f1 = random_file();
        let f2 = random_file();
        assert!(f1 != f2);
    }
}
