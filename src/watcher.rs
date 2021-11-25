extern crate notify;


use crate::documents;

use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;
// When a file is updated
// we need to delete it and reinsert (there's no update in the api)
// When a file is updated
// we need to delete it and reinsert (there's no update in the api)
// when a new file is created we need to insert it


// When a file is updated
// we need to delete it and reinsert (there's no update in the api)
// when a new file is created we need to insert it

pub fn watch(folder: &str) {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

    watcher.watch(folder, RecursiveMode::Recursive).unwrap();
    

    loop {
        match rx.recv() {
           Ok(event) => handle(event, folder), // Create("PATH")
           Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn handle(event: DebouncedEvent, folder: &str) {
    if let DebouncedEvent::Create(file) = event {
        // should check result
        documents::add_file(file.to_string_lossy().as_ref(), folder);
    }
}
