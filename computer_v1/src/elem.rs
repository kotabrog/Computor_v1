#[derive(Debug)]
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
