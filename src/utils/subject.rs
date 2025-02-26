use std::collections::HashMap;

use super::csv_file_handler::{CsvFileHandler, MyRow};

#[derive(Debug)]
pub struct Subjects(pub Vec<Subject>);

#[derive(Debug)]
pub struct Subject {
    pub name: String,
    pub weighted_arguments: WeightedArgs,
}
#[derive(Debug)]
pub struct WeightedArgs(pub Vec<(u16, String)>);

impl WeightedArgs {
    pub fn new(args: Vec<String>) -> Self {
        // at the beginning, all arguments have a standard value 100

        Self(
            args.into_iter()
                .map(|s| (100, s))
                .collect(),
        )
    }


}

impl Subject {
    pub fn new(name: impl Into<String>, args: Vec<String>) -> Self {
        Self {
            name: name.into(),
            weighted_arguments: WeightedArgs::new(args),
        }
    }
}
impl From<CsvFileHandler> for Subjects {
    fn from(mut value: CsvFileHandler) -> Self {
        let iter = value
            .reader
            .deserialize::<MyRow>()
            .map(|x| x.expect("error during deserialization"));

        let mut hs: HashMap<String, Vec<(u16,String)>> = HashMap::new();
        iter.for_each(|x| {
            hs.entry(x.subject_name)
                .and_modify(|y| {
                    y.push((x.rimembranza,x.argument.clone()));
                })
                .or_insert(vec![(x.rimembranza,x.argument.clone())]);
        });
        let subjects = hs
            .into_iter()
            .map(|(name, x)| {
                let weighted_arguments = WeightedArgs(x);
                Subject {
                    name,
                    weighted_arguments,
                }
            })
            .collect::<Vec<_>>();

        Self(subjects)
    }
}
