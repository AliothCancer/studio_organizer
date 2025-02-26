#![allow(dead_code)]

use std::{collections::HashMap, fs::File};

use csv::Reader;

use super::csv_file_handler::MyRow;

#[derive(Debug)]
pub struct Subjects(pub Vec<Subject>);
impl Subjects {
    /// merge arguments if subject name is already present, otherwise add subject and arguments
    pub(crate) fn merge_arguments(&mut self, subject_name: &String, arguments: Vec<String>) {
        if let Some(subject) = self.0.iter_mut().find(|x| &x.name == subject_name) {
            // here the subject is already present
            // first check differences

            let mut new_args = arguments
                .clone()
                .into_iter()
                .map(|x| (0_u16, x))
                .filter(|x| !subject.weighted_arguments.0.contains(x))
                .collect::<Vec<_>>();

            subject.weighted_arguments.0.append(&mut new_args);
            println!(
                "Subject: {} merged, current present args:{:?}",
                subject_name, arguments
            );
        } else {
            // here the subject is not present
            println!(
                "Subject: {} added with following args:{:?}",
                subject_name, arguments
            );
            self.0.push(Subject::new(subject_name, arguments));
        }
    }
    pub fn add_subject(&mut self, subject: Subject) {
        self.0.push(subject);
    }
}

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

        Self(args.into_iter().map(|s| (0, s)).collect())
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
impl From<String> for Subject {
    fn from(value: String) -> Self {
        Subject {
            name: value,
            weighted_arguments: WeightedArgs::new(vec![]),
        }
    }
}

impl From<&mut Reader<File>> for Subjects {
    fn from(value: &mut Reader<File>) -> Self {
        let iter = value
            .deserialize::<MyRow>()
            .map(|x| x.expect("error during deserialization"));

        let mut hs: HashMap<String, Vec<(u16, String)>> = HashMap::new();
        iter.for_each(|x| {
            hs.entry(x.subject_name)
                .and_modify(|y| {
                    y.push((x.rimembranza, x.argument.clone()));
                })
                .or_insert(vec![(x.rimembranza, x.argument.clone())]);
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
