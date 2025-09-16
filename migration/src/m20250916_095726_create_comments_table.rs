use sea_orm_migration::{ prelude::*, schema::* };

use crate::{ m20220101_000001_create_table::User, m20250916_095631_create_posts_table::Post };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Comment::Table)
                .if_not_exists()
                .col(pk_auto(Comment::Id))
                .col(integer(Comment::UserId).not_null())
                .col(integer(Comment::PostId).not_null())
                .col(string(Comment::Text))
                .col(string(Comment::CreatedAt).date_time().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-comments-user-id")
                        .from(Comment::Table, Comment::UserId)
                        .to(User::Table, User::Id)
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-comments-post-id")
                        .from(Comment::Table, Comment::UserId)
                        .to(Post::Table, Post::Id)
                )
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Comment::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Comment {
    Table,
    Id,
    UserId,
    PostId,
    CreatedAt,
    Text,
}
