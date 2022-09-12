use futures::executor::block_on;
use sea_orm::{Database, DbErr};

const DATABASE_URL: &str = "mysql://root:@localhost:3306";

mod db_utils;

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    db_utils::drop_or_create_database(db).await?;
    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
