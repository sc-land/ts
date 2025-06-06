use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "ts.pest"]
pub struct TSP;
