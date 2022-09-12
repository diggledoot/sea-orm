use sea_orm::{DatabaseConnection, ConnectionTrait, Statement, DbErr, Database};

use crate::DATABASE_URL;

pub async fn drop_or_create_database(db:DatabaseConnection)->Result<(),DbErr>{
    let db_name = "bakeries_db";

    db.execute(Statement::from_string(
        db.get_database_backend(),
        format!("DROP DATABASE IF EXISTS `{}`",db_name)
    ))
    .await?;

    db.execute(Statement::from_string(
        db.get_database_backend(),
        format!("CREATE DATABASE IF NOT EXISTS `{}`",db_name)
    ))
    .await?;
    
    let url = format!("{}/{}", DATABASE_URL, db_name);
    
    Database::connect(&url).await?;
    
    Ok(())
}