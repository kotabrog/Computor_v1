use crate::term::{Term, Coefficient};
use crate::fraction::Fraction;
use crate::utility::is_int_value;
use crate::math_utility::math_utility::sqrt;


fn degree_0_solution_all() -> String {
    "The solution is an arbitrary real number.".to_string()
}


fn degree_0_solution_non() -> String {
    "There is no solution.".to_string()
}


fn degree_0_solution(terms: &Vec<Term>) -> String {
    if terms.is_empty() {
        degree_0_solution_all()
    } else {
        if terms[0].coefficient.is_zero() {
            degree_0_solution_all()
        } else {
            degree_0_solution_non()
        }
    }
}


fn check_all_int_terms(terms: &Vec<Term>) -> bool {
    for term in terms {
        match term.coefficient {
            Coefficient::NumInt(_) => {},
            Coefficient::NumFloat(_) => return false,
        }
    }
    true
}


fn make_terms_no_gaps(terms: &Vec<Term>, degree: i64) -> Vec<Term> {
    let mut vec = Vec::new();
    let mut index = 0;
    for i in 0..=degree {
        if terms[index].degree == i {
            vec.push(terms[index].clone());
            index += 1;
        } else {
            vec.push(Term { coefficient: Coefficient::NumInt(0), degree: i });
        }
    }
    vec
}


fn degree_1_solution_2_terms_fraction(terms: &Vec<Term>) -> Option<String> {
    let b = match terms[0].coefficient {
        Coefficient::NumInt(n) => {
            match n.checked_neg() {
                Some(x) => x,
                None => return None,
            }
        },
        _ => 0,
    };
    let a = match terms[1].coefficient {
        Coefficient::NumInt(n) => n,
        _ => 0,
    };
    let mut fraction = match Fraction::safe_new(b, a) {
        Some(x) => x,
        None => return None,
    };
    fraction = fraction.make_irreducible_fraction();
    if b == 0 {
        Some("0".to_string())
    } else {
        Some(format!("{}", fraction))
    }
}


fn degree_1_solution_2_terms_decimals(terms: &Vec<Term>) -> String {
    let a = terms[1].coefficient.to_float();
    let b = - terms[0].coefficient.to_float();
    format!("{}", b / a)
}


fn degree_1_solution(terms: &Vec<Term>) -> String {
    // ax + b = 0
    let string = "The solution is:\n".to_string();
    let terms = make_terms_no_gaps(terms, 1);
    let is_all_int = check_all_int_terms(&terms);
    if is_all_int {
        match degree_1_solution_2_terms_fraction(&terms) {
            Some(s) => return string + s.as_str(),
            None => {},
        }
    }
    string + degree_1_solution_2_terms_decimals(&terms).as_str()
}


fn degree_2_discriminant(terms: &Vec<Term>) -> Coefficient {
    let c = &terms[0].coefficient;
    let b = &terms[1].coefficient;
    let a = &terms[2].coefficient;
    let temp = Coefficient::NumInt(-4);
    b.mul(b).add(&temp.mul(a).mul(c))
}


fn degree_2_solution_one_from_two_coefficient(a: &Coefficient, b: &Coefficient) -> String {
    if let (Coefficient::NumInt(n1), Coefficient::NumInt(n2)) = (a, b) {
        match Fraction::safe_new(*n2, *n1) {
            Some(fraction) => {
                let fraction = fraction.make_irreducible_fraction();
                if *n1 == 0 {
                    return "0".to_string()
                } else {
                    return format!("{}", fraction)
                }
            },
            None => {},
        };
    }
    let a = a.to_float();
    let b = b.to_float();
    format!("{}", b / a)
}


fn degree_2_solution_one(terms: &Vec<Term>) -> String {
    let string = "Discriminant is zero, the solution is:\n".to_string();
    let b = &terms[1].coefficient;
    let a = &terms[2].coefficient;
    let temp = Coefficient::NumInt(-2);
    let a = a.mul(&temp);
    string + degree_2_solution_one_from_two_coefficient(&a, b).as_str()
}


