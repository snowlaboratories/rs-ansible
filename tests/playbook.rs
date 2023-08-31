#[cfg(test)]
mod tests {
    use serde_json::json;

    use rs_ansible::options::*;
    use rs_ansible::playbook::*;

    #[test]
    fn generate_connection_options() {
        let options = AnsibleConnectionOptions {
            connection: "local",
            ..Default::default()
        };
        let expected = vec!["--connection", "local"];

        match options.gen_conn_opts() {
            Ok(res) => assert_eq!(res, expected),
            _ => panic!("Generate connection options"),
        }
    }

    #[test]
    fn generate_playbook_options() {
        struct PlaybookOptsTest<'a> {
            options: AnsiblePlaybookOptions<'a>,
            expected: Vec<&'a str>,
            desc: &'a str,
        }

        let tests = vec![
            PlaybookOptsTest {
                desc: "Empty AnsiblePlaybookOptions definition",
                options: AnsiblePlaybookOptions {
                    ..Default::default()
                },
                expected: vec![],
            },
            PlaybookOptsTest {
                desc: "AnsiblePlaybookOptions except extra vars",
                options: AnsiblePlaybookOptions {
                    flush_cache: true,
                    force_handlers: true,
                    list_tags: true,
                    list_tasks: true,
                    skip_tags: "tagN",
                    start_at_task: "second",
                    step: true,
                    tags: "tags",
                    ..Default::default()
                },
                expected: vec![
                    "--flush-cache",
                    "--force-handlers",
                    "--list-tags",
                    "--list-tasks",
                    "--skip-tags",
                    "tagN",
                    "--start-at-task",
                    "second",
                    "--step",
                    "--tags",
                    "tags",
                ],
            },
            PlaybookOptsTest {
                desc: "AnsiblePlaybookOptions with extra vars",
                options: AnsiblePlaybookOptions {
                    extra_vars: json!({
                        "extra": "var",
                    }),
                    extra_vars_file: vec!["@test.yml"],
                    flush_cache: true,
                    inventory: "inventory",
                    limit: "limit",
                    list_hosts: true,
                    list_tags: true,
                    list_tasks: true,
                    tags: "tags",
                    ..Default::default()
                },
                expected: vec![
                    "--extra-vars",
                    "{\"extra\":\"var\"}",
                    "--extra-vars",
                    "@test.yml",
                    "--flush-cache",
                    "--inventory",
                    "inventory",
                    "--limit",
                    "limit",
                    "--list-hosts",
                    "--list-tags",
                    "--list-tasks",
                    "--tags",
                    "tags",
                ],
            },
        ];

        for test in tests {
            let PlaybookOptsTest {
                desc,
                options,
                expected,
            } = test;
            match options.gen_opts() {
                Ok(res) => assert_eq!(res, expected),
                _ => panic!("{}", desc),
            }
        }
    }

    #[test]
    fn generate_command() {
        let playbook_cmd = AnsiblePlaybookCmd {
            playbooks: vec!["test/ansible/site.yml"],
            connection_options: AnsibleConnectionOptions {
                ask_pass: true,
                connection: "local",
                private_key: "pk",
                timeout: 10,
                user: "apenella",
                ..Default::default()
            },
            options: AnsiblePlaybookOptions {
                ask_vault_password: true,
                check: true,
                diff: true,
                forks: "10",
                list_hosts: true,
                module_path: "/dev/null",
                syntax_check: true,
                vault_id: "asdf",
                vault_password_file: "/dev/null",
                verbose: true,
                version: true,

                inventory: "test/ansible/inventory/all",
                limit: "myhost",
                extra_vars: json!({
                    "var1": "value1",
                }),
                flush_cache: true,
                tags: "tag1",
                ..Default::default()
            },
            privilege_escalation_options: AnsiblePrivilegeEscalationOptions {
                do_become: true,
                become_method: "sudo",
                become_user: "apenella",
                ask_become_pass: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let expected = vec![
            "ansible-playbook",
            "--ask-vault-password",
            "--check",
            "--diff",
            "--extra-vars",
            "{\"var1\":\"value1\"}",
            "--flush-cache",
            "--forks",
            "10",
            "--inventory",
            "test/ansible/inventory/all",
            "--limit",
            "myhost",
            "--list-hosts",
            "--module-path",
            "/dev/null",
            "--syntax-check",
            "--tags",
            "tag1",
            "--vault-id",
            "asdf",
            "--vault-password-file",
            "/dev/null",
            "-vvvv",
            "--version",
            "--ask-pass",
            "--connection",
            "local",
            "--private-key",
            "pk",
            "--timeout",
            "10",
            "--user",
            "apenella",
            "--ask-become-pass",
            "--become",
            "--become-method",
            "sudo",
            "--become-user",
            "apenella",
            "test/ansible/site.yml",
        ];

        match playbook_cmd.command() {
            Ok(res) => assert_eq!(res, expected),
            _ => panic!("generate AnsiblePlaybookCmd command"),
        }
    }
}
