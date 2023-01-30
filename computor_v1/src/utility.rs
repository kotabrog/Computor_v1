use std::collections::HashMap;

use crate::term::{Term, Coefficient};


pub fn hash_terms_to_sorted_vec(terms: HashMap<i64, Term>) -> Vec<Term> {
    let mut vec: Vec<Term> = terms.into_values().collect();
    vec.sort_by(|a, b| a.degree.cmp(&b.degree));
    vec
}


pub fn make_reduced_form_string(terms: &Vec<Term>) -> String {
    let mut string = String::new();
    for term in terms {
        string += match term.coefficient {
            Coefficient::NumInt(n) => {
                if n < 0 {
                    format!("- {} * X^{} ", -n, term.degree)
                } else if string.is_empty() {
                    format!("{} * X^{} ", n, term.degree)
                } else {
                    format!("+ {} * X^{} ", n, term.degree)
                }
            },
            
            Coefficient::NumFloat(n) => {
                if n < 0.0 {
                    format!("- {} * X^{} ", -n, term.degree)
                } else if string.is_empty() {
                    format!("{} * X^{} ", n, term.degree)
                } else {
                    format!("+ {} * X^{} ", n, term.degree)
                }
            },
        }.as_str();
    }
    if string.is_empty() {
        string += "0 ";
    }
    string += "= 0";
    string
}


pub fn evaluate_degree_of_terms(terms: &Vec<Term>) -> i64 {
    let mut degree = 0;
    for term in terms {
        if !term.coefficient.is_zero() {
            degree = degree.max(term.degree);
        }
    }
    degree
}


pub fn is_int_value(v: f64) -> bool {
    let int_v = v as i64;
    v - int_v as f64 == 0.0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_terms_to_sorted_vec_empty() {
        let terms = HashMap::new();
        let vec = Vec::new();
        assert_eq!(hash_terms_to_sorted_vec(terms), vec);
    }

    #[test]
    fn hash_terms_to_sorted_vec_one() {
        let mut terms = HashMap::new();
        terms.insert(1, Term { coefficient: Coefficient::NumInt(2), degree: (1) });
        let vec = vec![Term { coefficient: Coefficient::NumInt(2), degree: (1) }];
        assert_eq!(hash_terms_to_sorted_vec(terms), vec);
    }

    #[test]
    fn hash_terms_to_sorted_vec_three() {
        let mut terms = HashMap::new();
        terms.insert(1, Term { coefficient: Coefficient::NumInt(1), degree: 1 });
        terms.insert(3, Term { coefficient: Coefficient::NumInt(2), degree: 3 });
        terms.insert(2, Term { coefficient: Coefficient::NumInt(3), degree: 2 });
        let vec = vec![
            Term { coefficient: Coefficient::NumInt(1), degree: 1 },
            Term { coefficient: Coefficient::NumInt(3), degree: 2 },
            Term { coefficient: Coefficient::NumInt(2), degree: 3 },];
        assert_eq!(hash_terms_to_sorted_vec(terms), vec);
    }

    #[test]
    fn make_reduced_form_string_empty() {
        let vec = Vec::new();
        assert_eq!(make_reduced_form_string(&vec), "0 = 0".to_string());
    }

    #[test]
    fn make_reduced_form_string_zero_int() {
        let vec = vec![Term { coefficient: Coefficient::NumInt(0), degree: (1) }];
        assert_eq!(make_reduced_form_string(&vec), "0 * X^1 = 0".to_string());
    }

    #[test]
    fn make_reduced_form_string_zero_float() {
        let vec = vec![Term { coefficient: Coefficient::NumFloat(0.0), degree: (1) }];
        assert_eq!(make_reduced_form_string(&vec), "0 * X^1 = 0".to_string());
    }

    #[test]
    fn make_reduced_form_string_first_plus() {
        let vec = vec![Term { coefficient: Coefficient::NumInt(1), degree: (1) }];
        assert_eq!(make_reduced_form_string(&vec), "1 * X^1 = 0".to_string());
    }

    #[test]
    fn make_reduced_form_string_three_terms_int() {
        let vec = vec![
            Term { coefficient: Coefficient::NumInt(-1), degree: (0) },
            Term { coefficient: Coefficient::NumInt(-2), degree: (1) },
            Term { coefficient: Coefficient::NumInt(1), degree: (2) },
        ];
        assert_eq!(make_reduced_form_string(&vec), "- 1 * X^0 - 2 * X^1 + 1 * X^2 = 0".to_string());
    }

    #[test]
    fn make_reduced_form_string_three_terms_float() {
        let vec = vec![
            Term { coefficient: Coefficient::NumFloat(-1.2), degree: (0) },
            Term { coefficient: Coefficient::NumFloat(-2.2), degree: (1) },
            Term { coefficient: Coefficient::NumFloat(1.2), degree: (2) },
        ];
        assert_eq!(make_reduced_form_string(&vec), "- 1.2 * X^0 - 2.2 * X^1 + 1.2 * X^2 = 0".to_string());
    }

    #[test]
    fn evaluate_degree_of_terms_empty() {
        let vec = Vec::new();
        assert_eq!(evaluate_degree_of_terms(&vec), 0);
    }

    #[test]
    fn evaluate_degree_of_terms_one() {
        let vec = vec![Term { coefficient: Coefficient::NumInt(1), degree: (1) }];
        assert_eq!(evaluate_degree_of_terms(&vec), 1);
    }

    #[test]
    fn evaluate_degree_of_terms_in_zero() {
        let vec = vec![
            Term { coefficient: Coefficient::NumFloat(-1.2), degree: (0) },
            Term { coefficient: Coefficient::NumFloat(2.1), degree: (1) },
            Term { coefficient: Coefficient::NumFloat(0.0), degree: (2) },
        ];
        assert_eq!(evaluate_degree_of_terms(&vec), 1);
    }

    #[test]
    fn is_int_value_int() {
        assert_eq!(is_int_value(1.0), true);
    }

    #[test]
    fn is_int_value_float() {
        assert_eq!(is_int_value(1.1), false);
    }
}