fn degree_2_solution_two(terms: &Vec<Term>, discriminant: &Coefficient) -> String {
    let mut string = "Discriminant is strictly positive, the two solutions are:\n".to_string();
    let b = &terms[1].coefficient;
    let a = &terms[2].coefficient;
    let temp = Coefficient::NumInt(2);
    let a = a.mul(&temp);
    let discriminant_root = sqrt(discriminant.to_float());
    let discriminant = if is_int_value(discriminant_root) {
        Coefficient::NumInt(discriminant_root as i64)
    } else {
        Coefficient::NumFloat(discriminant_root)
    };
    let b_plus = b.mul_minus().add(&discriminant);
    let b_minus = b.mul_minus().add(&discriminant.mul_minus());
    string += degree_2_solution_one_from_two_coefficient(&a, &b_plus).as_str();
    string += "\n";
    string + degree_2_solution_one_from_two_coefficient(&a, &b_minus).as_str()
}


fn degree_2_solution_complex(terms: &Vec<Term>, discriminant: &Coefficient) -> String {
    let string = "Discriminant is strictly negative, the two complex solutions are:\n".to_string();
    let b = &terms[1].coefficient.to_float();
    let a = &terms[2].coefficient.to_float() * 2.0;
    let discriminant_root = sqrt(-discriminant.to_float());
    let real_num = -b / a;
    let complex_num = discriminant_root / a;
    string + format!("{} ± {}i", real_num, complex_num).as_str()
}


fn degree_2_solution(terms: &Vec<Term>) -> String {
    // ax^2 + bx + c = 0
    let terms = make_terms_no_gaps(terms, 2);
    let discriminant = degree_2_discriminant(&terms);
    if discriminant.is_zero() {
        degree_2_solution_one(&terms)
    } else if discriminant.is_plus() {
        degree_2_solution_two(&terms, &discriminant)
    } else {
        degree_2_solution_complex(&terms, &discriminant)
    }
}


