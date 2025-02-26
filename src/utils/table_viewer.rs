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
                main_table.add_row(row![bBdFG -> title]);
                
                arg_table.add_row(row![bBdFm => "n.","Argument","Rimembranza"]);
                for (n,arg) in subj.weighted_arguments.0.iter().enumerate(){
                    arg_table.add_row(row![(n+1).to_string()+".", arg.1, arg.0]);
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
        table.add_row(row![bFg=>"n.","Materie", "n. Arguments"]);
        for (n, subj) in self.0.iter().enumerate() {
            let row = ucfirst(&subj.name);
            let args_number = subj.weighted_arguments.0.iter().len();
            table.add_row(row![n + 1, row, args_number]);
        }
        table.printstd();
    }
}
