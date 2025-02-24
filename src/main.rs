mod utils;

use clap::{arg, command, value_parser, Arg, ArgAction, Command};
use std::path::PathBuf;
use utils::{csv_handler::CsvHandler, subject::Subject, *};

fn main() {
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
            my_prog_cmd::list_subjects();
        }

        if let Some(subject_name) = matches.get_one::<String>("subject") {
            my_prog_cmd::list_subject_args(subject_name);
        }
    }
}
