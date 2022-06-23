use std::io::Write;
use std::ops::{Deref, DerefMut};

use actix_web::{HttpServer, web};
use futures::StreamExt;
use rbatis::{
    crud::{CRUD, CRUDTable},
    rbatis::Rbatis,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::{Column, FromRow, Row};
use sqlx::types::chrono::NaiveDateTime;

use controllers::job_controller::job_controller;

mod controllers;
mod models;
mod rapi;
mod rpc;
mod utils;

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

async fn test_for_sqlx(database_url: &str) -> anyhow::Result<sqlx::pool::Pool<sqlx::MySql>> {
    let pool = sqlx::MySqlPool::connect(database_url).await?;

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

    let ret = sqlx::query_as!(MatcherModelMacro, "SELECT * FROM matcher WHERE id > ?", 32i32).fetch_all(&pool).await?;
    log::info!("{:#?}", ret);
    Ok(pool)
}

fn init_log(env: String) {
    let log_level = if env.to_lowercase() == "dev" {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
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
        .filter_level(log_level)
        .init();
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let current_env = std::env::var("RUNTIME_ENV").unwrap_or("dev".to_string());
    let database_url = std::env::var("DATABASE_URL").unwrap();

    init_log(current_env);

    log::info!("connecting to database");
    let pool = test_for_sqlx(&database_url).await?;

    log::info!("==================== use rbatis for mapping ===================");

    let rb = Rbatis::new();
    rb.link(&database_url).await?;

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

    let rb = std::sync::Arc::new(rb);

    HttpServer::new(move || {
        let json_config = web::JsonConfig::default().error_handler(|err, _req| {
            actix_web::error::InternalError::from_response(
                err,
                actix_web::HttpResponse::InternalServerError().finish(),
            ).into()
        });
        actix_web::App::new()
            .app_data(json_config)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(rb.clone()))
            .app_data(web::Data::new(reqwest::Client::new()))
            .route("/editJob", web::to(job_controller::edit_job))
            .route("/err", web::get().to(job_controller::error_return))
    })
        .bind(("127.0.0.1", 8088))?
        .run()
        .await
        .map_err(|e| e.into())
}
