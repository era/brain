#[macro_use] extern crate rocket;

mod database;

use rocket::serde::json::Json;


#[get("/")]
fn index() -> Json<String> {
    Json("hello world".to_string())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
