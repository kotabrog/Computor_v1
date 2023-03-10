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


#[derive(Debug, PartialEq, Clone)]
pub struct Term {
    pub coefficient: Coefficient,
    pub degree: i64,
}


impl Coefficient {
    pub fn add(&self, other: &Coefficient) -> Coefficient {
        match (self, other) {
            (Coefficient::NumInt(n1), Coefficient::NumInt(n2)) => {
                match n1.checked_add(*n2) {
                    Some(n) => Coefficient::NumInt(n),
                    None => Coefficient::NumFloat(*n1 as f64 + *n2 as f64),
                }
            },
            (Coefficient::NumInt(n1), Coefficient::NumFloat(n2)) => {
                Coefficient::NumFloat(*n1 as f64 + n2)
            },
            (Coefficient::NumFloat(n1), Coefficient::NumInt(n2)) => {
                Coefficient::NumFloat(n1 + *n2 as f64)
            },
            (Coefficient::NumFloat(n1), Coefficient::NumFloat(n2)) => {
                Coefficient::NumFloat(n1 + n2)
            }
        }
    }

    pub fn mul(&self, other: &Coefficient) -> Coefficient {
        match (self, other) {
            (Coefficient::NumInt(n1), Coefficient::NumInt(n2)) => {
                match n1.checked_mul(*n2) {
                    Some(n) => Coefficient::NumInt(n),
                    None => Coefficient::NumFloat(*n1 as f64 * *n2 as f64),
                }
            },
            (Coefficient::NumInt(n1), Coefficient::NumFloat(n2)) => {
                Coefficient::NumFloat(*n1 as f64 * n2)
            },
            (Coefficient::NumFloat(n1), Coefficient::NumInt(n2)) => {
                Coefficient::NumFloat(n1 * *n2 as f64)
            },
            (Coefficient::NumFloat(n1), Coefficient::NumFloat(n2)) => {
                Coefficient::NumFloat(n1 * n2)
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

    pub fn is_zero(&self) -> bool {
        match self {
            Coefficient::NumInt(n) => {
                *n == 0
            },
            Coefficient::NumFloat(n) => {
                *n == 0.0
            },
        }
    }

    pub fn to_float(&self) -> f64 {
        match self {
            Coefficient::NumInt(n) => {
                *n as f64
            },
            Coefficient::NumFloat(n) => {
                *n
            },
        }
    }

    pub fn is_plus(&self) -> bool {
        match self {
            Coefficient::NumInt(n) => {
                *n >= 0
            },
            Coefficient::NumFloat(n) => {
                *n >= 0.0
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coefficient_add_int_int() {
        let lhs = Coefficient::NumInt(1);
        let rhs = Coefficient::NumInt(2);
        assert_eq!(lhs.add(&rhs), Coefficient::NumInt(3));
    }
    
    #[test]
    fn coefficient_add_float_int() {
        let lhs = Coefficient::NumFloat(1.0);
        let rhs = Coefficient::NumInt(2);
        assert_eq!(lhs.add(&rhs), Coefficient::NumFloat(3.0));
    }

    #[test]
    fn coefficient_add_int_float() {
        let lhs = Coefficient::NumInt(2);
        let rhs = Coefficient::NumFloat(1.0);
        assert_eq!(lhs.add(&rhs), Coefficient::NumFloat(3.0));
    }

    #[test]
    fn coefficient_add_float_float() {
        let lhs = Coefficient::NumFloat(2.0);
        let rhs = Coefficient::NumFloat(1.0);
        assert_eq!(lhs.add(&rhs), Coefficient::NumFloat(3.0));
    }

    #[test]
    fn coefficient_add_error_int_int_overflow() {
        let lhs = Coefficient::NumInt(9223372036854775807);
        let rhs = Coefficient::NumInt(1);
        assert_eq!(lhs.add(&rhs), Coefficient::NumFloat(9223372036854775808_f64));
    }

    #[test]
    fn coefficient_mul_int_int() {
        let lhs = Coefficient::NumInt(3);
        let rhs = Coefficient::NumInt(2);
        assert_eq!(lhs.mul(&rhs), Coefficient::NumInt(6));
    }
    
    #[test]
    fn coefficient_mul_float_int() {
        let lhs = Coefficient::NumFloat(3.0);
        let rhs = Coefficient::NumInt(2);
        assert_eq!(lhs.mul(&rhs), Coefficient::NumFloat(6.0));
    }

    #[test]
    fn coefficient_mul_int_float() {
        let lhs = Coefficient::NumInt(2);
        let rhs = Coefficient::NumFloat(3.0);
        assert_eq!(lhs.mul(&rhs), Coefficient::NumFloat(6.0));
    }

    #[test]
    fn coefficient_mul_float_float() {
        let lhs = Coefficient::NumFloat(2.0);
        let rhs = Coefficient::NumFloat(3.0);
        assert_eq!(lhs.mul(&rhs), Coefficient::NumFloat(6.0));
    }

    #[test]
    fn coefficient_mul_error_int_int_overflow() {
        let lhs = Coefficient::NumInt(9223372036854775807);
        let rhs = Coefficient::NumInt(2);
        assert_eq!(lhs.mul(&rhs), Coefficient::NumFloat(9223372036854775807_f64 * 2.0));
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

    #[test]
    fn is_zero_int_zero() {
        let value = Coefficient::NumInt(0);
        assert_eq!(value.is_zero(), true);
    }

    #[test]
    fn is_zero_int_one() {
        let value = Coefficient::NumInt(1);
        assert_eq!(value.is_zero(), false);
    }

    #[test]
    fn is_zero_float_zerp() {
        let value = Coefficient::NumFloat(0.0);
        assert_eq!(value.is_zero(), true);
    }

    #[test]
    fn is_zero_float_one() {
        let value = Coefficient::NumFloat(1.0);
        assert_eq!(value.is_zero(), false);
    }

    #[test]
    fn to_float_int() {
        let value = Coefficient::NumInt(1);
        assert_eq!(value.to_float(), 1.0);
    }

    #[test]
    fn to_float_float() {
        let value = Coefficient::NumFloat(1.2);
        assert_eq!(value.to_float(), 1.2);
    }

    #[test]
    fn is_plus_int_plus() {
        let value = Coefficient::NumInt(1);
        assert_eq!(value.is_plus(), true);
    }

    #[test]
    fn is_plus_int_minus() {
        let value = Coefficient::NumInt(-1);
        assert_eq!(value.is_plus(), false);
    }

    #[test]
    fn is_plus_float_plus() {
        let value = Coefficient::NumFloat(1.0);
        assert_eq!(value.is_plus(), true);
    }

    #[test]
    fn is_plus_float_minus() {
        let value = Coefficient::NumFloat(-1.0);
        assert_eq!(value.is_plus(), false);
    }
}