use serde_json::json;
use std::error::Error;

use crate::options::AnsibleConnectionOptions;

/// Parameters described on `Options` section within
/// ansible-playbook's man page, and which defines which should be
/// the ansible-playbook execution behavior.
pub struct AnsiblePlaybookOptions {
    pub ask_vault_password: bool,      // ask for vault password
    pub check: bool, // don't make any changes; instead, try to predict some of the changes that may occur
    pub diff: bool, // when changing (small) files and templates, show the differences in those files; works great with --check
    pub extra_vars: serde_json::Value, // is a map of extra variables used on ansible-playbook execution
    pub extra_vars_file: Vec<String>,  // is a list of files used to load extra-vars
    pub flush_cache: bool,             // is the flush cache flag for ansible-playbook
    pub force_handlers: bool,          // run handlers even if a task fails
    pub forks: String,                 // specify number of parallel processes to use (default=50)
    pub inventory: String,             // specify inventory host path
    pub limit: String,                 // is selected hosts additional pattern
    pub list_hosts: bool,              // outputs a list of matching hosts
    pub list_tags: bool,               // is the list tags flag for ansible-playbook
    pub list_tasks: bool,              // is the list tasks flag for ansible-playbook
    pub module_path: String, // repend colon-separated path(s) to module library (default=~/.ansible/plugins/modules:/usr/share/ansible/plugins/modules)
    pub skip_tags: String,   // only run plays and tasks whose tags do not match these values
    pub start_at_task: String, // start the playbook at the task matching this name
    pub step: bool,          // one-step-at-a-time: confirm each task before running
    pub syntax_check: bool,  // is the syntax check flag for ansible-playbook
    pub tags: String,        // is the tags flag for ansible-playbook
    pub vault_id: String,    // the vault identity to use
    pub vault_password_file: String, // path to the file holding vault decryption key
    pub verbose: bool,       // verbose mode enabled
    pub verbose_v: bool,     // verbose mode -v enabled
    pub verbose_vv: bool,    // verbose mode -vv enabled
    pub verbose_vvv: bool,   // verbose mode -vvv enabled
    pub verbose_vvvv: bool,  // verbose mode -vvvv enabled
    pub version: bool, // show program's version number, config file location, configured module search path, module location, executable location and exit
}

impl Default for AnsiblePlaybookOptions {
    fn default() -> Self {
        AnsiblePlaybookOptions {
            ask_vault_password: false,
            check: false,
            diff: false,
            extra_vars: json!(null),
            extra_vars_file: vec![],
            flush_cache: false,
            force_handlers: false,
            forks: String::new(),
            inventory: String::new(),
            limit: String::new(),
            list_hosts: false,
            list_tags: false,
            list_tasks: false,
            module_path: String::new(),
            skip_tags: String::new(),
            start_at_task: String::new(),
            step: false,
            syntax_check: false,
            tags: String::new(),
            vault_id: String::new(),
            vault_password_file: String::new(),
            verbose: false,
            verbose_v: false,
            verbose_vv: false,
            verbose_vvv: false,
            verbose_vvvv: false,
            version: false,
        }
    }
}

impl AnsiblePlaybookOptions {
    const ASK_VAULT_PASSWORD_FLAG: &str = "--ask-vault-password";
    const CHECK_FLAG: &str = "--check";
    const DIFF_FLAG: &str = "--diff";
    const EXTRA_VARS_FLAG: &str = "--extra-vars";
    const FLUSH_CACHE_FLAG: &str = "--flush-cache";
    const FORCE_HANDLERS_FLAG: &str = "--force-handlers";
    const FORKS_FLAG: &str = "--forks";
    const INVENTORY_FLAG: &str = "--inventory";
    const LIMIT_FLAG: &str = "--limit";
    const LIST_HOSTS_FLAG: &str = "--list-hosts";
    const LIST_TAGS_FLAG: &str = "--list-tags";
    const LIST_TASKS_FLAG: &str = "--list-tasks";
    const MODULE_PATH_FLAG: &str = "--module-path";
    const SKIP_TAGS_FLAG: &str = "--skip-tags";
    const START_AT_TASK_FLAG: &str = "--start-at-task";
    const STEP_FLAG: &str = "--step";
    const SYNTAX_CHECK_FLAG: &str = "--syntax-check";
    const TAGS_FLAG: &str = "--tags";
    const VAULT_ID_FLAG: &str = "--vault-id";
    const VAULT_PASSWORD_FILE_FLAG: &str = "--vault-password-file";
    const VERSION_FLAG: &str = "--version";
    const VERBOSE_FLAG: &str = "-vvvv";
    const VERBOSE_V_FLAG: &str = "-v";
    const VERBOSE_VV_FLAG: &str = "-vv";
    const VERBOSE_VVV_FLAG: &str = "-vvv";
    const VERBOSE_VVVV_FLAG: &str = "-vvvv";

