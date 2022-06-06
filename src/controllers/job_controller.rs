pub mod job_controller {
    // use crate::models::models::models;
    use crate::models::AuditInfo;
    use actix_web::web;
    // use models as entity_models;

    pub async fn testtt(
        request: actix_web::HttpRequest,
        pool: web::Data<sqlx::MySqlPool>,
    ) -> actix_web::Result<impl actix_web::Responder> {
        let ret = AuditInfo::get_by_id(&pool, 10)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        return Ok(actix_web::web::Json(ret));
    }

    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EditJobReq {
        permitted: bool,
        auditor_token: String,
        job_id: i64,
        addr: String,
        r#type: i32,
    }

    pub async fn edit_job(
        req: web::Json<EditJobReq>,
        pool: web::Data<sqlx::MySqlPool>,
    ) -> actix_web::Result<impl actix_web::Responder> {
        let is_auditing = req.permitted && !req.auditor_token.is_empty();

        let ai = AuditInfo::get_job_info(&pool, req.job_id, &req.addr, req.r#type).await.map_err(actix_web::error::ErrorInternalServerError)?;

        return Ok("");
    }
}
