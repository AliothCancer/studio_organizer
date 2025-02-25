mod utils;

use clap::{arg, command, value_parser, Arg, ArgAction, Command};
use std::{path::PathBuf, str::FromStr};
use utils::{file_handler::FileHandler, subject::Subject, *};
#[macro_use] extern crate prettytable;
fn main() {
    let file = FileHandler::new(PathBuf::from_str("data_file.typ").unwrap());
    let subjects = file.load_typ_file();

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
