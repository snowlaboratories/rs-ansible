use rs_ansible::options::*;
use rs_ansible::playbook::*;

fn main() {
	let conn_opts = AnsibleConnectionOptions {
		connection: String::from("local"),
        ..Default::default()
	};
	let playbook_opts = AnsiblePlaybookOptions {
		inventory: String::from("127.0.0.1,"),
        ..Default::default()
	};

	let playbook = AnsiblePlaybookCmd {
        binary: String::new(),
		playbooks:         vec![String::from("site.yml"), String::from("site2.yml")],
		options:           playbook_opts,
		connection:           conn_opts,
	};

    match playbook.run() {
        Ok(_) => println!("Yay"),
        _ => panic!("Something went wrong"),
    }
}
