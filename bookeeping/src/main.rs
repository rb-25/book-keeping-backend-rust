#[macro_use] extern crate rocket;

mod models;
mod routes;
mod utils;

use rocket::Rocket;
use rocket_contrib::databases::diesel;

#[database("my_app")]
pub struct Db(diesel::PgConnection);

fn main() {
    rocket::build()
       .attach(Db::fairing())
       .mount("/", routes![routes::users::index])
       .launch();
}
