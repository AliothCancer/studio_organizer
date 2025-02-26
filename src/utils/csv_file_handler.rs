#![allow(unused)]
use std::{collections::{HashMap, HashSet}, fs::{File, OpenOptions}, path::PathBuf};

use csv::{self, Reader, Writer};
use serde::{Deserialize, Serialize};

use super::subject::{self, Subject, Subjects};

pub struct CsvFileHandler {
    pub reader: Reader<File>,
    pub writer: Writer<File>
}
impl CsvFileHandler {
    pub fn new(file_path: PathBuf) -> Self {
        let mut reader = csv::Reader::from_path(file_path.clone()).unwrap();
        let options = OpenOptions::new().write(true).open(file_path).unwrap();
        let mut writer = csv::Writer::from_writer(options);
        Self { reader, writer }
    }
    pub fn get_arguments(&mut self, requested_subject_name: String) -> Vec<String> {
        self.reader
            .deserialize::<MyRow>()
            .map(|x| x.expect("error during deserialization"))
            .filter(|x| matches!(x, MyRow {
                    subject_name,
                    argument,
                    rimembranza,
                } if subject_name == &requested_subject_name))
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
    pub fn write(mut self, subjects: Subjects){
        for subj in subjects.0{
            //dbg!(&subj);
            let rows = Rows::from(subj);
            rows.0.iter().for_each(|x|{
                self.writer.serialize(x).expect("Error during serialization");
            });
        }
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

struct Rows(Vec<MyRow>);

impl From<Subject> for Rows {
    fn from(value: Subject) -> Self {
        Rows(
            value.weighted_arguments.0.into_iter().map(|(rimembranza, argument)|{
                MyRow { subject_name: value.name.clone(), argument, rimembranza }
            }).collect::<Vec<_>>()
        )
    }
}