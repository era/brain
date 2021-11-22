#[macro_use] extern crate rocket;

mod database;

use std::env;
use rocket::serde::json::Json;

// This whole function needs to be refactored
fn db() -> database::DataBase {
    if let Ok(uri) = env::var("SQLITE") {

        let mut db = database::DataBase::connect(&uri); //setup on a toml file
   
        db.add_source(database::Source {table: "twitter_tweets".to_string(), text_column: "text".to_string(), id_column: "id".to_string()}); //TODO IMPROVE THIS

        db.add_source(database::Source {table: "twitter_likes".to_string(), text_column: "text".to_string(), id_column: "id".to_string()}); //TODO IMPROVE THIS
    
        return db;


    } else {

        panic!("No SQLITE env variable defined");
    }

}

#[get("/?<search>")]
fn index(search: &str) -> Json<Vec<database::Data>> {

    let results = db().search(search);
    Json(results)
}

#[launch]
fn rocket() -> _ {
   
    rocket::build().mount("/", routes![index])
}
