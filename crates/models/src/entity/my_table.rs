use std::fmt::Write;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "myTable")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub date: Option<String>,
    pub time: Option<String>,
}

pub enum TestModelForQuery {
    Table,
    Id,
    Name,
    Phone,
    Email,
    Date,
    Time,
}

impl Iden for TestModelForQuery {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "{}", match self {
            Self::Table => "myTable",
            Self::Id => "id",
            Self::Name => "name",
            Self::Phone => "phone",
            Self::Email => "email",
            Self::Date => "date",
            Self::Time => "time",
        }).unwrap()
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
