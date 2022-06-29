pub mod models;

pub use models::models::AuditInfo;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TimeArgs {
    weekday: String,
    month: String,
    day: String,
    hour: String,
    minute: String,
    second: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct JobDetail<T> {
    id: u32,
    created_at: String,
    updated_at: String,
    deleted_at: Option<String>,
    name: String,
    group_id: u32,
    command: Vec<String>,
    code: String,
    depend_jobs: Vec<T>,
    last_cost_time: i32,
    last_exec_time: String,
    next_exec_time: String,
    failed: bool,
    last_exit_status: String,
    created_user_id: i32,
    created_username: String,
    updated_user_id: i32,
    updated_username: String,
    work_user: String,
    work_ip: Vec<String>,
    work_env: Vec<String>,
    work_dir: String,
    kill_child_process: bool,
    timeout: i32,
    process_num: i32,
    error_mail_notify: bool,
    error_apinotify: bool,
    error_dingding_notify: bool,
    retry_num: i32,
    status: i32,
    is_sync: bool,
    mail_to: Vec<T>,
    apito: Vec<T>,
    dingding_to: Vec<T>,
    max_concurrent: i32,
    timeout_trigger: Vec<T>,
    time_args: TimeArgs,
}
