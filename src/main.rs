use std::{io::Write};

use rbatis::{
    crud::{CRUDTable, CRUD},
    rbatis::Rbatis,
};
use serde::{Deserialize, Serialize};

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

impl CRUDTable for MatcherModel {
    fn table_columns() -> String {
        return "id,match_type,match_value,match_target,version,deleted_at,created_at,updated_at,upstream,match_method".to_string();
    }
    fn table_name() -> String {
        return "matcher".to_owned();
    }
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("n"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%3f"),
                record.level(),
                record.args()
            )
        })
        .filter_level(log::LevelFilter::Debug)
        .init();

    log::info!("connecting to database");

    // let connection = establish_connection().await;

    // let ret = models::test(&connection).await?;
    // if let Some(m) = ret {
    //     log::info!("{:#?}", m);
    // }

    let rb = Rbatis::new();
    rb.link("mysql://root:root@127.0.0.1:3306/mgateway").await?;

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

    Ok(())
}
