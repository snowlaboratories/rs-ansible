use rs_ansible::*;

fn main() {
    let conn_opts = AnsibleConnectionOptions {
        connection: "local".into(),
        ..Default::default()
    };
    let playbook_opts = AnsiblePlaybookOptions {
        inventory: "127.0.0.1,".into(),
        ..Default::default()
    };

    let playbook = AnsiblePlaybookCmd {
        playbooks: vec!["site.yml".into(), "site2.yml".into()],
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
