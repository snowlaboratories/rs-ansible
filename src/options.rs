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
pub struct AnsibleConnectionOptions {
    pub ask_pass: bool,
    pub connection: String,
    pub private_key: String,
    pub scp_extra_args: String,
    pub sftp_extra_args: String,
    pub ssh_common_args: String,
    pub ssh_extra_args: String,
    pub timeout: i32,
    pub user: String,
}

impl Default for AnsibleConnectionOptions {
    fn default() -> Self {
        AnsibleConnectionOptions {
            ask_pass: false,
            connection: String::new(),
            private_key: String::new(),
            scp_extra_args: String::new(),
            sftp_extra_args: String::new(),
            ssh_common_args: String::new(),
            ssh_extra_args: String::new(),
            timeout: -1,
            user: String::new(),
        }
    }
}

impl AnsibleConnectionOptions {
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
            cmd.push(self.connection.clone());
        }

        if !self.private_key.is_empty() {
            cmd.push(Self::PRIVATE_KEY_FLAG.to_string());
            cmd.push(self.private_key.clone());
        }

        if !self.scp_extra_args.is_empty() {
            cmd.push(Self::SCP_EXTRA_ARGS_FLAG.to_string());
            cmd.push(self.scp_extra_args.clone());
        }

        if !self.sftp_extra_args.is_empty() {
            cmd.push(Self::SFTP_EXTRA_ARGS_FLAG.to_string());
            cmd.push(self.sftp_extra_args.clone());
        }

        if !self.ssh_common_args.is_empty() {
            cmd.push(Self::SSH_COMMON_ARGS_FLAG.to_string());
            cmd.push(self.ssh_common_args.clone());
        }

        if !self.ssh_extra_args.is_empty() {
            cmd.push(Self::SSH_EXTRA_ARGS_FLAG.to_string());
            cmd.push(self.ssh_extra_args.clone());
        }

        if self.timeout > 0 {
            cmd.push(Self::TIMEOUT_FLAG.to_string());
            cmd.push(self.timeout.to_string());
        }

        if !self.user.is_empty() {
            cmd.push(Self::USER_FLAG.to_string());
            cmd.push(self.user.clone());
        }

        return Ok(cmd);
    }

    /// String return a list of connection options flags to be used on
    /// ansible-playbook execution
    pub fn to_string(&self) -> String {
        let mut options = self
            .gen_conn_opts()
            .expect("generate options")
            .join(" ");
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
pub struct AnsiblePrivilegeEscalationOptions {
    pub ask_become_pass: bool,
    pub do_become: bool,
    pub become_method: String,
    pub become_user: String,
}

impl AnsiblePrivilegeEscalationOptions {
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
            cmd.push(self.become_method.clone());
        }

        if !self.become_user.is_empty() {
            cmd.push(Self::BECOME_USER_FLAG.to_string());
            cmd.push(self.become_user.clone());
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
