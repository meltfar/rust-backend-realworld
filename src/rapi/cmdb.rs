pub mod cmdb_api {

    #[derive(serde::Deserialize)]
    pub struct AssociateUser {
        pub id: i64,
        pub name: String,
        pub phone: String,
        pub email: String,
    }

    pub async fn get_responsible_user_by_addr(
        client: &reqwest::Client,
        ip_addr: &str,
    ) -> Result<Vec<AssociateUser>, reqwest::Error> {
        client
            .get("http://10.25.224.61:8080/aiops-api/user/queryUserByHost.json")
            .query(&[("ip", ip_addr)])
            .send()
            .await?
            .json::<Vec<AssociateUser>>()
            .await
    }
}
