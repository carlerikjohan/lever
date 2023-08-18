use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::features)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Feature {
    pub id: String,
    pub name: String,
    pub enabled: bool,
}