#![allow(unreachable_pub)]

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "image")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub ref_registry: String,
    pub ref_repository: String,
    pub ref_mirror_registry: Option<String>,
    pub ref_tag: Option<String>,
    pub ref_digest: Option<String>,
    pub manifest: String,
    pub size_on_disk: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::image_wit_interface::Entity")]
    ImageWitInterface,
}

impl Related<super::image_wit_interface::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ImageWitInterface.def()
    }
}

/// Many-to-many: Image <-> WitInterface through ImageWitInterface
impl Related<super::wit_interface::Entity> for Entity {
    fn to() -> RelationDef {
        super::image_wit_interface::Relation::WitInterface.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::image_wit_interface::Relation::Image.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
