#[cfg(test)]
mod tests {
    use rs_ansible::options::*;

    #[test]
    fn generate_connection_options() {
        let options = AnsibleConnectionOptions {
            ask_pass: true,
            connection: String::from("local"),
            private_key: String::from("pk"),
            scp_extra_args: String::from("scp-extra-args"),
            sftp_extra_args: String::from("sftp-extra-args"),
            ssh_common_args: String::from("ssh-common-args"),
            ssh_extra_args: String::from("ssh-extra-args"),
            timeout: 10,
            user: String::from("user"),
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
            connection: String::from("local"),
            private_key: String::from("pk"),
            scp_extra_args: String::from("scp-extra-args"),
            sftp_extra_args: String::from("sftp-extra-args"),
            ssh_common_args: String::from("ssh-common-args"),
            ssh_extra_args: String::from("ssh-extra-args"),
            timeout: 10,
            user: String::from("user"),
        };

        let expected = " --ask-pass --connection local --private-key pk --scp-extra-args scp-extra-args --sftp-extra-args sftp-extra-args --ssh-common-args ssh-common-args --ssh-extra-args ssh-extra-args --timeout 10 --user user";

        assert_eq!(options.to_string(), expected);
    }

    #[test]
    fn generate_privesc_options() {
        let options = AnsiblePrivilegeEscalationOptions {
            do_become: true,
            become_method: String::from("become-method"),
            become_user: String::from("become-user"),
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
            become_method: String::from("become-method"),
            become_user: String::from("become-user"),
            ask_become_pass: true,
        };

        let expected =
            " --ask-become-pass --become --become-method become-method --become-user become-user";

        assert_eq!(options.to_string(), expected);
    }
}
