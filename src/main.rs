#[macro_use] extern crate rocket;

mod database;

use rocket::http::RawStr;
use rocket::serde::json::Json;
use rocket::form::Form;

// This whole function needs to be refactored
fn db() -> database::DataBase {
    let mut db = database::DataBase::connect("test"); //setup on a toml file
   
    db.add_source(database::Source {table: "twitter_tweets".to_string(), text_column: "text".to_string(), id_column: "id".to_string()}); //TODO IMPROVE THIS

    db.add_source(database::Source {table: "twitter_likes".to_string(), text_column: "text".to_string(), id_column: "id".to_string()}); //TODO IMPROVE THIS
    
    return db;
}

#[get("/?<search>")]
fn index(search: &str) -> Json<String> {
    Json("hello world".to_string())
}

#[launch]
fn rocket() -> _ {
   
    rocket::build().mount("/", routes![index])
}
