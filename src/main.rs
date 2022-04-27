use models::establish_connection;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let connection = establish_connection();
    let ret = models::test_11(&connection)?;

    for cv in ret {
        println!("id: {}", cv.id);
        println!("ca: {}", cv.created_at);
        println!("ua: {}", cv.updated_at);
    }

    Ok(())
}
