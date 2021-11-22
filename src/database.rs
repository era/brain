use sqlite;
use sqlite::State;

pub struct Data {
    text: String,
    table: String,
    id: String, // using string here, since the id of a table may not always be numeric
}

pub struct Source {
    table: String,
    text_column: String,
    id_column: String
}

pub struct DataBase {
    sources: Vec<Source>,
    conn: sqlite::Connection, //need to change this in the future
}

impl DataBase {
    pub fn connect(self: &mut Self, uri: &str) {
       let conn = sqlite::open(uri);

       if let Ok(conn) = conn {
            self.conn = conn;
       } else {
            panic!("Could not open a connection with SQLITE3 database");
       }

    }

    pub fn add_source(self: &mut Self, source: Source) {
        self.sources.push(source);
    }

    pub fn search(self: &Self, text: &str) -> Vec<Data> {
        let mut results: Vec<Data> = vec![];
        for source in &self.sources {
            let sql = source.search_sql();
            let mut statement = self.conn
                                    .prepare(sql)
                                    .unwrap();
            
            statement.bind(1, text);
            
            while let State::Row = statement.next().unwrap() {
                let id = statement.read::<String>(0).unwrap();
                let result_text = statement.read::<String>(1).unwrap();
                let result = Data {table: source.table.to_string(), id: id, text: result_text};
                results.push(result);
            }
        }
        vec![]
    }
}

impl Source {
    fn search_sql(self: &Self) -> String {
        format!("select {id}, {text} from {table} where {table} match ?",
                        id=self.id_column, text=self.text_column, table=self.table)
    }

}


#[cfg(test)]
mod tests {
    use crate::database;

    #[test]
    fn format_source_sql() {
        let source = database::Source {table: "MY_TABLE".to_string(), id_column: "id".to_string(), text_column:"text".to_string()};

        assert_eq!("select id, text from MY_TABLE where MY_TABLE match ?", source.search_sql())
    }

}


