use diesel::prelude::*;
use diesel::pg::Pg;

table! {
    users (id) {
        id -> Integer,
        name -> VarChar,
        email -> VarChar,
    }
}

#[derive(Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}