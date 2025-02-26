#![allow(unused)]

use std::{collections::HashMap, fs::OpenOptions, io::Read, iter::Skip, path::PathBuf};

use csv;

use super::subject::{self, Subject, Subjects};

pub struct TypFileHandler {
    typ_file_path: PathBuf,
}

impl TypFileHandler {
    pub fn new(file_path: PathBuf) -> Self {
        if file_path.exists() {
            Self {
                typ_file_path: file_path,
            }
        } else {
            panic!("File does not exist")
        }
    }
    pub fn load_typ_file(&self) -> Subjects {
        if self.typ_file_path.exists() {
            let mut options = OpenOptions::new();
            let mut file = options
                .read(true)
                .open(self.typ_file_path.clone())
                .expect("error trying to open typ file");

            // the typ file content
            let mut buf_reader = String::new();
            file.read_to_string(&mut buf_reader);

            // parsing typ file
            let mut lines = buf_reader.lines().enumerate();

            // find the line to start to parse, corresponding to "---"
            if let Some((start_line_number, _delimiter)) = lines.find(|(_n, x)| x.contains("---")) {
                // parse the typ file to a Hashmap<String=subject_name,Vec<String=argument>>
                let parsed_typ = parse_typ(lines);
                let subjects = parsed_typ
                    .into_iter()
                    .map(|(subject_name, arguments)| Subject::new(subject_name, arguments))
                    .collect::<Vec<_>>();
                Subjects(subjects)
            } else {
                Subjects(vec![])
            }
        } else {
            panic!("File does not exist")
        }
    }
}

fn parse_typ<'a>(lines: impl Iterator<Item = (usize, &'a str)>) -> HashMap<String, Vec<String>> {
    // already divided in lines, I have to distinguish "="-line from "=="-line
    // order count, == refer to the previous =
    let mut lines = lines.filter(|(n, x)| !x.is_empty()).peekable();

    let mut subjects = HashMap::new();

    let mut current_subject = String::new();

    for (_n, line) in lines {
        let (header, title) = line.split_once(" ").unwrap();
        if header == "=" {
            current_subject = title.to_ascii_lowercase();
            subjects.insert(current_subject.clone(), vec![]);
        } else if header == "==" {
            let mut args = subjects
                .get_mut(&current_subject.to_ascii_lowercase())
                .unwrap();
            args.push(title.to_string());
        }
    }

    subjects
}
