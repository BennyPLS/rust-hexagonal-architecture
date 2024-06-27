use diesel::{Identifiable, Insertable, Queryable};

#[derive(Debug, Queryable, Identifiable)]
#[diesel(table_name = crate::users::infrastructure::diesel::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::users::infrastructure::diesel::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub id: uuid::Uuid,
    pub name: &'a str,
    pub password: &'a str,
    pub email: &'a str,
}
