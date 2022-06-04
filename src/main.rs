use std::io::Write;
use std::ops::{Deref, DerefMut};

use actix_web::{web, HttpServer};
use futures::StreamExt;
use rbatis::{
    crud::{CRUDTable, CRUD},
    rbatis::Rbatis,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{Column, FromRow, Row};

mod models;
use models::models as entity_models;

#[derive(Deserialize, Serialize, Debug)]
struct MatcherModel {
    pub id: i64,
    pub match_type: i32,
    pub match_value: String,
    pub match_target: i32,
    pub version: i32,
    pub deleted_at: Option<rbatis::DateTimeNative>,
    pub created_at: rbatis::DateTimeNative,
    pub updated_at: rbatis::DateTimeNative,
    pub upstream: String,
    pub match_method: String,
}

mod my_date_format {
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

mod my_date_format_optional {
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

#[derive(Debug)]
struct MyNaiveDateTime(NaiveDateTime);

#[derive(Debug, Deserialize, Serialize, FromRow)]
#[allow(dead_code)]
struct MatcherModelMacro {
    pub id: i64,
    pub match_type: i32,
    pub match_value: String,
    pub match_target: i32,
    pub version: i32,
    #[serde(with = "my_date_format_optional")]
    pub deleted_at: Option<NaiveDateTime>,
    #[serde(with = "my_date_format")]
    pub created_at: NaiveDateTime,
    #[serde(with = "my_date_format")]
    pub updated_at: NaiveDateTime,
    pub upstream: String,
    pub match_method: String,
}

// so, it turns out that introduce a new type to replace NaiveDateTime will NOT work.
// The only way to marshal date to string is to use a trait: serde(with = "module_name").

const FORMAT_STR: &str = "%Y-%m-%d %H:%M:%S";

impl Serialize for MyNaiveDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.format(FORMAT_STR);
        serializer.serialize_str(&s.to_string())
    }
}

impl<'de> Deserialize<'de> for MyNaiveDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT_STR)
            .map(|r| MyNaiveDateTime(r))
            .map_err(serde::de::Error::custom)
    }
}

impl Deref for MyNaiveDateTime {
    type Target = NaiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MyNaiveDateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CRUDTable for MatcherModel {
    fn table_name() -> String {
        return "matcher".to_owned();
    }
    fn table_columns() -> String {
        return "id,match_type,match_value,match_target,version,deleted_at,created_at,updated_at,upstream,match_method".to_string();
    }
}

async fn test_for_sqlx() -> anyhow::Result<sqlx::pool::Pool<sqlx::MySql>> {
    let pool = sqlx::MySqlPool::connect("mysql://root:root@192.168.150.73:3306/mgateway").await?;

    log::info!("==================== run in low level ===================");

    let ret = sqlx::query("SELECT * FROM matcher WHERE id > ? LIMIT 1")
        .bind("123")
        .fetch_one(&pool)
        .await?;

    for i in ret.columns() {
        log::info!("{}", i.name());
    }
    let v = ret.get::<String, &str>("match_value");
    log::info!("{}", v);

    log::info!("=================== response mappings ====================");

    let stream_ret = sqlx::query_as::<sqlx::MySql, MatcherModelMacro>(
        "SELECT * FROM matcher WHERE match_value LIKE ? LIMIT 10",
    )
    .bind("%/v1.0%")
    .fetch(&pool);

    stream_ret
        .for_each(|v| {
            if let Ok(mm) = v {
                log::info!("{:#?}", mm);
            }
            futures::future::ready(())
        })
        .await;

    log::info!("==================== use macro to check validity and mapping ===================");

    let mmret = sqlx::query_as::<sqlx::MySql, MatcherModelMacro>("SELECT * FROM matcher LIMIT 10")
        .bind("1")
        .fetch_all(&pool)
        .await?;

    let all_ai = sqlx::query_as::<_, entity_models::AuditInfo>("SELECT * FROM audit_info LIMIT 10")
        .bind("1")
        .fetch_all(&pool)
        .await?;

    log::info!("{:#?}", all_ai);

    // mmret
    //     .for_each(|v| {
    //         if let Ok(vv) = v {
    //             log::info!("{:#?}", vv);
    //         }
    //         futures::future::ready(())
    //     })
    //     .await;

    Ok(pool)
}

fn init_log() {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("0"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%3f"),
                record.level(),
                record.args()
            )
        })
        .filter_level(log::LevelFilter::Debug)
        .init();
}

async fn testtt(
    request: actix_web::HttpRequest,
    pool: web::Data<sqlx::MySqlPool>,
) -> actix_web::Result<impl actix_web::Responder> {
    let ret = models::models::AuditInfo::get_by_id(&pool, 10)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    models::models::AuditInfo::get_list_by_node_address(&pool, "123123");

    return Ok(actix_web::web::Json(ret));
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    init_log();

    log::info!("connecting to database");
    let pool = test_for_sqlx().await?;

    log::info!("==================== use rbatis for mapping ===================");

    let rb = Rbatis::new();
    rb.link("mysql://root:root@192.168.150.73:3306/mgateway")
        .await?;

    let wrapper = rb
        .new_wrapper()
        .eq("version", 1u32)
        .order_by(false, &["id"])
        .limit(1);
    let ret = rb.fetch_by_wrapper::<Option<MatcherModel>>(wrapper).await?;
    log::info!("wocao, zhewanyi xingma:{:#?}", ret);

    let wrapper = rb
        .new_wrapper()
        .gt("version", 1u32)
        .order_by(false, &["id"])
        .limit(10);

    let ret = rb.fetch_list_by_wrapper::<MatcherModel>(wrapper).await?;
    log::info!("{:#?}", ret);

    HttpServer::new(move || {
        actix_web::App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", actix_web::web::to(testtt))
    })
    .bind(("127.0.0.1", 8088))?
    .run()
    .await
    .map_err(|e| e.into())
}
