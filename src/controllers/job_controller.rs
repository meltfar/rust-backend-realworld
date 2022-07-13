pub mod job_controller {
    use actix_web::web;

    use crate::{
        models::AuditInfo, models::JobDetail, models::models::models, models::TimeArgs,
        rapi::cmdb_api, rapi::jiacron,
    };
    // use crate::models::models::models;
    use crate::error;
    use crate::utils::MyError;

// use models as entity_models;

    // use models as entity_models;

    #[derive(serde::Serialize)]
    struct CommonResponse<'a> {
        code: i32,
        data: &'a str,
    }

    type Response<T> = actix_web::Result<T, MyError>;

    #[derive(serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EditJobReq {
        permitted: String,
        auditor_token: String,
        job_id: u32,
        addr: String,
        r#type: i32,
        reject_reason: String,
        audit_group_id: u32,
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
        // Ok("123123")
        let m = MyError::new(404u16, String::from("wochao"));
        let ret: Response<String> = Err(m);
        return ret;
    }

    fn generate_callback_url(addr: &str, id: u32, r#type: i32) -> String {
        let input = format!("http://{}/#/edit/{}?id={}&addr={}&tabKey={}&mode=audit&token=", "app.c.vip.migu.cn", if r#type == 1 { "crontab_job" } else { "daemon_job" }, id, addr, 1, ).as_str();
        let callback = urlencoding::encode(input);
        return callback.into_owned();
    }

    fn get_auditor_info_by_header(auditor_token: &str) -> Result<CMDBAuditUserInfo, MyError> {
        let auditor_token = auditor_token
            .split(".")
            .skip(1)
            .next()
            .ok_or(MyError::from_string("auditor token not valid"))?;

        let decoded_token = base64::decode(auditor_token)?;
        let mut decoded_token_str = String::new();
        unsafe {
            decoded_token_str = String::from_utf8_unchecked(decoded_token);
        }
        let auditor_user_info = serde_json::from_str::<CMDBAuditUserInfo>(decoded_token_str.as_ref())?;
        Ok(auditor_user_info)
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

        let users = cmdb_api::get_responsible_user_by_addr(&client, req.audit_group_id).await?;

        let mut id_outers = (1u32, true, 1u32);

        let mut prepared_job = JobDetail {
            id: req.job_id,
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

        let auditor_user_info = get_auditor_info_by_header(req.auditor_token.as_str());

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

            if req.auditor_token.is_empty() || auditor_user_info.is_err() {
                return Err(MyError::from_string("no auditor token provided"));
            }

            // let uid = auditor_user_info.unwrap().id;
            let uid = auditor_user_info.as_ref().unwrap().id;

            if users
                .iter()
                .find(|v| v.id == uid)
                .is_none()
            {
                return Err(MyError::new(403, format!("请联系对应系统负责人进行审核")));
            }

            prepared_job.status = if req.permitted == "yes" { 1 } else { 403 }
        } else {
            // just editing
            let auth_token = request
                .headers()
                .get("authorization")
                .ok_or(error!("no authorization header provide."))?
                .to_str()
                .unwrap_or("");

            let auth_token_decoded = base64::decode(auth_token
                .split(".")
                .collect::<Vec<&str>>()
                .get(1)
                .ok_or(error!("jwt token validate failed"))?)?;
            let auth_token_decoded = String::from_utf8(auth_token_decoded)?;
            let auth_payload = serde_json::from_str::<AuthorizationPayload>(auth_token_decoded.as_str())?;

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
            prepared_job.group_id = auth_payload.group_id;
            prepared_job.updated_user_id = auth_payload.user_id as i32;
            prepared_job.updated_username = auth_payload.username.clone();
            prepared_job.created_user_id = auth_payload.user_id as i32;
            prepared_job.created_username = auth_payload.username;
        }

        // check job type here, then call apis respectively.
        // let mut created_username = "".to_string();
        // let mut job_name = "".to_string();
        // let mut created_job_id = 0u32;
        let mut ret_job_detail = JobDetail::default();
        {
            ret_job_detail = jiacron::edit_job_call(
                &client,
                jiacron::EditJobRequest {
                    user_id: id_outers.2,
                    root: id_outers.1,
                    group_id: id_outers.0,
                    job: &prepared_job,
                },
                &req.addr,
            )
                .await?;
            // created_username = ret.created_username;
            // job_name = ret.name;
            // created_job_id = ret.id;
        }

        // when run to here, the result of remote api call was success.
        if !is_auditing && job_info.is_err() {
            let users_str = users
                .into_iter()
                .map(|f| f.phone)
                .collect::<Vec<String>>().join("-");
            AuditInfo::create_job_info(
                &pool,
                ret_job_detail.id,
                &req.addr,
                req.r#type,
                &users_str,
                req.audit_group_id,
            ).await?;

            let cc = client.clone();
            let ur = generate_callback_url(req.addr.as_ref(), ret_job_detail.id, req.r#type);
            actix_web::rt::spawn(async move {
                let cjd = CronJobDetail {
                    name: ret_job_detail.name.clone(),
                    created_username: ret_job_detail.created_username.clone(),
                };
                let _ = cmdb_api::send_mail(cc, ur.as_str(), "", &cjd).await;
                cmdb_api::send_sms(client, "", cjd).await
            });
        } else if is_auditing && job_info.is_ok() {
            let ji = job_info.unwrap();
            if ji.permitted == "yes" {
                return Err(MyError::from_string("已经通过的无需再次审核"));
            }
            let aui = auditor_user_info.unwrap();
            // update job info
            AuditInfo::update_job_info(
                &pool,
                ji.job_id,
                &ji.node_address,
                1, // TODO: job_type
                aui.phone,
                aui.name,
                &req.permitted,
                &req.reject_reason,
                // serde_json::to_string(&req)?.as_str(),
            ).await?;
        }

        return Ok(web::Json(CommonResponse {
            code: 200,
            data: "over",
        }));
    }
}
