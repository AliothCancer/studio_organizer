mod utils;

use clap::{Parser, Subcommand, Args, ArgAction};
use std::path::PathBuf;
use utils::{csv_file_handler::CsvFileHandler, subject::Subjects};
#[macro_use]
extern crate prettytable;

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

    match &cli.command {
        Commands::Show(args) => show(args),
        Commands::Add(args) => add(args),
    }
}

fn show(args: &ShowArgs) {
    let csv_file = CsvFileHandler::new(PathBuf::from("studio_data.csv"));
    let subjects = Subjects::from(csv_file);

    if args.subjects {
        subjects.list_all_subjects();
    }
    if let Some(subject_name) = &args.subject {
        subjects.list_subject_args(subject_name);
    }
}

fn add(args: &AddArgs) {
    // Se sono stati forniti argomenti, li suddividiamo per virgola
    let arg_list = args.arguments.as_ref().map(|s| {
        s.split(',')
         .map(|s| s.trim().to_string())
         .collect::<Vec<_>>()
    });
    
    
    match arg_list {
        Some(in_args) => {
            println!("Aggiunta materia: {}", args.subject);
            println!("Con argomenti: {:?}", in_args)
        },
        None => println!("Senza argomenti"),
    }
}
