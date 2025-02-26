#[derive(Debug)]
pub struct Subjects(pub Vec<Subject>);


#[derive(Debug)]
pub struct Subject {
    pub name: String,
    pub weighted_arguments: WeightedArgs,
}
#[derive(Debug)]
pub struct WeightedArgs {
    pub args: Vec<String>,
    pub weights: Vec<u16>,
}
impl WeightedArgs {
    pub fn new(args: Vec<String>) -> Self {
        // at the beginning, all arguments have a standard value 100
        let weights = (0..args.len()).map(|_| 0).collect::<Vec<u16>>();
        Self { args, weights }
    }
    
    pub(crate) fn iter(&self) -> std::iter::Zip<std::slice::Iter<'_, std::string::String>, std::slice::Iter<'_, u16>> {
        self.args.iter().zip(self.weights.iter())
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
