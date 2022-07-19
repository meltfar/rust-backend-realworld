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

    pub(crate) fn _get_prefix<'a, T>(url: T) -> &'a str
        where
            T: AsRef<str> + 'a,
    {
        if url.as_ref().starts_with("app") {
            "https"
        } else {
            "http"
        }
    }

    fn get_cmdb_url() -> String {
        std::env::var("CMDB_URL").unwrap_or("10.25.224.61:8080".to_string())
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
        let cmdb_url = get_cmdb_url();

        #[derive(serde::Deserialize)]
        struct TmpRet {
            result: Vec<AssociateUser>,
        }
        client
            .get(format!(
                "http://{}/aiops-api/user/queryUserByHost.json",
                cmdb_url
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
        dest_users: Vec<&str>,
        task_info: &impl TaskInfoLike,
    ) -> Result<(), reqwest::Error> {
        log::debug!("mail: {} - {} - {:#?}", task_info.get_name(), callback_url, dest_users);
        let cmdb_url = get_cmdb_url();
        let mut params = std::collections::HashMap::new();

        dest_users.iter().for_each(|f| {params.insert("to", *f);});
        let title = format!("【定时任务】机器{}中新增任务申请: {}", dest_server_addr, task_info.get_name());
        params.insert("title", &title);

        let template = format!("<div><div>您好，用户 {} 在机器({})中新增了定时任务: {}，请点击 <a href='{}'>{}</a> 前往进行审核；</div><br/><br/><div>您也可以在<a href='http://app.c.vip.migu.cn/root-case-front/#/?jwtCallback=http%3A%2F%2Fapp.c.vip.migu.cn%2Fjiacrontab%2F%23%2Faudit%2Flist%3Ftoken%3D'>http://app.c.vip.migu.cn/root-case-front/#/?jwtCallback=http%3A%2F%2Fapp.c.vip.migu.cn%2Fjiacrontab%2F%23%2Faudit%2Flist%3Ftoken%3D</a>中查看所有待审批任务</div></div>", task_info.get_created_username(), dest_server_addr, task_info.get_name(), callback_url, callback_url);
        params.insert("content", &template);
        params.insert("isHtml", "true");

        let resp = client.post(format!("http://{}/aiops-api/email/send.json", cmdb_url)).form(&params).send().await?;
        log::debug!("email got response: {:#?}", resp.text().await);
        Ok(())
    }

    pub async fn send_sms(
        client: actix_web::web::Data<reqwest::Client>,
        dest_server_addr: &str,
        dest_users: Vec<&str>,
        task_info: impl TaskInfoLike,
    ) -> Result<(), reqwest::Error> {
        log::debug!("sms: {} - {} - {:#?}", dest_server_addr, task_info.get_name(), dest_users);

        let cmdb_url = get_cmdb_url();
        let mut params = std::collections::HashMap::new();
        dest_users.into_iter().for_each(|f| {params.insert("phone", f);});

        let cnt = format!("【定时任务】用户 {} 在机器 ({}) 中新增定时任务: {}，请查看邮件进行审核", task_info.get_created_username(), dest_server_addr, task_info.get_name());
        params.insert("content", &cnt);

        let resp = client.post(format!("http://{}/aiops-api/sms/send.json", cmdb_url)).form(&params).send().await?;
        log::debug!("sms got response: {:#?}", resp.text().await);

        Ok(())
    }
}
