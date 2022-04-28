use sea_orm::{Database, DatabaseConnection};

pub mod model;
pub mod schema;

pub async fn establish_connection() -> DatabaseConnection {
    let database_url = "mysql://root:root@192.168.0.3:3306/mgateway";
    let db = Database::connect(database_url).await.expect(&format!("failed to connect to: {}", database_url));

    return db;
}

// TODO: generate schemas of mysql
pub fn enable_tracing() {

}

