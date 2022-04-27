extern crate anyhow;
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

pub fn test_11(conn: &MysqlConnection) -> anyhow::Result<Vec<model::ConfigVersion>> {
    use schema::config_version::dsl::*;

    let ret = config_version
        .filter(id.eq(1))
        .load::<model::ConfigVersion>(conn)?;

    dbg!(&ret);

    Ok(ret)
}
