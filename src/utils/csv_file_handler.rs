#![allow(unused)]
use std::path::PathBuf;

use csv;
use serde::{Deserialize, Serialize};

pub struct CsvFileHandler {
    file_path: PathBuf,
}
impl CsvFileHandler {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }
    pub fn read(&self) {
        let mut reader = csv::Reader::from_path(self.file_path.clone()).unwrap();
        let rows = reader
            .deserialize::<MyRow>()
            .map(|x| x.expect("errore durante la deserializzazione"))
            .collect::<Vec<_>>();
        dbg!(rows);
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct MyRow {
    #[serde(rename = "materia")]
    subject_name: String,
    #[serde(rename = "argomento")]
    argument: String,
    #[serde(rename = "rimembranza")]
    rimembranza: u8,
}
