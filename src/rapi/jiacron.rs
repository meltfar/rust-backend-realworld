use crate::{models::JobDetail, utils::MyError};

use super::call_api;

#[derive(serde::Serialize)]
pub struct EditJobRequest {
    user_id: u32,
    group_id: u32,
    root: bool,
    job: JobDetail<u8>,
}

pub async fn edit_job_call(
    client: &reqwest::Client,
    params: EditJobRequest,
    addr: &str,
) -> Result<(), MyError> {
    call_api::<_, _, JobDetail<u8>>(client, addr, "CrontabJob.Edit", params).await?;

    Ok(())
}
