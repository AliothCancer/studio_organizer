#![allow(unused)]
use std::{collections::{HashMap, HashSet}, fs::File, path::PathBuf};

use csv::{self, Reader};
use serde::{Deserialize, Serialize};

use super::subject;

pub struct CsvFileHandler {
    file_path: PathBuf,
    pub reader: Reader<File>,
}
impl CsvFileHandler {
    pub fn new(file_path: PathBuf) -> Self {
        let mut reader = csv::Reader::from_path(file_path.clone()).unwrap();
        Self { file_path, reader }
    }
    pub fn get_arguments(&mut self, requested_subject_name: String) -> Vec<String> {
        self.reader
            .deserialize::<MyRow>()
            .map(|x| x.expect("error during deserialization"))
            .filter(|x| match x {
                MyRow {
                    subject_name,
                    argument,
                    rimembranza,
                } if subject_name == &requested_subject_name => true,
                _ => false,
            })
            .map(|x| x.argument)
            .collect::<Vec<String>>()
    }
    pub fn get_subjects(&mut self) -> HashSet<String> {
        self.reader
            .deserialize::<MyRow>()
            .map(|x| x.expect("error during deserialization"))
            .map(|x| x.subject_name)
            .collect::<HashSet<String>>()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MyRow {
    #[serde(rename = "materia")]
    pub subject_name: String,
    #[serde(rename = "argomento")]
    pub argument: String,
    #[serde(rename = "rimembranza")]
    pub rimembranza: u16,
}
