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
}
