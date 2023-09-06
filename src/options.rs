use std::env;
use std::error::Error;

const ANSIBLE_FORCE_COLOR_ENV: &str = "ANSIBLE_FORCE_COLOR";
const ANSIBLE_HOST_KEY_CHECKING_ENV: &str = "ANSIBLE_HOST_KEY_CHECKING";

pub fn ansible_force_color() {
    env::set_var(ANSIBLE_FORCE_COLOR_ENV, "true");
}

pub fn ansible_avoid_host_key_checking() {
    env::set_var(ANSIBLE_HOST_KEY_CHECKING_ENV, "false");
}

pub fn ansible_set_env(key: &str, value: &str) {
    env::set_var(key, value);
}

/// Has those parameters described on `Connections Options` section within
/// ansible-playbook's man page, and which defines how to connect to hosts.
#[derive(Clone, Copy)]
pub struct AnsibleConnectionOptions<'a> {
    pub ask_pass: bool,
    pub connection: &'a str,
    pub private_key: &'a str,
    pub scp_extra_args: &'a str,
    pub sftp_extra_args: &'a str,
    pub ssh_common_args: &'a str,
    pub ssh_extra_args: &'a str,
    pub timeout: i32,
    pub user: &'a str,
}

impl Default for AnsibleConnectionOptions<'_> {
    fn default() -> Self {
        AnsibleConnectionOptions {
            ask_pass: false,
            connection: "",
            private_key: "",
            scp_extra_args: "",
            sftp_extra_args: "",
            ssh_common_args: "",
            ssh_extra_args: "",
            timeout: -1,
            user: "",
        }
    }
}

impl AnsibleConnectionOptions<'_> {
    const ASK_PASS_FLAG: &str = "--ask-pass";
    const CONNECTION_FLAG: &str = "--connection";
    const PRIVATE_KEY_FLAG: &str = "--private-key";
    const SCP_EXTRA_ARGS_FLAG: &str = "--scp-extra-args";
    const SFTP_EXTRA_ARGS_FLAG: &str = "--sftp-extra-args";
    const SSH_COMMON_ARGS_FLAG: &str = "--ssh-common-args";
    const SSH_EXTRA_ARGS_FLAG: &str = "--ssh-extra-args";
    const TIMEOUT_FLAG: &str = "--timeout";
    const USER_FLAG: &str = "--user";

    /// Returns a list of connection options flags to be used on
    /// ansible-playbook execution
    pub fn gen_conn_opts(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut cmd = vec![];

        if self.ask_pass {
            cmd.push(Self::ASK_PASS_FLAG.to_string());
        }

        if !self.connection.is_empty() {
            cmd.push(Self::CONNECTION_FLAG.to_string());
            cmd.push(self.connection.to_string().clone());
        }

        if !self.private_key.is_empty() {
            cmd.push(Self::PRIVATE_KEY_FLAG.to_string());
            cmd.push(self.private_key.to_string().clone());
        }

        if !self.scp_extra_args.is_empty() {
            cmd.push(Self::SCP_EXTRA_ARGS_FLAG.to_string());
            cmd.push(self.scp_extra_args.to_string().clone());
        }

        if !self.sftp_extra_args.is_empty() {
            cmd.push(Self::SFTP_EXTRA_ARGS_FLAG.to_string());
            cmd.push(self.sftp_extra_args.to_string().clone());
        }

        if !self.ssh_common_args.is_empty() {
            cmd.push(Self::SSH_COMMON_ARGS_FLAG.to_string());
            cmd.push(self.ssh_common_args.to_string().clone());
        }

        if !self.ssh_extra_args.is_empty() {
            cmd.push(Self::SSH_EXTRA_ARGS_FLAG.to_string());
            cmd.push(self.ssh_extra_args.to_string().clone());
        }

        if self.timeout > 0 {
            cmd.push(Self::TIMEOUT_FLAG.to_string());
            cmd.push(self.timeout.to_string());
        }

        if !self.user.is_empty() {
            cmd.push(Self::USER_FLAG.to_string());
            cmd.push(self.user.to_string().clone());
        }

        return Ok(cmd);
    }

    /// String return a list of connection options flags to be used on
    /// ansible-playbook execution
    pub fn to_string(&self) -> String {
        let mut options = self.gen_conn_opts().expect("generate options").join(" ");
        options.insert(0, ' ');

        return options;
    }
}

/// Parameters described on `Privilege Escalation Options` section within
/// ansible-playbook's man page, and which controls how and which user
/// you become as on target hosts.
/* do_become methods
    ksu        Kerberos substitute user
    pbrun      PowerBroker run
    enable     Switch to elevated permissions on a network device
    sesu       CA Privileged Access Manager
    pmrun      Privilege Manager run
    runas      Run As user
    sudo       Substitute User DO
    su         Substitute User
    doas       Do As user
    pfexec     profile based execution
    machinectl Systemd's machinectl privilege escalation
    dzdo       Centrify's Direct Authorize
*/
#[derive(Default, Clone, Copy)]
pub struct AnsiblePrivilegeEscalationOptions<'a> {
    pub ask_become_pass: bool,
    pub do_become: bool,
    pub become_method: &'a str,
    pub become_user: &'a str,
}

impl AnsiblePrivilegeEscalationOptions<'_> {
    const ASK_BECOME_PASS_FLAG: &str = "--ask-become-pass";
    const BECOME_FLAG: &str = "--become";
    const BECOME_METHOD_FLAG: &str = "--become-method";
    const BECOME_USER_FLAG: &str = "--become-user";

    /// returns a list of privilege escalation options flags to be used on
    /// ansible-playbook execution
    pub fn gen_cmd_privesc_opts(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut cmd = vec![];

        if self.ask_become_pass {
            cmd.push(Self::ASK_BECOME_PASS_FLAG.to_string());
        }

        if self.do_become {
            cmd.push(Self::BECOME_FLAG.to_string());
        }

        if !self.become_method.is_empty() {
            cmd.push(Self::BECOME_METHOD_FLAG.to_string());
            cmd.push(self.become_method.to_string().clone());
        }

        if !self.become_user.is_empty() {
            cmd.push(Self::BECOME_USER_FLAG.to_string());
            cmd.push(self.become_user.to_string().clone());
        }

        return Ok(cmd);
    }

    pub fn to_string(&self) -> String {
        let mut options = self
            .gen_cmd_privesc_opts()
            .expect("generate options")
            .join(" ");
        options.insert(0, ' ');

        return options;
    }
}
