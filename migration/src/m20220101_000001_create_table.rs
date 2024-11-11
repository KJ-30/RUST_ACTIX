use sea_orm_migration::{prelude::*, schema::*};

use crate::m20241104_080800_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(
                        pk_auto(Post::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(string(Post::Title).string().not_null())
                    .col(string(Post::Uuid).unique_key().not_null())
                    .col(string(Post::Image).string())
                    .col(string(Post::Text).string().not_null())
                    .col(string(Post::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-post-users-Id")
                            .from(Post::Table, Post::Uuid)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
    Uuid,
    Image,
    UserId,
}
