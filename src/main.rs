use std::io::Write;

use futures::StreamExt;
use rbatis::{
    crud::{CRUD, CRUDTable},
    rbatis::Rbatis,
};
use serde::{Deserialize, Serialize};
use sqlx::{Column, FromRow, Row};

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

#[derive(FromRow, Debug)]
#[allow(dead_code)]
struct MatcherModelSqlx {
    pub id: i64,
    pub match_type: i32,
    pub match_value: String,
    pub match_target: i32,
    pub version: i32,
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub upstream: String,
    pub match_method: String,
}

#[derive(Debug)]
#[allow(dead_code)]
struct MatcherModelMacro {
    pub id: i64,
    pub match_type: i32,
    pub match_value: String,
    pub match_target: i32,
    pub version: i32,
    pub deleted_at: Option<sqlx::types::time::PrimitiveDateTime>,
    pub created_at: sqlx::types::time::PrimitiveDateTime,
    pub updated_at: sqlx::types::time::PrimitiveDateTime,
    pub upstream: String,
    pub match_method: String,
}

impl CRUDTable for MatcherModel {
    fn table_name() -> String {
        return "matcher".to_owned();
    }
    fn table_columns() -> String {
        return "id,match_type,match_value,match_target,version,deleted_at,created_at,updated_at,upstream,match_method".to_string();
    }
}

async fn test_for_sqlx() -> anyhow::Result<()> {
    let pool = sqlx::MySqlPool::connect("mysql://root:root@192.168.150.73:3306/mgateway").await?;

    log::info!("==================== run in low level ===================");

    let ret = sqlx::query("SELECT * FROM matcher WHERE id > ? LIMIT 1").bind("123").fetch_one(&pool).await?;

    for i in ret.columns() {
        log::info!("{}", i.name());
    }
    let v = ret.get::<String, &str>("match_value");
    log::info!("{}", v);

    log::info!("=================== response mappings ====================");

    let stream_ret = sqlx::query_as::<_, MatcherModelSqlx>("SELECT * FROM matcher WHERE match_value LIKE ? LIMIT 10").bind("%/v1.0%").fetch(&pool);

    stream_ret.for_each(|v| {
        if let Ok(mm) = v {
            log::info!("{:#?}", mm);
        }
        futures::future::ready(())
    }).await;

    log::info!("==================== use macro to check validity and mapping ===================");

    let matchers = sqlx::query_as!(MatcherModelMacro, "SELECT * FROM matcher WHERE match_value LIKE ? LIMIT ?", "%/v1.0%", 10).fetch_all(&pool).await?;

    for m in matchers {
        log::info!("{:#?}", m);
    }

    Ok(())
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

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    init_log();

    log::info!("connecting to database");
    test_for_sqlx().await?;

    log::info!("==================== use rbatis for mapping ===================");

    let rb = Rbatis::new();
    rb.link("mysql://root:root@192.168.150.73:3306/mgateway").await?;

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

    Ok(())
}
