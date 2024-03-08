pub mod competitions;
pub mod participants;
pub mod registrations;
pub mod groups;

pub type Database = sqlx::Postgres;
pub type DatabasePool = sqlx::Pool<Database>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "gender")]
#[sqlx(rename_all = "lowercase")]
pub enum Gender {
    Female,
    Male,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "stroke")]
#[sqlx(rename_all = "lowercase")]
pub enum Stroke {
    Butterfly,
    Back,
    Breast,
    Freestyle,
}
