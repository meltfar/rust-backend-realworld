pub mod cmdb_api {

    #[derive(serde::Deserialize)]
    pub struct AssociateUser {
        pub id: i64,
        pub name: String,
        pub phone: String,
        pub email: String,
    }

    const CMDB_SERVER_ADDR: &'static str = "http://10.25.224.61:8080";

    pub async fn get_responsible_user_by_addr(
        client: &reqwest::Client,
        ip_addr: &str,
    ) -> Result<Vec<AssociateUser>, reqwest::Error> {
        client
            .get(format!(
                "{}/aiops-api/user/queryUserByHost.json",
                CMDB_SERVER_ADDR
            ))
            .query(&[("ip", ip_addr)])
            .send()
            .await?
            .json::<Vec<AssociateUser>>()
            .await
    }

    pub trait TaskInfoLike {
        fn get_name(&self) -> &str;

        fn get_created_username(&self) -> &str;
    }

    pub async fn send_mail(
        client: &reqwest::Client,
        callback_url: &str,
        dest_server_addr: &str,
        task_info: &impl TaskInfoLike
    ) -> Result<(), reqwest::Error> {
        task_info.get_name();
        Ok(())
    }
}
