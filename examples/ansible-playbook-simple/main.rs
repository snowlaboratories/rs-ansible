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
        binary: "",
        playbooks: vec!["site.yml", "site2.yml"],
        options: playbook_opts,
        connection_options: conn_opts,
        ..Default::default()
    };

    match playbook.run() {
        Ok(_) => println!("Yay"),
        _ => panic!("Something went wrong"),
    }
}
