#[cfg(test)]
mod tests {
    use rs_ansible::*;

    #[test]
    fn generate_connection_options() {
        let options = AnsibleConnectionOptions {
            ask_pass: true,
            connection: "local".into(),
            private_key: "pk".into(),
            scp_extra_args: "scp-extra-args".into(),
            sftp_extra_args: "sftp-extra-args".into(),
            ssh_common_args: "ssh-common-args".into(),
            ssh_extra_args: "ssh-extra-args".into(),
            timeout: 10,
            user: "user".into(),
        };

        let expected = vec![
            "--ask-pass",
            "--connection",
            "local",
            "--private-key",
            "pk",
            "--scp-extra-args",
            "scp-extra-args",
            "--sftp-extra-args",
            "sftp-extra-args",
            "--ssh-common-args",
            "ssh-common-args",
            "--ssh-extra-args",
            "ssh-extra-args",
            "--timeout",
            "10",
            "--user",
            "user",
        ];

        match options.gen_conn_opts() {
            Ok(res) => assert_eq!(res, expected),
            _ => panic!("Err"),
        }
    }

    #[test]
    fn generate_connection_string() {
        let options = AnsibleConnectionOptions {
            ask_pass: true,
            connection: "local".into(),
            private_key: "pk".into(),
            scp_extra_args: "scp-extra-args".into(),
            sftp_extra_args: "sftp-extra-args".into(),
            ssh_common_args: "ssh-common-args".into(),
            ssh_extra_args: "ssh-extra-args".into(),
            timeout: 10,
            user: "user".into(),
        };

        let expected = " --ask-pass --connection local --private-key pk --scp-extra-args scp-extra-args --sftp-extra-args sftp-extra-args --ssh-common-args ssh-common-args --ssh-extra-args ssh-extra-args --timeout 10 --user user";

        assert_eq!(options.to_string(), expected);
    }

    #[test]
    fn generate_privesc_options() {
        let options = AnsiblePrivilegeEscalationOptions {
            do_become: true,
            become_method: "become-method".into(),
            become_user: "become-user".into(),
            ask_become_pass: true,
        };

        let expected = vec![
            "--ask-become-pass",
            "--become",
            "--become-method",
            "become-method",
            "--become-user",
            "become-user",
        ];

        match options.gen_cmd_privesc_opts() {
            Ok(res) => assert_eq!(res, expected),
            _ => panic!("Err"),
        }
    }

    #[test]
    fn generate_privesc_string() {
        let options = AnsiblePrivilegeEscalationOptions {
            do_become: true,
            become_method: "become-method".into(),
            become_user: "become-user".into(),
            ask_become_pass: true,
        };

        let expected =
            " --ask-become-pass --become --become-method become-method --become-user become-user";

        assert_eq!(options.to_string(), expected);
    }
}
