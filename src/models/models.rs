pub mod models {
    use sea_query::{ConditionalStatement, enum_def};
    use serde::{Deserialize, Serialize};
    use sqlx::{FromRow, types::chrono};

    pub mod my_date_format {
        use chrono::NaiveDateTime;
        use serde::{self, Deserialize, Deserializer, Serializer};

        const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

        pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
        {
            let s = format!("{}", date.format(FORMAT));
            serializer.serialize_str(&s)
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
            where
                D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
        }
    }

    pub mod my_date_format_optional {
        use chrono::NaiveDateTime;
        use serde::{self, Deserialize, Deserializer, Serializer};

        const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

        pub fn serialize<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
        {
            if let Some(s) = date {
                let s = format!("{}", s.format(FORMAT));
                serializer.serialize_str(&s)
            } else {
                serializer.serialize_str("")
            }
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
            where
                D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            if s == "" {
                return Ok(None);
            }
            NaiveDateTime::parse_from_str(&s, FORMAT)
                .map(|v| Some(v))
                .map_err(serde::de::Error::custom)
        }
    }

    #[derive(Debug, Deserialize, Serialize, FromRow)]
    #[serde(rename_all = "camelCase")]
    #[enum_def(prefix = "", suffix = "Def")]
    #[allow(dead_code)]
    pub struct AuditInfo {
        pub id: i32,
        pub job_id: i32,
        pub node_address: String,
        pub r#type: i32,
        pub real_auditor: String,
        pub permitted: String,
        pub reason: Option<String>,
        pub candidate_auditor: String,
        #[serde(with = "my_date_format")]
        pub created_at: chrono::NaiveDateTime,
        #[serde(with = "my_date_format")]
        pub updated_at: chrono::NaiveDateTime,
        pub raw_body: Option<String>,
        pub real_auditor_name: String,
        pub audit_group_id: i32,
    }

    #[derive(serde::Deserialize, serde::Serialize)]
    pub struct JobInfoResponse {
        pub data: Vec<AuditInfo>,
        pub total: u32,
    }

    impl AuditInfo {
        pub async fn _get_by_id(
            pool: &sqlx::Pool<sqlx::MySql>,
            id: i64,
        ) -> Result<AuditInfo, sqlx::Error> {
            return sqlx::query_as::<_, AuditInfo>("SELECT * FROM audit_info WHERE id = ? LIMIT 1")
                .bind(id)
                .fetch_one(pool)
                .await;
        }

        pub async fn _get_list_by_node_address<T>(
            pool: &sqlx::Pool<sqlx::MySql>,
            addr: T,
        ) -> Result<Vec<AuditInfo>, sqlx::Error>
            where
                T: AsRef<str> + Sync,
        {
            return sqlx::query_as::<_, AuditInfo>(
                "SELECT * FROM audit_info WHERE node_address = ?",
            )
                .bind(addr.as_ref())
                .fetch_all(pool)
                .await;
        }

        pub async fn create_job_info(
            pool: &sqlx::MySqlPool,
            job_id: u32,
            address: &str,
            job_type: i32,
            auditor: &str,
            group_id: u32,
        ) -> Result<(), sqlx::Error> {
            sqlx::query_as::<_, (i32, )>("INSERT INTO audit_info (job_id, node_address, type, candidate_auditor, audit_group_id) VALUES (?, ?, ?, ?, ?)").bind(job_id).bind(address).bind(job_type).bind(auditor).bind(group_id).fetch_all(pool).await.map(|_| ())
        }

        pub async fn get_job_info<T>(
            pool: &sqlx::MySqlPool,
            job_id: u32,
            addr: T,
            job_type: i32,
        ) -> Result<AuditInfo, sqlx::Error>
            where
                T: AsRef<str> + Sync,
        {
            return sqlx::query_as::<_, AuditInfo>(
                "SELECT * FROM audit_info WHERE job_id = ? AND node_address = ? AND type = ?",
            )
                .bind(job_id)
                .bind(addr.as_ref())
                .bind(job_type)
                .fetch_one(pool)
                .await;
        }

        pub async fn update_job_info(
            pool: &sqlx::MySqlPool,
            job_id: i32,
            address: &str,
            job_type: i32,
            auditor: &str,
            auditor_name: &str,
            permitted: &str,
            reason: &str,
        ) -> Result<(), sqlx::Error> {
            sqlx::query_as::<_, (i32, )>("UPDATE audit_info SET real_auditor = ?, permitted = ?, reason = ?, real_auditor_name = ? WHERE job_id = ? AND node_address = ? AND type = ?").bind(auditor).bind(permitted).bind(reason).bind(auditor_name).bind(job_id).bind(address).bind(job_type).fetch_all(pool).await.map(|_| ())
        }

        pub async fn get_job_info_list(pool: &sqlx::MySqlPool, phone: &str, group_id_list: Vec<u32>, job_type: i64, page: i64, limit: i64) -> Result<JobInfoResponse, sqlx::Error> {
            let mut cond = sea_query::Cond::all().add(sea_query::Cond::any().add(sea_query::Expr::col(AuditInfoDef::AuditGroupId).is_in(group_id_list)).add(sea_query::Expr::col(AuditInfoDef::CandidateAuditor).like(format!("%{}%", phone))));
            let mut query = sea_query::Query::select().from(AuditInfoDef::Table).limit(limit as u64).offset(((page - 1) * limit) as u64).to_owned();

            let mut sql = String::from("SELECT * FROM WHERE audit_group_id IN (?) OR candidate_auditor LIKE ? ");
            match job_type {
                1 => {
                    sql.push_str("AND WHERE permitted = 'yes'");
                    cond = cond.add(sea_query::Expr::col(AuditInfoDef::Permitted).eq("yes"));
                }
                2 => {
                    sql.push_str("AND WHERE permitted = 'no'");
                    cond = cond.add(sea_query::Expr::col(AuditInfoDef::Permitted).eq("no"));
                }
                3 => {}
                0 | _ => {
                    sql.push_str("AND WHERE permitted = ''");
                    cond = cond.add(sea_query::Expr::col(AuditInfoDef::Permitted).eq(""));
                }
            }
            sql.push_str(" LIMIT ? OFFSET ?");

            query.cond_where(cond);

            // let _ret = sqlx::query_as::<_, AuditInfo>(sql.as_str()).fetch_all(pool).await?;
            let out = query.clone().expr(sea_query::Func::count(sea_query::Expr::value(1u32))).to_string(sea_query::MysqlQueryBuilder);
            log::info!("{}", out);
            // TODO: expand all columns into this array, also try to find out is there a method to pass a '*' in it.
            log::info!("{}", query.columns([AuditInfoDef::Id]).to_string(sea_query::MysqlQueryBuilder));

            Ok(JobInfoResponse {
                data: vec![],
                total: 0,
            })
        }
    }

    // Users
    pub async fn check_user_permissions(
        pool: &sqlx::MySqlPool,
        user_id: i64,
        group_id: i64,
        address: &str,
    ) -> sqlx::Result<i8> {
        if group_id == 1 {
            return Ok(1);
        }

        if address.is_empty() {
            return Ok(0);
        }

        let count = sqlx::query_as::<_, (i64, )>(
            "SELECT COUNT(1) as id_num FROM users WHERE id = ? AND group_id = ?",
        )
            .bind(user_id)
            .bind(group_id)
            .fetch_one(pool)
            .await?;

        if count.0 <= 0 {
            return Ok(0);
        }

        let count = sqlx::query_as::<_, (i64, )>(
            "SELECT COUNT(1) as id_num FROM nodes WHERE group_id = ? AND addr = ?",
        )
            .bind(group_id)
            .bind(address)
            .fetch_one(pool)
            .await?;

        Ok(if count.0 <= 0 { 0 } else { 1 })
    }
}
