#![allow(unreachable_pub)]

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "migrations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub version: i32,
    pub applied_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
