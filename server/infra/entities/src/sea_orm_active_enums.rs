//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "period_name")]
pub enum PeriodName {
    #[sea_orm(string_value = "start_at")]
    StartAt,
    #[sea_orm(string_value = "end_at")]
    EndAt,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "question_type")]
pub enum QuestionType {
    #[sea_orm(string_value = "text")]
    Text,
    #[sea_orm(string_value = "multiple")]
    Multiple,
    #[sea_orm(string_value = "single")]
    Single,
}
