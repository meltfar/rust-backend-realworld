pub mod cmdb;
pub mod jiacron;

pub use cmdb::cmdb_api;
use crate::utils::MyError;

#[derive(serde::Serialize)]
struct ApiParams<'a, K: serde::Serialize> {
    pub addr: &'a str,
    pub method: &'a str,
    pub params: K,
}

#[derive(serde::Deserialize)]
struct ApiResponse<R> {
    pub code: i32,
    pub msg: String,
    pub data: R,
}

pub async fn call_api<R, T, K>(client: &reqwest::Client, addr: T, method_name: T, params: K) -> Result<R, MyError> where T: AsRef<str>, K: serde::Serialize, R: serde::de::DeserializeOwned {
    let ap = ApiParams {
        addr: addr.as_ref(),
        method: method_name.as_ref(),
        params,
    };
    let admin_url = std::env::var("ADMIN_URL").unwrap_or("192.168.150.73:20000".to_string());
    let ret = client.post(format!("http://{}/jiacrontab/v1/callApi", admin_url)).json(&ap).send().await?;
    // let ret = client.post("https://postman-echo.com/post").json(&ap).send().await?;
    // log::info!("{}", ret.text().await?);

    if ret.status() != 200 {
        let t = ret.text().await?;
        log::error!("failed to connect to api admin: {}", &t);
        return Err(crate::error!(t));
    }

    let ar = ret.json::<ApiResponse<R>>().await?;
    if ar.code != 0 {
        return Err(crate::utils::MyError::from_string(ar.msg));
    }

    Ok(ar.data)
    // ar.map(|r|r.data).map_err(|e| e.into())
    // Err(crate::error!("nonono"))
}

#[tokio::test]
async fn test_call_api() {
    let client = reqwest::Client::new();
    let i = call_api::<String, _, _>(&client, "10.25.97.206:20002", "CrontabJob.List", "").await;
}