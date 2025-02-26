mod utils;

#[macro_use]
extern crate prettytable;

use clap::{ArgAction, Args, Parser, Subcommand};
use std::path::PathBuf;
use utils::{
    csv_file_handler::CsvFileHandler,
    subject::{Subject, Subjects},
};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Mostra le materie e/o i relativi argomenti da studio_data.csv
    Show(ShowArgs),
    /// Aggiunge una materia, con o senza argomenti
    Add(AddArgs),
}

#[derive(Args)]
struct ShowArgs {
    /// Elenca tutte le materie contenute in studio_data.csv
    #[arg(short = 'l', long = "subjects", action = ArgAction::SetTrue)]
    subjects: bool,
    /// Elenca gli argomenti di una materia specifica
    #[arg(short = 's', long = "subject", value_name = "SUBJECT_NAME")]
    subject: Option<String>,
}

#[derive(Args)]
struct AddArgs {
    /// Nome della materia da aggiungere
    #[arg(short = 'm', long = "subject", value_name = "SUBJECT_NAME")]
    subject: String,
    /// Lista di argomenti della materia, separati da virgola (opzionale)
    #[arg(short = 'a', long = "arguments", value_name = "ARGUMENTS")]
    arguments: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let file_path = "studio_data.csv";
    let mut csv_file_handler = CsvFileHandler::new(PathBuf::from(file_path));
    let mut subjects = Subjects::from(&mut csv_file_handler.reader);

    match &cli.command {
        Commands::Show(args) => subjects.show(args),
        Commands::Add(args) => {
            let should_write = subjects.add(args);
            if should_write {
                csv_file_handler.write(subjects);
            }
        }
    }
}

impl Subjects {
    fn show(&self, args: &ShowArgs) {
        if args.subjects {
            self.list_all_subjects();
        }
        if let Some(subject_name) = &args.subject {
            self.list_subject_args(subject_name);
        }
    }

    fn add(&mut self, args: &AddArgs) -> bool {
        // Se sono stati forniti argomenti, li suddividiamo per virgola
        let arg_list = args.arguments.as_ref().map(|s| {
            s.split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<_>>()
        });

        match arg_list {
            Some(in_args) => {
                self.merge_arguments(&args.subject, in_args);
            }
            None => self.add_subject(Subject::from(args.subject.clone())),
        }
        true
    }
}
