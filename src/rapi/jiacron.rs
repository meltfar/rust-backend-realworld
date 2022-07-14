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
