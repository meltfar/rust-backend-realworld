use sea_orm::{Database, DatabaseConnection, EntityTrait};

pub mod entity;

pub async fn establish_connection() -> DatabaseConnection {
    let database_url = "mysql://root:root@192.168.40.128:3306/gateway";
    let db = Database::connect(database_url).await.expect(&format!("failed to connect to: {}", database_url));

    return db;
}

pub async fn test(db: &DatabaseConnection) -> anyhow::Result<Option<entity::my_table::Model>> {
    let ret = entity::my_table::Entity::find_by_id(1).one(db).await?;
    Ok(ret)
}

pub fn enable_tracing() {}
