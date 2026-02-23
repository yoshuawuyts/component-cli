#![allow(unreachable_pub)]

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "wit_interface")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub wit_text: String,
    pub package_name: Option<String>,
    pub world_name: Option<String>,
    pub import_count: i32,
    pub export_count: i32,
    pub created_at: String,
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

/// Many-to-many: WitInterface <-> Image through ImageWitInterface
impl Related<super::image::Entity> for Entity {
    fn to() -> RelationDef {
        super::image_wit_interface::Relation::Image.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::image_wit_interface::Relation::WitInterface
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
