

pub struct Data {
    text: String,
    table: String,
    id: String, // using string here, since the id of a table may not always be numeric
}

struct Source {
    table: String,
    text_column: String,
    id_column: String
}

impl Source {
    fn search(self: &Self, text: &str) -> Vec<Data> {
        vec![]
    }

}

pub fn search(text: &str) -> Vec<Data> {

    vec![]
}
