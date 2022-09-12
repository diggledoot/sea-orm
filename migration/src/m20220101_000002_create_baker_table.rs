// m20220101_000002_create_baker_table.rs

use sea_orm_migration::prelude::*;

use super::m20220101_000001_create_bakery_table::Bakery;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20220101_000002_create_baker_table" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Baker table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Baker::Table)
                    .col(
                        ColumnDef::new(Baker::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Baker::Name).string().not_null())
                    .col(ColumnDef::new(Baker::ContactDetails).json())
                    .col(ColumnDef::new(Baker::BakeryId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-baker-bakery_id")
                            .from(Baker::Table, Baker::BakeryId)
                            .to(Bakery::Table, Bakery::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Baker table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Baker::Table).to_owned())
            .await
    }
}

// For ease of access
#[derive(Iden)]
pub enum Baker {
    Table,
    Id,
    Name,
    ContactDetails,
    BakeryId,
}