use prettytable::Table;
use ucfirst::ucfirst;

use super::subject::Subjects;

impl Subjects {
    /// print the arguments for the given subject name
    pub(crate) fn list_subject_args(&self, subject_name: &str) {
        let _ = table!();
        let mut main_table = Table::new();
        let mut arg_table = Table::new();
        for subj in self.0.iter(){
            if subj.name == subject_name{
                let title = subject_name.to_uppercase();
                main_table.add_row(row![Bb -> title]);
                
                arg_table.add_row(row![bFm => "n.","Argument","Weight"]);
                for (n,(arg, weight)) in subj.weighted_arguments.iter().enumerate(){
                    arg_table.add_row(row![(n+1).to_string()+".", arg,weight]);
                }
            }

        }
        ;
        main_table.print_tty(true).unwrap();
        arg_table.print_tty(true).unwrap();
        
    }

    /// print all the subjects name written in data_file.typ
    pub(crate) fn list_all_subjects(&self) {
        let mut table = Table::new();
        for (n, subj) in self.0.iter().enumerate() {
            let row = ucfirst(&subj.name);

            table.add_row(row![n + 1, row]);
        }
        table.printstd();
    }
}
