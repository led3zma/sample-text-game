use csv::{ReaderBuilder, StringRecord};
use std::fs;

const FILE_NAME: &str = "story.csv";

fn main() {
    let contents = fs::read_to_string(FILE_NAME).unwrap();
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(contents.as_bytes());
    rdr.records()
        .for_each(|result| println!("{:#?}", result.unwrap().get(2).unwrap().trim()));
}
