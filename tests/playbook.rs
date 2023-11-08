#[cfg(test)]
mod tests {
    use rs_ansible::*;
    use serde_json::json;

    #[test]
    fn generate_connection_options() {
        let options = AnsibleConnectionOptions {
            connection: "local".into(),
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
            options: AnsiblePlaybookOptions,
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
                    skip_tags: "tagN".into(),
                    start_at_task: "second".into(),
                    step: true,
                    tags: "tags".into(),
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
                    extra_vars_file: vec!["@test.yml".into()],
                    flush_cache: true,
                    inventory: "inventory".into(),
                    limit: "limit".into(),
                    list_hosts: true,
                    list_tags: true,
                    list_tasks: true,
                    tags: "tags".into(),
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
            playbooks: vec!["test/ansible/site.yml".into()],
            connection_options: AnsibleConnectionOptions {
                ask_pass: true,
                connection: "local".into(),
                private_key: "pk".into(),
                timeout: 10,
                user: "apenella".into(),
                ..Default::default()
            },
            options: AnsiblePlaybookOptions {
                ask_vault_password: true,
                check: true,
                diff: true,
                forks: "10".into(),
                list_hosts: true,
                module_path: "/dev/null".into(),
                syntax_check: true,
                vault_id: "asdf".into(),
                vault_password_file: "/dev/null".into(),
                verbose: true,
                version: true,

                inventory: "test/ansible/inventory/all".into(),
                limit: "myhost".into(),
                extra_vars: json!({
                    "var1": "value1",
                }),
                flush_cache: true,
                tags: "tag1".into(),
                ..Default::default()
            },
            privilege_escalation_options: AnsiblePrivilegeEscalationOptions {
                do_become: true,
                become_method: "sudo".into(),
                become_user: "apenella".into(),
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
