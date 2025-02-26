mod utils;

use clap::{arg, command, ArgAction, Command};
use std::path::PathBuf;
use utils::{csv_file_handler::CsvFileHandler, subject::Subjects};
#[macro_use] extern crate prettytable;
fn main() {
    let csv_file = CsvFileHandler::new(PathBuf::from("studio_data.csv"));
    let subjects = Subjects::from(csv_file);

    let matches = command!() // requires `cargo` feature
        .subcommand(
            Command::new("show").args([
                arg!(-l --subjects ... "List all subjects contained in studio_data.csv")
                    .action(ArgAction::SetTrue),
                arg!(-s --subject <SUBJECT> "List all the arguments ordered by weight" ),
            ]),
        )
        .get_matches();

    // retrieve a subject
    if let Some(matches) = matches.subcommand_matches("show") {
        if matches.get_flag("subjects") {
            subjects.list_all_subjects();
        }

        if let Some(subject_name) = matches.get_one::<String>("subject") {
            subjects.list_subject_args(subject_name);
        }
    }
    
    
}
