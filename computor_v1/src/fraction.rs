use std::fmt;

use crate::math_utility::math_utility::checked_gcd;


#[derive(Debug, PartialEq)]
pub struct Fraction {
    top: i64,
    bottom: i64,
}


impl Fraction {
    pub fn safe_new(top: i64, bottom: i64) -> Option<Fraction> {
        if bottom == 0 {
            None
        } else {
            if bottom < 0 {
                let neg_top = match top.checked_neg() {
                    Some(n) => n,
                    None => return None,
                };
                let neg_bottom = match bottom.checked_neg() {
                    Some(n) => n,
                    None => return None,
                };
                if neg_top == i64::MIN {
                    return None
                }
                Some(Fraction {top: neg_top, bottom: neg_bottom})
            } else {
                if top == i64::MIN {
                    return None
                }
                Some(Fraction {top, bottom})
            }
        }
    }


    pub fn make_irreducible_fraction(&self) -> Fraction {
        let gcd_value = match checked_gcd(self.top, self.bottom) {
            Some(v) => v,
            None => 1,
        };
        Fraction {top: self.top / gcd_value, bottom: self.bottom / gcd_value}
    }
}


impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.bottom == 1 {
            write!(f, "{}", self.top)
        } else {
            write!(f, "{} / {}", self.top, self.bottom)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_new_bottom_zero() {
        let top = 2;
        let bottom = 0;
        assert_eq!(Fraction::safe_new(top, bottom), None);
    }

    #[test]
    fn safe_new_normal() {
        let top = 2;
        let bottom = 1;
        assert_eq!(Fraction::safe_new(top, bottom),
            Some(Fraction {top: 2, bottom: 1}));
    }

    #[test]
    fn safe_new_top_plus_bottom_minus() {
        let top = 2;
        let bottom = -1;
        assert_eq!(Fraction::safe_new(top, bottom),
            Some(Fraction {top: -2, bottom: 1}));
    }

    #[test]
    fn safe_new_top_minus_bottom_minus() {
        let top = -2;
        let bottom = -1;
        assert_eq!(Fraction::safe_new(top, bottom),
            Some(Fraction {top: 2, bottom: 1}));
    }

    #[test]
    fn make_irreducible_fraction_2_4() {
        let fraction = Fraction {top: 2, bottom: 4};
        assert_eq!(fraction.make_irreducible_fraction(), 
            Fraction {top: 1, bottom: 2});
    }

    #[test]
    fn make_irreducible_fraction_1_2() {
        let fraction = Fraction {top: 1, bottom: 2};
        assert_eq!(fraction.make_irreducible_fraction(), 
            Fraction {top: 1, bottom: 2});
    }

    #[test]
    fn make_irreducible_fraction_minus_1_2() {
        let fraction = Fraction {top: -1, bottom: 2};
        assert_eq!(fraction.make_irreducible_fraction(), 
            Fraction {top: -1, bottom: 2});
    }

    #[test]
    fn make_irreducible_fraction_0_2() {
        let fraction = Fraction {top: 0, bottom: 2};
        assert_eq!(fraction.make_irreducible_fraction(), 
            Fraction {top: 0, bottom: 1});
    }

    #[test]
    fn display_1_2() {
        let fraction = Fraction {top: 1, bottom: 2};
        assert_eq!(format!("{}", fraction), "1 / 2".to_string());
    }

    #[test]
    fn display_minus_1_2() {
        let fraction = Fraction {top: -1, bottom: 2};
        assert_eq!(format!("{}", fraction), "-1 / 2".to_string());
    }

    #[test]
    fn display_minus_2_1() {
        let fraction = Fraction {top: 2, bottom: 1};
        assert_eq!(format!("{}", fraction), "2".to_string());
    }
}
