pub mod job_controller {
    use actix_web::web;

    // use crate::models::models::models;
    use crate::{models::AuditInfo, models::models::models, rapi::cmdb_api};
    use crate::error;
    use crate::rpc::cmdb::cmdb;
    use crate::utils::MyError;

// use models as entity_models;

    // use models as entity_models;

    type Response<T> = actix_web::Result<T, MyError>;

    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EditJobReq {
        permitted: String,
        auditor_token: String,
        job_id: i64,
        addr: String,
        r#type: i32,
        reject_reason: String,
        group_id: u32
    }

    #[derive(serde::Deserialize)]
    #[allow(unused)]
    struct AuthorizationPayload {
        exp: i32,
        #[serde(rename = "Version")]
        version: i32,
        #[serde(rename = "UserID")]
        user_id: i64,
        #[serde(rename = "Mail")]
        mail: String,
        #[serde(rename = "Username")]
        username: String,
        #[serde(rename = "GroupID")]
        group_id: i64,
        #[serde(rename = "Root")]
        root: bool,
    }

    pub struct CronJobDetail {
        pub name: String,
        pub created_username: String,
    }

    impl cmdb_api::TaskInfoLike for CronJobDetail {
        fn get_name(&self) -> &str {
            return &self.name;
        }

        fn get_created_username(&self) -> &str {
            return &self.created_username;
        }
    }

    pub async fn error_return() -> Response<impl actix_web::Responder> {
        // std::fs::File::open("dfsdffd")?;
        // Ok("123123")
        let m = MyError::new(404u16, String::from("wochao"));
        let ret: Response<String> = Err(m);
        return ret;
    }

    pub async fn edit_job(
        req: web::Json<EditJobReq>,
        pool: web::Data<sqlx::MySqlPool>,
        client: web::Data<reqwest::Client>,
        request: actix_web::HttpRequest,
    ) -> Response<impl actix_web::Responder> {
        // TODO: jwt token verify.
        let auth_token = request
            .headers()
            .get("authorization")
            .ok_or(error!("no authorization header provide."))?
            .to_str()
            .unwrap_or("");

        let auth_payload = serde_json::from_str::<AuthorizationPayload>(
            auth_token
                .split(".")
                .collect::<Vec<&str>>()
                .get(1)
                .ok_or(error!("jwt token validate failed"))?,
        )?;

        let is_auditing = !req.permitted.is_empty() && !req.auditor_token.is_empty();

        let job_info = AuditInfo::get_job_info(&pool, req.job_id, &req.addr, req.r#type).await?;

        let users = cmdb_api::get_responsible_user_by_addr(&client, &req.addr).await?;

        let retttt = cmdb::query_user_cmdb("123").await?;

        let mut status_code = 0;

        let mut id_outers = (1, true, 1);

        if is_auditing {
            if req.permitted == "no" && req.reject_reason.is_empty() {
                return Err(MyError::new(
                    403,
                    format!("审核需要选择是否通过，若不通过需要阐述理由，请检查填写"),
                ));
            }

            if users
                .iter()
                .find(|v| v.id == auth_payload.user_id)
                .is_none()
            {
                return Err(MyError::new(403, format!("请联系对应系统负责人进行审核")));
            }

            status_code = if req.permitted == "yes" { 1 } else { 403 };

            /*
            ids = {};
            idOuters = {
                GroupID: 1,
                Root: true,
                UserID: 1
            }
             ;*/
        } else {
            // just edit
            if models::check_user_permissions(
                &pool,
                auth_payload.user_id,
                auth_payload.group_id,
                &req.addr,
            )
                .await?
                <= 0
            {
                // return Err(actix_web::error::ErrorForbidden("所属组无操作权限"));
                return MyError::new_result(403, "所属组无操作权限");
            }
        }

        cmdb_api::send_mail(
            &client,
            "",
            "",
            &CronJobDetail {
                name: "".to_owned(),
                created_username: "".to_string(),
            },
        )
            .await?;

        return Ok("");
    }
}
