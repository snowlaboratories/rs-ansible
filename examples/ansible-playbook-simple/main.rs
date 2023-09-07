use rs_ansible::options::*;
use rs_ansible::playbook::*;

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
        playbooks: vec!["site.yml", "site2.yml"],
        options: playbook_opts,
        connection_options: conn_opts,
        ..Default::default()
    };

    match playbook.run() {
        // Ok(child: std::process::Child) => ... to control the process
        Ok(_) => println!("Playbook Started"),
        Err(err) => panic!("Something went wrong: {}", err),
    };
}
