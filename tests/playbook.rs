#[cfg(test)]
mod tests {
    use serde_json::json;

    use rs_ansible::options::*;
    use rs_ansible::playbook::*;

    #[test]
    fn generate_connection_options() {
        struct ConnectionOptsTest<'a> {
            options: AnsibleConnectionOptions,
            expected: Vec<&'a str>,
            desc: &'a str,
        }

        let tests = vec![ConnectionOptsTest {
            desc: "Generate connection options",
            options: AnsibleConnectionOptions {
                connection: String::from("local"),
                ..Default::default()
            },
            expected: vec!["--connection", "local"],
        }];

        for test in tests {
            let ConnectionOptsTest {
                desc,
                options,
                expected,
            } = test;
            match options.gen_conn_opts() {
                Ok(res) => assert_eq!(res, expected),
                _ => panic!("{}", desc),
            }
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
                    skip_tags: String::from("tagN"),
                    start_at_task: String::from("second"),
                    step: true,
                    tags: String::from("tags"),
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
                    extra_vars_file: vec![String::from("@test.yml")],
                    flush_cache: true,
                    inventory: String::from("inventory"),
                    limit: String::from("limit"),
                    list_hosts: true,
                    list_tags: true,
                    list_tasks: true,
                    tags: String::from("tags"),
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
}
