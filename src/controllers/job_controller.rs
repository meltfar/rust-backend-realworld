pub mod job_controller {
    use actix_web::web;

    // use crate::models::models::models;
    use crate::error;
    use crate::utils::MyError;
    use crate::{
        models::models::models, models::AuditInfo, models::JobDetail, models::TimeArgs,
        rapi::cmdb_api, rapi::jiacron,
    };

    // use models as entity_models;

    // use models as entity_models;

    type Response<T> = actix_web::Result<T, MyError>;

    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EditJobReq {
        permitted: String,
        auditor_token: String,
        job_id: u32,
        addr: String,
        r#type: i32,
        reject_reason: String,
        group_id: u32,
        second: String,
        minute: String,
        hour: String,
        day: String,
        weekday: String,
        month: String,
        #[serde(flatten)]
        job_content: JobDetail,
    }

    #[derive(serde::Deserialize)]
    #[allow(unused)]
    struct AuthorizationPayload {
        exp: i32,
        #[serde(rename = "Version")]
        version: i32,
        #[serde(rename = "UserID")]
        user_id: u32,
        #[serde(rename = "Mail")]
        mail: String,
        #[serde(rename = "Username")]
        username: String,
        #[serde(rename = "GroupID")]
        group_id: u32,
        #[serde(rename = "Root")]
        root: bool,
    }

    #[derive(serde::Deserialize)]
    struct CMDBAuditUserInfo<'a> {
        id: u32,
        phone: &'a str,
        name: &'a str,
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
        let req = req.into_inner();

        let is_auditing = !req.permitted.is_empty() && !req.auditor_token.is_empty();

        let job_info = AuditInfo::get_job_info(&pool, req.job_id, &req.addr, req.r#type).await;

        let users = cmdb_api::get_responsible_user_by_addr(&client, req.group_id).await?;

        let mut status_code = 0;

        let mut id_outers = (1u32, true, 1u32);

        let mut ids: Option<(u32, u32, String)> = None;

        if is_auditing {
            if req.permitted.is_empty() {
                return Err(MyError::new(
                    400,
                    format!("审核要选择是否通过，若不同过需要阐述理由"),
                ));
            }
            if req.permitted == "no" && req.reject_reason.is_empty() {
                return Err(MyError::new(
                    403,
                    format!("审核需要选择是否通过，若不通过需要阐述理由，请检查填写"),
                ));
            }

            if req.auditor_token.is_empty() {
                return Err(MyError::from_string("no auditor token provided"));
            }
            let auditor_token = req
                .auditor_token
                .split(".")
                .skip(1)
                .next()
                .ok_or(MyError::from_string("auditor token not valid"))?;

            let auditor_user_info = serde_json::from_str::<CMDBAuditUserInfo>(auditor_token)?;

            if users
                .iter()
                .find(|v| v.id == auditor_user_info.id)
                .is_none()
            {
                return Err(MyError::new(403, format!("请联系对应系统负责人进行审核")));
            }

            status_code = if req.permitted == "yes" { 1 } else { 403 };
        } else {
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
            // just edit
            if models::check_user_permissions(
                &pool,
                auth_payload.user_id.into(),
                auth_payload.group_id.into(),
                &req.addr,
            )
            .await?
                <= 0
            {
                // return Err(actix_web::error::ErrorForbidden("所属组无操作权限"));
                return MyError::new_result(403, "所属组无操作权限");
            }

            id_outers = (
                auth_payload.group_id,
                auth_payload.root,
                auth_payload.user_id,
            );
            ids = Some((auth_payload.group_id, auth_payload.user_id, auth_payload.username))
        }

        // TODO: check job type here, then call apis respectively.
        let mut created_username = "".to_string();
        let mut job_name = "".to_string();
        let mut created_job_id = 0u32;
        {
            let mut jd = JobDetail {
                id: req.job_id,
                status: status_code,
                time_args: TimeArgs {
                    weekday: req.weekday,
                    month: req.month,
                    day: req.day,
                    hour: req.hour,
                    minute: req.minute,
                    second: req.second,
                },
                ..req.job_content
            };
            if ids.is_some() {
                let ids = ids.unwrap();
                jd.group_id = ids.0;
                jd.updated_user_id = ids.1 as i32;
                jd.created_user_id = ids.1 as i32;
                jd.updated_username = ids.2.clone();
                jd.created_username = ids.2;
            }

            let ret = jiacron::edit_job_call(
                &client,
                jiacron::EditJobRequest {
                    user_id: id_outers.2,
                    root: id_outers.1,
                    group_id: id_outers.0,
                    job: &jd,
                },
                &req.addr,
            )
            .await?;
            created_username = ret.created_username;
            job_name = ret.name;
            created_job_id = ret.id;
        }

        // when run to here, the result of remote api call was success.
        if !is_auditing && job_info.is_err() {
            let users_str = users.iter().map(|f| f.phone.clone()).reduce(|n, o|{
                if o.len() <=0 {
                    n
                } else{
                    format!("{}-{}", o, n)
                }
            }).ok_or(MyError::from_string("未能保存审核用户"))?;
            AuditInfo::create_job_info(&pool, created_job_id, &req.addr, req.r#type, &users_str, req.group_id).await?;
        }

        let cc = client.clone();
        actix_web::rt::spawn(async move {
            let cjd = CronJobDetail {
                name: "".to_owned(),
                created_username: "".to_string(),
            };
            cmdb_api::send_mail(cc, "", "", cjd)
        });
        // cmdb_api::send_mail(
        //     &client,
        //     "",
        //     "",
        //     &CronJobDetail {
        //         name: "".to_owned(),
        //         created_username: "".to_string(),
        //     },
        // )
        // .await?;

        return Ok("");
    }
}
