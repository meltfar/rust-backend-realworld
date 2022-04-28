#[macro_use]
extern crate diesel;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

pub mod model;
pub mod schema;

pub fn establish_connection() -> MysqlConnection {
    let database_url = "mysql://root:root@192.168.0.3:3306/mgateway";
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to...{}", database_url))
}

pub fn get_config_version(conn: &MysqlConnection) -> anyhow::Result<model::ConfigVersion> {
    use schema::config_version::dsl::*;

    let mut ret = config_version
        .filter(id.eq(1))
        .load::<model::ConfigVersion>(conn)?;
    dbg!(&ret);
    if ret.len() > 0 {
        Ok(ret.remove(0))
    } else {
        Err(anyhow::anyhow!("no config version found"))
    }
}
