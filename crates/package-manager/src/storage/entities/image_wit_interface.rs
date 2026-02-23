#![allow(unreachable_pub)]

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "image_wit_interface")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub image_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub wit_interface_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::image::Entity",
        from = "Column::ImageId",
        to = "super::image::Column::Id"
    )]
    Image,
    #[sea_orm(
        belongs_to = "super::wit_interface::Entity",
        from = "Column::WitInterfaceId",
        to = "super::wit_interface::Column::Id"
    )]
    WitInterface,
}

impl Related<super::image::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Image.def()
    }
}

impl Related<super::wit_interface::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WitInterface.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
