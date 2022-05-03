use sea_orm::{ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter};

pub mod entity;

pub async fn establish_connection() -> DatabaseConnection {
    let database_url = "mysql://root:root@192.168.40.128:3306/gateway";
    let db = Database::connect(database_url).await.expect(&format!("failed to connect to: {}", database_url));

    let r = entity::my_table::Entity::find().one(&db).await.unwrap();
    println!("{:#?}", r);

    return db;
}

pub async fn test(db: &DatabaseConnection) -> anyhow::Result<Option<entity::my_table::Model>> {
    let ret = entity::my_table::Entity::find_by_id(1).one(db).await?;
    let rr = entity::my_table::Entity::find().filter(entity::my_table::Column::Name.like("%w%")).one(db).await?;
    if let Some(mt) = rr {
        println!("with www: {:#?}", mt);
    }

    // test for sql builder
    let query_ret = sea_orm::sea_query::Query::select().from(entity::my_table::TestModelForQuery::Table).and_where(sea_orm::sea_query::Expr::col(entity::my_table::TestModelForQuery::Id).gt(1)).build(sea_orm::sea_query::MysqlQueryBuilder);

    println!("{}", query_ret.0);
    println!("{:#?}", query_ret.1);

    Ok(ret)
}

pub fn enable_tracing() {}
