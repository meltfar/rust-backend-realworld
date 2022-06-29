pub mod cmdb;
pub mod jiacron;

pub use cmdb::cmdb_api;
use crate::utils::MyError;

pub async fn call_api<T, K, R>(client: &reqwest::Client, addr: T, method_name: T, params: K) -> Result<R, MyError> where T: AsRef<str>, K: serde::Serialize, R: serde::de::DeserializeOwned {
    let ret = client.post("http://www.baidu.com/v1/callApi").json(&params).send().await?;
    if ret.status() != 200 {
        let t = ret.text().await?;
        log::error!("failed to connect to api admin: {}", &t);
        return Err(crate::error!(t));
    }

    ret.json().await.map_err(|e| e.into())
}