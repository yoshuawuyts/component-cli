#![allow(unreachable_pub)]

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "known_package_tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub known_package_id: i64,
    pub tag: String,
    pub tag_type: String,
    pub last_seen_at: String,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::known_package::Entity",
        from = "Column::KnownPackageId",
        to = "super::known_package::Column::Id"
    )]
    KnownPackage,
}

impl Related<super::known_package::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::KnownPackage.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
