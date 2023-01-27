#[derive(Debug, PartialEq)]
pub enum Elem {
    X,
    Plus,
    Minus,
    Prod,
    Power,
    Equal,
    NumInt(i64),
    NumFloat(f64),
}


#[derive(Debug)]
pub enum Coefficient {
    NumInt(i64),
    NumFloat(f64),
}


#[derive(Debug)]
pub struct Term {
    coefficient: Coefficient,
    degree: i64,
}
