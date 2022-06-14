pub mod cmdb {
    // { id: number; name: string; phone: string; email: string }[]
    #[derive(serde::Deserialize)]
    pub struct CmdbQueryResp {
        pub id: i64,
        pub name: String,
        pub phone: String,
        pub email: String,
    }

    pub async fn query_user_cmdb(addr: &str) -> Result<CmdbQueryResp, reqwest::Error>  {
        let cmdb_url = std::env::var("CMDB_URL").unwrap_or("10.25.224.61:8080".to_string());
        reqwest::get(format!(
            "https://${cmdb_url}/aiops-api/user/queryUserByHost.json?ip=${addr}"
        ))
        .await?
        .json::<CmdbQueryResp>()
        .await
    }
}
