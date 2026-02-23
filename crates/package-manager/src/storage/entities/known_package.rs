#![allow(unreachable_pub)]

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "known_package")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub registry: String,
    pub repository: String,
    pub description: Option<String>,
    pub last_seen_at: String,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::known_package_tag::Entity")]
    KnownPackageTag,
}

impl Related<super::known_package_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::KnownPackageTag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
