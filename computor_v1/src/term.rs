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


#[derive(Debug, PartialEq, Clone)]
pub enum Coefficient {
    NumInt(i64),
    NumFloat(f64),
}


#[derive(Debug, PartialEq)]
pub struct Term {
    pub coefficient: Coefficient,
    pub degree: i64,
}


impl Coefficient {
    pub fn checked_add(&self, other: &Coefficient) -> Result<Coefficient, String> {
        match (self, other) {
            (Coefficient::NumInt(n1), Coefficient::NumInt(n2)) => {
                match n1.checked_add(*n2) {
                    Some(n) => Ok(Coefficient::NumInt(n)),
                    None => Err(format!("Overflow value {} + {}", n1, n2)),
                }
            },
            (Coefficient::NumInt(n1), Coefficient::NumFloat(n2)) => {
                Ok(Coefficient::NumFloat(*n1 as f64 + n2))
            },
            (Coefficient::NumFloat(n1), Coefficient::NumInt(n2)) => {
                Ok(Coefficient::NumFloat(n1 + *n2 as f64))
            },
            (Coefficient::NumFloat(n1), Coefficient::NumFloat(n2)) => {
                Ok(Coefficient::NumFloat(n1 + n2))
            }
        }
    }

    pub fn mul_minus(&self) -> Coefficient {
        match self {
            Coefficient::NumInt(n) => {
                match n.checked_mul(-1) {
                    Some(value) => Coefficient::NumInt(value),
                    None => Coefficient::NumFloat(*n as f64 * -1_f64),
                }
            },
            Coefficient::NumFloat(n) => {
                Coefficient::NumFloat(n * -1_f64)
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coefficient_checked_add_int_int() {
        let lhs = Coefficient::NumInt(1);
        let rhs = Coefficient::NumInt(2);
        assert_eq!(lhs.checked_add(&rhs), Ok(Coefficient::NumInt(3)));
    }
    
    #[test]
    fn coefficient_checked_add_float_int() {
        let lhs = Coefficient::NumFloat(1.0);
        let rhs = Coefficient::NumInt(2);
        assert_eq!(lhs.checked_add(&rhs), Ok(Coefficient::NumFloat(3.0)));
    }

    #[test]
    fn coefficient_checked_add_int_float() {
        let lhs = Coefficient::NumInt(2);
        let rhs = Coefficient::NumFloat(1.0);
        assert_eq!(lhs.checked_add(&rhs), Ok(Coefficient::NumFloat(3.0)));
    }

    #[test]
    fn coefficient_checked_add_float_float() {
        let lhs = Coefficient::NumFloat(2.0);
        let rhs = Coefficient::NumFloat(1.0);
        assert_eq!(lhs.checked_add(&rhs), Ok(Coefficient::NumFloat(3.0)));
    }

    #[test]
    fn coefficient_checked_add_error_int_int_overflow() {
        let lhs = Coefficient::NumInt(9223372036854775807);
        let rhs = Coefficient::NumInt(1);
        assert_eq!(lhs.checked_add(&rhs), Err(format!("Overflow value {} + {}", 9223372036854775807_i64, 1)));
    }

    #[test]
    fn coefficient_mul_minus_int_to_int() {
        let value = Coefficient::NumInt(1);
        assert_eq!(value.mul_minus(), Coefficient::NumInt(-1));
    }
    
    #[test]
    fn coefficient_mul_minus_int_to_float() {
        let value = Coefficient::NumInt(-9223372036854775808);
        assert_eq!(value.mul_minus(), Coefficient::NumFloat(9223372036854775808_f64));
    }

    #[test]
    fn coefficient_mul_minus_float_to_float() {
        let value = Coefficient::NumFloat(1_f64);
        assert_eq!(value.mul_minus(), Coefficient::NumFloat(-1_f64));
    }
}