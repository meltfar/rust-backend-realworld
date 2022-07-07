pub mod cmdb;
pub mod jiacron;

pub use cmdb::cmdb_api;
use crate::utils::MyError;

pub async fn call_api<R, T, K>(client: &reqwest::Client, addr: T, method_name: T, params: K) -> Result<R, MyError> where T: AsRef<str>, K: serde::Serialize, R: serde::de::DeserializeOwned {
    let ret = client.post("http://10.25.97.205:20000/v1/callApi").json(&params).send().await?;
    if ret.status() != 200 {
        let t = ret.text().await?;
        log::error!("failed to connect to api admin: {}", &t);
        return Err(crate::error!(t));
    }

    ret.json().await.map_err(|e| e.into())
}

#[tokio::test]
async fn test_call_api() {
    let client = reqwest::Client::new();
    let i = call_api::<String, _, _>(&client, "10.25.97.206:20002", "CrontabJob.List", "").await;
}