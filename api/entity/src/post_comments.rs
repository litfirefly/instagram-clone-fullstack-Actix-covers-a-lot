//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "post_comments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(Some(16)))")]
    pub user_id: Vec<u8>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(Some(16)))")]
    pub post_id: Vec<u8>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(Some(16)))", nullable)]
    pub parent_id: Option<Vec<u8>>,
    #[sea_orm(column_type = "Text")]
    pub text: String,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::posts::Entity",
        from = "Column::PostId",
        to = "super::posts::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Posts,
}

impl Related<super::posts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Posts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}