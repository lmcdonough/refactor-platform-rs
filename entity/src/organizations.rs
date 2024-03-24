//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, ToSchema, Serialize, Deserialize)]
#[schema(as = entity::organizations::Model)] // OpenAPI schema
#[sea_orm(schema_name = "refactor_platform", table_name = "organizations")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    #[sea_orm(unique)]
    pub external_id: Uuid,
    pub name: Option<String>,
    pub logo: Option<String>,
    #[schema(value_type = String, format = DateTime)] // Applies to OpenAPI schema
    pub created_at: DateTimeWithTimeZone,
    #[schema(value_type = String, format = DateTime)] // Applies to OpenAPI schema
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::coaching_relationships::Entity")]
    CoachingRelationships,
}

impl Related<super::coaching_relationships::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CoachingRelationships.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
