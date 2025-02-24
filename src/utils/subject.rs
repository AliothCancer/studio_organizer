


use std::vec;


#[derive(Debug)]
pub struct Subject{
    pub name: String,
    pub weighted_arguments: WeightedArgs,
}
#[derive(Debug)]
pub struct WeightedArgs{
    pub args: Vec<String>,
    pub weights: Vec<u32>
}

impl Subject{
    pub fn new(name: impl Into<String>) -> Self{
        Self{
            name: name.into(),
            weighted_arguments: WeightedArgs { args: vec![], weights: vec![] },
        }
    } 
}