pub fn solution(terms: &Vec<Term>, degree: i64) -> String {
    match degree {
        0 => degree_0_solution(&terms),
        1 => degree_1_solution(&terms),
        2 => {degree_2_solution(&terms)},
        _ => {"".to_string()}
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn degree_0_solution_empty() {
        let terms = Vec::new();
        assert_eq!(degree_0_solution(&terms), "The solution is an arbitrary real number.".to_string());
    }

    #[test]
    fn degree_0_solution_zero() {
        let terms = vec![
            Term {coefficient: Coefficient::NumInt(0), degree: 0},
        ];
        assert_eq!(degree_0_solution(&terms), "The solution is an arbitrary real number.".to_string());
    }

    #[test]
    fn degree_0_solution_one() {
        let terms = vec![
            Term {coefficient: Coefficient::NumInt(1), degree: 0},
        ];
        assert_eq!(degree_0_solution(&terms), "There is no solution.".to_string());
    }

    #[test]
    fn degree_1_solution_only_a() {
        let terms = vec![
            Term {coefficient: Coefficient::NumInt(0), degree: 0},
            Term {coefficient: Coefficient::NumInt(1), degree: 1},
        ];
        assert_eq!(degree_1_solution(&terms), "The solution is:\n0".to_string());
    }

    #[test]
    fn degree_1_solution_plus_a_b_int() {
        let terms = vec![
            Term {coefficient: Coefficient::NumInt(1), degree: 0},
            Term {coefficient: Coefficient::NumInt(1), degree: 1},
        ];
        assert_eq!(degree_1_solution(&terms), "The solution is:\n-1".to_string());
    }

    #[test]
    fn degree_1_solution_plus_a_b_fraction() {
        let terms = vec![
            Term {coefficient: Coefficient::NumInt(1), degree: 0},
            Term {coefficient: Coefficient::NumInt(2), degree: 1},
        ];
        assert_eq!(degree_1_solution(&terms), "The solution is:\n-1 / 2".to_string());
    }

    #[test]
    fn degree_1_solution_a_b_fraction_neg_borrom() {
        let terms = vec![
            Term {coefficient: Coefficient::NumInt(1), degree: 0},
            Term {coefficient: Coefficient::NumInt(-2), degree: 1},
        ];
        assert_eq!(degree_1_solution(&terms), "The solution is:\n1 / 2".to_string());
    }

    #[test]
    fn degree_1_solution_a_b_fraction_to_int() {
        let terms = vec![
            Term {coefficient: Coefficient::NumInt(2), degree: 0},
            Term {coefficient: Coefficient::NumInt(1), degree: 1},
        ];
        assert_eq!(degree_1_solution(&terms), "The solution is:\n-2".to_string());
    }

    #[test]
    fn degree_1_solution_b_float() {
        let terms = vec![
            Term {coefficient: Coefficient::NumFloat(2.0), degree: 0},
            Term {coefficient: Coefficient::NumInt(1), degree: 1},
        ];
        assert_eq!(degree_1_solution(&terms), "The solution is:\n-2".to_string());
    }

    #[test]
    fn degree_1_solution_a_float() {
        let terms = vec![
            Term {coefficient: Coefficient::NumInt(1), degree: 0},
            Term {coefficient: Coefficient::NumFloat(2.0), degree: 1},
        ];
        assert_eq!(degree_1_solution(&terms), "The solution is:\n-0.5".to_string());
    }

    #[test]
    fn degree_1_solution_a_b_float() {
        let terms = vec![
            Term {coefficient: Coefficient::NumFloat(1.0), degree: 0},
            Term {coefficient: Coefficient::NumFloat(2.0), degree: 1},
        ];
        assert_eq!(degree_1_solution(&terms), "The solution is:\n-0.5".to_string());
    }

    #[test]
    fn degree_2_discriminant_only_a() {
        let terms = vec![
            Term {coefficient: Coefficient::NumInt(0), degree: 0},
            Term {coefficient: Coefficient::NumInt(0), degree: 1},
            Term {coefficient: Coefficient::NumInt(1), degree: 2},
        ];
        assert_eq!(degree_2_discriminant(&terms), Coefficient::NumInt(0));
    }

    #[test]
    fn degree_2_discriminant_only_ac() {
        let terms = vec![
            Term {coefficient: Coefficient::NumInt(2), degree: 0},
            Term {coefficient: Coefficient::NumInt(0), degree: 1},
            Term {coefficient: Coefficient::NumInt(1), degree: 2},
        ];
        assert_eq!(degree_2_discriminant(&terms), Coefficient::NumInt(-8));
    }

    #[test]
    fn degree_2_discriminant_float_ac() {
        let terms = vec![
            Term {coefficient: Coefficient::NumInt(2), degree: 0},
            Term {coefficient: Coefficient::NumInt(0), degree: 1},
            Term {coefficient: Coefficient::NumFloat(1.0), degree: 2},
        ];
        assert_eq!(degree_2_discriminant(&terms), Coefficient::NumFloat(-8.0));
    }

    #[test]
    fn degree_2_discriminant_all_int() {
        let terms = vec![
            Term {coefficient: Coefficient::NumInt(2), degree: 0},
            Term {coefficient: Coefficient::NumInt(3), degree: 1},
            Term {coefficient: Coefficient::NumInt(1), degree: 2},
        ];
        assert_eq!(degree_2_discriminant(&terms), Coefficient::NumInt(1));
    }

    #[test]
    fn degree_2_discriminant_all_float() {
        let terms = vec![
            Term {coefficient: Coefficient::NumInt(2), degree: 0},
            Term {coefficient: Coefficient::NumFloat(3.0), degree: 1},
            Term {coefficient: Coefficient::NumInt(1), degree: 2},
        ];
        assert_eq!(degree_2_discriminant(&terms), Coefficient::NumFloat(1.0));
    }

    #[test]
    fn degree_2_solution_only_a() {
        let terms = vec![
            Term {coefficient: Coefficient::NumInt(0), degree: 0},
            Term {coefficient: Coefficient::NumInt(0), degree: 1},
            Term {coefficient: Coefficient::NumInt(-1), degree: 2},
        ];
        assert_eq!(degree_2_solution(&terms),
            "Discriminant is zero, the solution is:\n0".to_string());
    }

    #[test]
    fn degree_2_solution_only_ac() {
        let terms = vec![
            Term {coefficient: Coefficient::NumInt(1), degree: 0},
            Term {coefficient: Coefficient::NumInt(0), degree: 1},
            Term {coefficient: Coefficient::NumInt(-1), degree: 2},
        ];
        assert_eq!(degree_2_solution(&terms),
            "Discriminant is strictly positive, the two solutions are:\n-1\n1".to_string());
    }

    #[test]
    fn degree_2_solution_complex() {
        let terms = vec![
            Term {coefficient: Coefficient::NumInt(1), degree: 0},
            Term {coefficient: Coefficient::NumInt(0), degree: 1},
            Term {coefficient: Coefficient::NumInt(1), degree: 2},
        ];
        assert_eq!(degree_2_solution(&terms),
            "Discriminant is strictly negative, the two complex solutions are:\n-0 ± 1i".to_string());
    }

    #[test]
    fn degree_2_solution_complex_b() {
        let terms = vec![
            Term {coefficient: Coefficient::NumInt(1), degree: 0},
            Term {coefficient: Coefficient::NumInt(4), degree: 1},
            Term {coefficient: Coefficient::NumInt(5), degree: 2},
        ];
        assert_eq!(degree_2_solution(&terms),
            "Discriminant is strictly negative, the two complex solutions are:\n-0.4 ± 0.2i".to_string());
    }

    #[test]
    fn degree_2_solution_one() {
        let terms = vec![
            Term {coefficient: Coefficient::NumInt(1), degree: 0},
            Term {coefficient: Coefficient::NumInt(-2), degree: 1},
            Term {coefficient: Coefficient::NumInt(1), degree: 2},
        ];
        assert_eq!(degree_2_solution(&terms),
            "Discriminant is zero, the solution is:\n1".to_string());
    }

    #[test]
    fn degree_2_solution_one_float_to_int() {
        let terms = vec![
            Term {coefficient: Coefficient::NumFloat(1.2), degree: 0},
            Term {coefficient: Coefficient::NumFloat(-2.4), degree: 1},
            Term {coefficient: Coefficient::NumFloat(1.2), degree: 2},
        ];
        assert_eq!(degree_2_solution(&terms),
            "Discriminant is zero, the solution is:\n1".to_string());
    }

    #[test]
    fn degree_2_solution_one_float_to_float() {
        let terms = vec![
            Term {coefficient: Coefficient::NumFloat(9.0 / 4.0), degree: 0},
            Term {coefficient: Coefficient::NumFloat(-3.0), degree: 1},
            Term {coefficient: Coefficient::NumInt(1), degree: 2},
        ];
        assert_eq!(degree_2_solution(&terms),
            "Discriminant is zero, the solution is:\n1.5".to_string());
    }

    #[test]
    fn degree_2_solution_two() {
        let terms = vec![
            Term {coefficient: Coefficient::NumInt(2), degree: 0},
            Term {coefficient: Coefficient::NumInt(-3), degree: 1},
            Term {coefficient: Coefficient::NumInt(1), degree: 2},
        ];
        assert_eq!(degree_2_solution(&terms),
            "Discriminant is strictly positive, the two solutions are:\n2\n1".to_string());
    }

    #[test]
    fn degree_2_solution_two_fraction() {
        let terms = vec![
            Term {coefficient: Coefficient::NumInt(-3), degree: 0},
            Term {coefficient: Coefficient::NumInt(1), degree: 1},
            Term {coefficient: Coefficient::NumInt(2), degree: 2},
        ];
        assert_eq!(degree_2_solution(&terms),
            "Discriminant is strictly positive, the two solutions are:\n1\n-3 / 2".to_string());
    }

    #[test]
    fn degree_2_solution_two_float() {
        let terms = vec![
            Term {coefficient: Coefficient::NumInt(-3), degree: 0},
            Term {coefficient: Coefficient::NumFloat(1.0), degree: 1},
            Term {coefficient: Coefficient::NumInt(2), degree: 2},
        ];
        assert_eq!(degree_2_solution(&terms),
            "Discriminant is strictly positive, the two solutions are:\n1\n-1.5".to_string());
    }
}