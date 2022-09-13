use futures::executor::block_on;
use sea_orm::{Database, DbErr};

const DATABASE_URL: &str = "mysql://root:@localhost:3306/bakeries_db";

mod db_utils;
mod entities;

use entities::{prelude::*, *};
use sea_orm::*;

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    //database creation
    // db_utils::drop_or_create_database(db).await?;

    //for insert operation
    let happy_bakery = bakery::ActiveModel {
        name: ActiveValue::Set("Happy Bakery".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let result = Bakery::insert(happy_bakery).exec(&db).await?;

    //update operation
    let sad_bakery = bakery::ActiveModel {
        id: ActiveValue::Set(result.last_insert_id),
        name: ActiveValue::Set("Sad Bakery".to_owned()),
        ..Default::default()
    };
    Bakery::update(sad_bakery.clone()).exec(&db).await?;

    //insert child table record
    let john = baker::ActiveModel {
        name: ActiveValue::Set("John".to_owned()),
        bakery_id: ActiveValue::Set(result.last_insert_id),
        ..Default::default()
    };
    Baker::insert(john).exec(&db).await?;

    // //Query!
    // // Finding all is built-in
    // let bakeries: Vec<bakery::Model> = Bakery::find().all(&db).await?;
    // assert_eq!(bakeries.len(), 1);

    // // Finding by id is built-in
    // let sad_bakery: Option<bakery::Model> = Bakery::find_by_id(1).one(&db).await?;
    // assert_eq!(sad_bakery.unwrap().name, "Sad Bakery");

    // // Finding by arbitrary column with `filter()`
    // let sad_bakery: Option<bakery::Model> = Bakery::find()
    //     .filter(bakery::Column::Name.eq("Sad Bakery"))
    //     .one(&db)
    //     .await?;
    // assert_eq!(sad_bakery.unwrap().id, 1);

    // //Delete!
    // let john = baker::ActiveModel {
    //     id: ActiveValue::Set(1), // The primary key must be set
    //     ..Default::default()
    // };
    // john.delete(&db).await?;

    // let sad_bakery = bakery::ActiveModel {
    //     id: ActiveValue::Set(1), // The primary key must be set
    //     ..Default::default()
    // };
    // sad_bakery.delete(&db).await?;

    // let bakeries: Vec<bakery::Model> = Bakery::find().all(&db).await?;
    // assert!(bakeries.is_empty());
    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
