use sea_orm::{ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};

pub mod entity;

pub async fn establish_connection() -> DatabaseConnection {
    let database_url = "mysql://root:root@127.0.0.1:3306/mgateway";
    let db = Database::connect(database_url)
        .await
        .expect(&format!("failed to connect to: {}", database_url));

    return db;
}

pub async fn test(db: &DatabaseConnection) -> anyhow::Result<Option<entity::module::Model>> {
    let ret = entity::module::Entity::find_by_id(1).one(db).await?;
    let rr = entity::module::Entity::find()
        .filter(entity::module::Column::ModuleName.like("%w%"))
        .one(db)
        .await?;
    if let Some(mt) = rr {
        println!("with www: {:#?}", mt);
    }

    let joret = entity::matcher::Entity::find()
        .join(
            sea_orm::JoinType::LeftJoin,
            entity::matcher::Entity::belongs_to(entity::matcher_module::Entity)
                .from(entity::matcher::Column::Id)
                .to(entity::matcher_module::Column::MatcherId)
                .into(),
        )
        .limit(10)
        .filter(entity::matcher_module::Column::SortIndex.gt(2))
        .all(db)
        .await?;
    println!("{:#?}", joret);

    // test for sql builder
    Ok(ret)
}

pub fn enable_tracing() {}
