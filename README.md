# rs-ansible
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) ![Test](https://github.com/snowlaboratories/rs-ansible/actions/workflows/ci.yml/badge.svg)
![rs-ansible-logo](docs/assets/logo.png "rs-ansible logo" )

## Overview

`rs-ansible` is a Rust library designed to facilitate the seamless execution of Ansible playbooks and commands directly from within a Rust program.


## Features

- **Playbook Execution:** Execute Ansible playbooks programmatically within your Rust application.
- **Command Execution:** Run Ansible commands and modules from your Rust code.
- **Integration:** Easily integrate Ansible tasks into your Rust application's workflow.
- **Flexibility:** Supports custom Ansible configurations and inventory management.
- **Error Handling:** Provides robust error handling and reporting for Ansible tasks.


## Installation

Add `rs-ansible` to your project's `Cargo.toml`:

```toml
[dependencies]
rs-ansible = { git = "https://github.com/snowlaboratories/rs-ansible.git" }
```

## Usage

### Running an Ansible Playbook

```rust
use rs_ansible::options::{AnsibleConnectionOptions};
use rs_ansible::playbook::{AnsiblePlaybookOptions, AnsiblePlaybookCmd};

fn main() {
    let conn_opts = AnsibleConnectionOptions {
        connection: "local",
        ..Default::default()
    };
    let playbook_opts = AnsiblePlaybookOptions {
        inventory: "127.0.0.1,",
        ..Default::default()
    };

    let playbook = AnsiblePlaybookCmd {
        binary: "",
        playbooks: vec!["site.yml", "site2.yml"],
        options: playbook_opts,
        connection_options: conn_opts,
        ..Default::default()
    };

    match playbook.run() {
        Ok(_) => println!("Yay"),
        _ => panic!("Something went wrong"),
    };
}
```

For more advanced usage and customization, please refer to the provided [examples](./examples/).

## Contributing

We welcome contributions from the open-source community. If you find any issues or have ideas for improvements, please open an issue or submit a pull request on our [GitHub repository](https://github.com/snowlaboratories/rs-ansible).

## License

This project is licensed under the [MIT License](LICENSE).

## Contact

For questions, suggestions, or support, feel free to open an [issue ](https://github.com/snowlaboratories/rs-ansible/issues).

## Acknowledgments

We would like to express our gratitude to the Ansible community for their excellent work in developing and maintaining Ansible.
Great thanks also to [@apenella](https://github.com/apenella) for their [`go-ansible`](https://github.com/apenella/go-ansible/) original librairy, which greatly inpired us.

Thank you for choosing `rs-ansible` for your Rust and Ansible integration needs! We look forward to seeing how you leverage its power in your projects.
