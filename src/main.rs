use models::establish_connection;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let connection = establish_connection();
    let ret = models::get_config_version(&connection)?;

    println!("id: {}", ret.id);
    println!("ca: {}", ret.created_at);
    println!("ua: {}", ret.updated_at);

    Ok(())
}
