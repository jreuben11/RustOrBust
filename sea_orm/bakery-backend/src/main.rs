use futures::executor::block_on;
use sea_orm::{Database, DbErr};

const DATABASE_URL: &str = "mysql://root:password@localhost:3306";
const DB_NAME: &str = "bakeries_db";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}