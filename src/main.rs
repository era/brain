#[macro_use] extern crate rocket;

mod database;
mod documents;

use std::env;
use rocket::serde::json::Json;

// This whole function needs to be refactored
fn db() -> database::DataBase {
    if let Ok(uri) = env::var("SQLITE") {

        let mut db = database::DataBase::connect(&uri);
   
        db.add_source(database::Source {table: "twitter_tweets".to_string(), text_column: "text".to_string(), id_column: "id".to_string()}); //TODO IMPROVE THIS

        db.add_source(database::Source {table: "twitter_likes".to_string(), text_column: "text".to_string(), id_column: "id".to_string()}); //TODO IMPROVE THIS
    
        return db;

    } else {
        panic!("No SQLITE env variable defined");
    }
}

// the result of this function should be a union of
// database data and documents data
#[get("/?<search>")]
fn index(search: &str) -> Json<Vec<database::Data>> {
    let results = db().search(search);
    Json(results)
}

#[launch]
fn rocket() -> _ {
    if let Ok(folder) = env::var("DIGITAL_GARDEN") {
        // need to check result of this
        documents::index(&folder);

    } 
    // setup log
    // index digital garden
    rocket::build().mount("/", routes![index])
}
