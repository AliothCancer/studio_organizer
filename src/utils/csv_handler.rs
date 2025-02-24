#![allow(unused)]

use std::{fs::OpenOptions, io::Read, path::PathBuf};

use csv;

pub struct CsvHandler {
    file_path: PathBuf,
}

impl CsvHandler {
    pub fn new(file_path: PathBuf) -> Self {
        if file_path.exists() {
            Self { file_path }
        } else {
            panic!("File does not exist")
        }
    }
    pub fn load_typ_file(file_path: PathBuf) {
        if file_path.exists() {
            let mut options = OpenOptions::new();
            let mut file = options
                .read(true)
                .open(file_path)
                .expect("error trying to open typ file");

            // the typ file content
            let mut buf_reader = String::new();
            file.read_to_string(&mut buf_reader);

            // parsing typ file
            let mut lines = buf_reader.lines().enumerate();
            
            // find the line to start to parse, corresponding to "---"
            if let Some((start_line_number, _delimiter)) = lines.find(|(_n,x)| x.contains("---")){
                lines.skip(start_line_number);
                
            }

        } else {
            panic!("File does not exist")
        }
    }
}
