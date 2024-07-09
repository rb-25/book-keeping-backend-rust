use rocket::response::content;
use rocket_contrib::json::Json;

use crate::models::User;

#[get("/users")]
pub fn index() -> Json<Vec<User>> {
    // TO DO: implement database query to retrieve users
    Json(vec![])
}