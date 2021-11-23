use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{doc, Index, ReloadPolicy, IndexWriter, IndexReader};
use tantivy::schema::Field;
use tempfile::TempDir;
use regex::Regex;

use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

use once_cell::sync::OnceCell;

static WRITER: OnceCell<IndexWriter> = OnceCell::new();
static READER: OnceCell<IndexReader> = OnceCell::new();


fn schema() -> Schema {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("body", TEXT);

    let schema = schema_builder.build();
    return schema;
}

pub fn index(folder: &str) -> tantivy::Result<()> {
    // for now using a temp folder,
    // we probably should change this soon
    let index_path = TempDir::new()?;

    let schema = schema(); 

    let index = Index::create_in_dir(&index_path, schema.clone())?;

    let mut index_writer = index.writer(50_000_000)?;

    let body = schema.get_field("body").unwrap();
    // need to check for result
    let index_writer = match add_folder(folder, index_writer, body) {
        Ok(index_writer) => index_writer,
        _ => panic!("Could not index folder"), //we should not panic here, probably return an error
    };

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;
    // should check result 
    READER.set(reader);
    // should check result
    WRITER.set(index_writer);

    Ok(())
}

fn add_folder(folder: &str, mut writer: IndexWriter, body: Field) -> Result<IndexWriter, io::Error> {
    
    let markdown = Regex::new(r".{1,}\.md$").unwrap(); // ok to unwrap since the regex is tested and works
    for entry in fs::read_dir(folder)? {
        let dir = entry?;
        let file = dir.path();
        let file = file.to_string_lossy();
        if markdown.is_match(file.as_ref()) {
            let file_content = fs::read(file.as_ref())?;
            let file_content = String::from_utf8_lossy(&file_content);
            writer.add_document(doc!(body => file_content.as_ref()));
        }
    }
    
    writer.commit();
    // given back the writer we borrowed
    Ok(writer)   
}
// neeed to refresh index when a file is modified or a file is added
// need a search function => https://github.com/quickwit-inc/tantivy/blob/main/examples/basic_search.rs
