pub mod models;

pub use models::models::AuditInfo;

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct TimeArgs {
    pub weekday: String,
    pub month: String,
    pub day: String,
    pub hour: String,
    pub minute: String,
    pub second: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct JobDetail {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "CreatedAt")]
    pub created_at: String,
    #[serde(rename = "UpdatedAt")]
    pub updated_at: String,
    #[serde(rename = "DeletedAt")]
    pub deleted_at: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "groupID")]
    pub group_id: u32,
    #[serde(rename = "command")]
    pub command: Vec<String>,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "dependJobs")]
    pub depend_jobs: serde_json::Value,
    #[serde(rename = "lastCostTime")]
    pub last_cost_time: i32,
    #[serde(rename = "lastExecTime")]
    pub last_exec_time: String,
    #[serde(rename = "nextExecTime")]
    pub next_exec_time: String,
    #[serde(rename = "failed")]
    pub failed: bool,
    #[serde(rename = "lastExitStatus")]
    pub last_exit_status: String,
    #[serde(rename = "createdUserId")]
    pub created_user_id: i32,
    #[serde(rename = "createdUsername")]
    pub created_username: String,
    #[serde(rename = "updatedUserID")]
    pub updated_user_id: i32,
    #[serde(rename = "updatedUsername")]
    pub updated_username: String,
    #[serde(rename = "workUser")]
    pub work_user: String,
    #[serde(rename = "workIp")]
    pub work_ip: Vec<String>,
    #[serde(rename = "workEnv")]
    pub work_env: Vec<String>,
    #[serde(rename = "workDir")]
    pub work_dir: String,
    #[serde(rename = "killChildProcess")]
    pub kill_child_process: bool,
    #[serde(rename = "timeout")]
    pub timeout: i32,
    #[serde(rename = "processNum")]
    pub process_num: i32,
    #[serde(rename = "errorMailNotify")]
    pub error_mail_notify: bool,
    #[serde(rename = "errorAPINotify")]
    pub error_apinotify: bool,
    #[serde(rename = "errorDingdingNotify")]
    pub error_dingding_notify: bool,
    #[serde(rename = "retryNum")]
    pub retry_num: i32,
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(rename = "isSync")]
    pub is_sync: bool,
    #[serde(rename = "mailTo")]
    pub mail_to: serde_json::Value,
    #[serde(rename = "APITo")]
    pub apito: serde_json::Value,
    #[serde(rename = "DingdingTo")]
    pub dingding_to: serde_json::Value,
    #[serde(rename = "maxConcurrent")]
    pub max_concurrent: i32,
    #[serde(rename = "timeoutTrigger")]
    pub timeout_trigger: serde_json::Value,
    #[serde(rename = "timeArgs")]
    pub time_args: TimeArgs,
}

impl Default for JobDetail {
    fn default() -> Self {
        Self {
            id: Default::default(),
            created_at: Default::default(),
            updated_at: Default::default(),
            deleted_at: Default::default(),
            name: Default::default(),
            group_id: Default::default(),
            command: Default::default(),
            code: Default::default(),
            depend_jobs: Default::default(),
            last_cost_time: Default::default(),
            last_exec_time: Default::default(),
            next_exec_time: Default::default(),
            failed: Default::default(),
            last_exit_status: Default::default(),
            created_user_id: Default::default(),
            created_username: Default::default(),
            updated_user_id: Default::default(),
            updated_username: Default::default(),
            work_user: Default::default(),
            work_ip: Default::default(),
            work_env: Default::default(),
            work_dir: Default::default(),
            kill_child_process: Default::default(),
            timeout: Default::default(),
            process_num: Default::default(),
            error_mail_notify: Default::default(),
            error_apinotify: Default::default(),
            error_dingding_notify: Default::default(),
            retry_num: Default::default(),
            status: Default::default(),
            is_sync: Default::default(),
            mail_to: Default::default(),
            apito: Default::default(),
            dingding_to: Default::default(),
            max_concurrent: Default::default(),
            timeout_trigger: serde_json::Value::default(),
            time_args: { TimeArgs::default() },
        }
    }
}

pub struct DaemonJobDetail {}
