#[macro_use] extern crate rocket;

mod database;
mod documents;
mod watcher;

use std::env;
use rocket::serde::{Serialize, json::Json};
use std::thread;


#[derive(Serialize)]
struct Result {
    database: Vec<database::Data>,
    garden: Vec<documents::Data>,
}

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
fn index(search: &str) -> Json<Result> {
    let db_results = db().search(search);
    let garden_results = documents::search(search);
    let results = Result {database: db_results, garden: garden_results};
    Json(results)
}

#[launch]
fn rocket() -> _ {
    //env_logger::init();
    if let Ok(folder) = env::var("DIGITAL_GARDEN") {
        // need to check result of this
        documents::index(&folder);
        thread::spawn(move || {
            watcher::watch(&folder);
        });

    } 
    // setup log
    // index digital garden
    rocket::build().mount("/", routes![index])
}
