use models::establish_connection;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let connection = establish_connection().await?;

    Ok(())
}