    fn gen_verbosity(&self) -> &str {
        if self.verbose {
            return Self::VERBOSE_FLAG;
        }
        if self.verbose_v {
            return Self::VERBOSE_V_FLAG;
        }
        if self.verbose_vv {
            return Self::VERBOSE_VV_FLAG;
        }
        if self.verbose_vvv {
            return Self::VERBOSE_VVV_FLAG;
        }
        if self.verbose_vvvv {
            return Self::VERBOSE_VVVV_FLAG;
        }
        return "";
    }

    fn gen_extra_args(&self) -> String {
        return self.extra_vars.to_string();
    }

    /// Returns a list of options flags to be used on ansible-playbook execution
    pub fn gen_opts(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut cmd: Vec<String> = Vec::new();

        if self.ask_vault_password {
            cmd.push(Self::ASK_VAULT_PASSWORD_FLAG.to_string());
        }

        if self.check {
            cmd.push(Self::CHECK_FLAG.to_string());
        }

        if self.diff {
            cmd.push(Self::DIFF_FLAG.to_string());
        }

        if !self.extra_vars.is_null() {
            cmd.push(Self::EXTRA_VARS_FLAG.to_string());
            cmd.push(self.gen_extra_args());
        }

        for file in self.extra_vars_file.clone().into_iter() {
            cmd.push(Self::EXTRA_VARS_FLAG.to_string());
            cmd.push(file.clone());
        }

        if self.flush_cache {
            cmd.push(Self::FLUSH_CACHE_FLAG.to_string());
        }

        if self.force_handlers {
            cmd.push(Self::FORCE_HANDLERS_FLAG.to_string());
        }

        if !self.forks.is_empty() {
            cmd.push(Self::FORKS_FLAG.to_string());
            cmd.push(self.forks.clone());
        }

        if !self.inventory.is_empty() {
            cmd.push(Self::INVENTORY_FLAG.to_string());
            cmd.push(self.inventory.clone());
        }

        if !self.limit.is_empty() {
            cmd.push(Self::LIMIT_FLAG.to_string());
            cmd.push(self.limit.clone());
        }

        if self.list_hosts {
            cmd.push(Self::LIST_HOSTS_FLAG.to_string());
        }

        if self.list_tags {
            cmd.push(Self::LIST_TAGS_FLAG.to_string());
        }

        if self.list_tasks {
            cmd.push(Self::LIST_TASKS_FLAG.to_string());
        }

        if !self.module_path.is_empty() {
            cmd.push(Self::MODULE_PATH_FLAG.to_string());
            cmd.push(self.module_path.clone());
        }

        if !self.skip_tags.is_empty() {
            cmd.push(Self::SKIP_TAGS_FLAG.to_string());
            cmd.push(self.skip_tags.clone());
        }

        if !self.start_at_task.is_empty() {
            cmd.push(Self::START_AT_TASK_FLAG.to_string());
            cmd.push(self.start_at_task.clone());
        }

        if self.step {
            cmd.push(Self::STEP_FLAG.to_string());
        }

        if self.syntax_check {
            cmd.push(Self::SYNTAX_CHECK_FLAG.to_string());
        }

        if !self.tags.is_empty() {
            cmd.push(Self::TAGS_FLAG.to_string());
            cmd.push(self.tags.clone());
        }

        if !self.vault_id.is_empty() {
            cmd.push(Self::VAULT_ID_FLAG.to_string());
            cmd.push(self.vault_id.clone());
        }

        if !self.vault_password_file.is_empty() {
            cmd.push(Self::VAULT_PASSWORD_FILE_FLAG.to_string());
            cmd.push(self.vault_password_file.clone());
        }

        let verbose_flag = self.gen_verbosity();
        if !verbose_flag.is_empty() {
            cmd.push(verbose_flag.to_string());
        }

        if self.version {
            cmd.push(Self::VERSION_FLAG.to_string());
        }

        Ok(cmd)
    }
}

pub struct AnsiblePlaybookCmd {
    pub binary: String,
    pub playbooks: Vec<String>,
    pub options: AnsiblePlaybookOptions,
    pub connection: AnsibleConnectionOptions,
    // ... (other fields)
}

impl AnsiblePlaybookCmd {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        // ... (implementation for running ansible-playbook)
        todo!()
    }

    pub fn command(&self) -> Result<Vec<String>, Box<dyn Error>> {
        // ... (implementation for generating command)
        todo!()
    }
}
