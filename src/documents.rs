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
use rocket::serde::{Serialize, json::Json};

use once_cell::sync::OnceCell;

static WRITER: OnceCell<IndexWriter> = OnceCell::new();
static READER: OnceCell<IndexReader> = OnceCell::new();
static INDEX: OnceCell<Index> = OnceCell::new();

#[derive(Serialize)]
pub struct Data {
    pub text: String,
    pub path: String,
}

pub fn search(text: &str) -> Vec<Data> {
    let mut results = vec![];

    let searcher = match READER.get() {
        Some(reader) => reader.searcher(),
        _ => return vec![]
    };

    let index = match INDEX.get() {
        Some(index) => index,
        _ => return vec![],
    };

    let schema = schema();
    let body = schema.get_field("body").unwrap();
    let path = schema.get_field("path").unwrap();

    let query_parser = QueryParser::for_index(&index, vec![body]);

    let query = query_parser.parse_query(text).unwrap();
    
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10)).unwrap();

    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address).unwrap();

        let (doc_body, doc_path) = match (retrieved_doc.get_first(body).unwrap(), retrieved_doc.get_first(path).unwrap()) {
            (Value::Str(doc_body), Value::Str(doc_path)) => (doc_body, doc_path),
            (_, _) => continue,

        };
        
        results.push(Data {text: doc_body.to_string(), path: doc_path.to_string()});

    }

    return results;
}

fn schema() -> Schema {
    let mut schema_builder = Schema::builder();
    // we probably should only store the path and not the body
    // and in the result show the url for the website
    schema_builder.add_text_field("body", TEXT | STORED);

    schema_builder.add_text_field("path", TEXT | STORED);
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


    let path = schema.get_field("path").unwrap();

    // need to check for result
    let index_writer = match add_folder(folder, index_writer, body, path) {
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
    // should check result
    INDEX.set(index);
    Ok(())
}

fn add_folder(folder: &str, mut writer: IndexWriter, body: Field, file_path: Field) -> Result<IndexWriter, io::Error> {
    
    let markdown = Regex::new(r".{1,}\.md$").unwrap(); // ok to unwrap since the regex is tested and works
    for entry in fs::read_dir(folder)? {
        let dir = entry?;
        let file = dir.path();
        let file = file.to_string_lossy();
        if markdown.is_match(file.as_ref()) {
            let path = file.as_ref().replace(folder, "").replace(".md", "");
            
            let file_content = fs::read(file.as_ref())?;
            let file_content = String::from_utf8_lossy(&file_content);
            writer.add_document(doc!(
                    body => String::from(file_content),
                    file_path => path,
                    ));
           
        }
    }
    // need to check if it worked
    writer.commit(); 
    // given back the writer we borrowed
    Ok(writer)   
}
//TODO neeed to refresh index when a file is modified or a file is added
