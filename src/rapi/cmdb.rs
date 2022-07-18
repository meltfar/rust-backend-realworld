pub mod cmdb_api {
    #[derive(serde::Deserialize, Debug)]
    pub struct AssociateUser {
        pub id: u32,
        pub name: String,
        pub phone: String,
        pub email: String,
    }

    #[derive(serde::Deserialize, Debug)]
    pub struct MicroserviceGroup {
        pub id: u32,
        #[serde(rename = "opsContactId")]
        pub ops_contact_id: u32,
    }

    pub(crate) fn get_prefix<'a, T>(url: T) -> &'a str
        where
            T: AsRef<str> + 'a,
    {
        if url.as_ref().starts_with("app") {
            "https"
        } else {
            "http"
        }
    }

    pub async fn get_microservice_group_simple_list(client: &reqwest::Client) -> Result<Vec<MicroserviceGroup>, reqwest::Error> {
        let cmdb_url = std::env::var("CMDB_URL").unwrap_or("10.25.224.61:8080".to_string());

        #[derive(serde::Deserialize)]
        struct TmpRet {
            result: Vec<MicroserviceGroup>,
        }
        client.get(format!("http://{}/aiops-api/microServiceGroup/simpleList", cmdb_url)).send().await?.json::<TmpRet>().await.map(|f| f.result)
    }

    pub async fn get_responsible_user_by_addr(
        client: &reqwest::Client,
        group_id: u32,
    ) -> Result<Vec<AssociateUser>, reqwest::Error> {
        let cmdb_url = std::env::var("CMDB_URL").unwrap_or("10.25.224.61:8080".to_string());
        let prefix = get_prefix(&cmdb_url);

        #[derive(serde::Deserialize)]
        struct TmpRet {
            result: Vec<AssociateUser>,
        }
        client
            .get(format!(
                "{}://{}/aiops-api/user/queryUserByHost.json",
                prefix, cmdb_url
            ))
            .query(&[("groupId", group_id)])
            .send()
            .await?
            .json::<TmpRet>()
            .await
            .map(|f| f.result)
    }

    pub trait TaskInfoLike {
        fn get_name(&self) -> &str;

        fn get_created_username(&self) -> &str;
    }

    pub async fn send_mail(
        client: actix_web::web::Data<reqwest::Client>,
        callback_url: &str,
        dest_server_addr: &str,
        task_info: &impl TaskInfoLike,
    ) -> Result<(), reqwest::Error> {
        log::info!("mail: {} - {}", task_info.get_name(), callback_url);
        Ok(())
    }

    pub async fn send_sms(
        client: actix_web::web::Data<reqwest::Client>,
        dest_server_addr: &str,
        task_info: impl TaskInfoLike,
    ) -> Result<(), reqwest::Error> {
        log::info!("sms: {} - {}", dest_server_addr, task_info.get_name());
        Ok(())
    }
}
