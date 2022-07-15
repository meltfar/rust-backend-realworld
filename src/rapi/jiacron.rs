use crate::{models::JobDetail, utils::MyError};

use super::call_api;

#[derive(serde::Serialize)]
pub struct EditJobRequest<'a> {
    pub user_id: u32,
    pub group_id: u32,
    pub root: bool,
    pub job: &'a JobDetail,
}

pub async fn edit_job_call(
    client: &reqwest::Client,
    params: EditJobRequest<'_>,
    addr: &str,
) -> Result<JobDetail, MyError> {
    call_api::<_, _, EditJobRequest<'_>>(client, addr, "CrontabJob.Edit", params).await
}

pub async fn get_job_call(client: &reqwest::Client, job_id: u32, addr: &str) -> Result<serde_json::Value, MyError> {
    let mut tmp = serde_json::Map::new();
    tmp.insert("group_id".to_string(), serde_json::Value::from(1u32));
    tmp.insert("job_id".to_string(), serde_json::Value::from(job_id));
    call_api::<_, _, _>(client, addr, "CrontabJob.Get", tmp).await
}