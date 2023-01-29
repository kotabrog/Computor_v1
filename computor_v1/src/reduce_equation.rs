use std::collections::HashMap;

use crate::term::{Term, Coefficient};


// fn check_degree_one_terms(terms: &Vec<Term>) -> (i64, i64) {
//     let mut min_degree = 0;
//     let mut max_degree = 0;
//     for term in terms {
//         min_degree = min_degree.min(term.degree);
//         max_degree = max_degree.max(term.degree);
//     }
//     (min_degree, max_degree)
// }


// fn check_degree(left_terms: &Vec<Term>, right_terms: &Vec<Term>) -> (i64, i64) {
//     let (left_min_degree, left_max_degree) = check_degree_one_terms(&left_terms);
//     let (right_min_degree, right_max_degree) = check_degree_one_terms(&right_terms);
//     (left_min_degree.min(right_min_degree), left_max_degree.max(right_max_degree))
// }


fn update_terms_coefficient(terms: &mut HashMap<i64, Term>, term: &Term, is_right: bool) {
    let value = terms.entry(term.degree)
        .or_insert(Term { coefficient: Coefficient::NumInt(0), degree: term.degree});
    let add_value = if is_right {term.coefficient.mul_minus()} else {term.coefficient.clone()};
    value.coefficient = value.coefficient.add(&add_value);
}


pub fn reduce_equation(left_terms: &Vec<Term>, right_terms: &Vec<Term>) -> HashMap<i64, Term> {
    let mut terms = HashMap::new();
    for term in left_terms {
        update_terms_coefficient(&mut terms, &term, false);
    }
    for term in right_terms {
        update_terms_coefficient(&mut terms, &term, true);
    }
    terms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduce_equation_empty(){
        let left_vec = Vec::new();
        let right_vec = Vec::new();
        assert_eq!(reduce_equation(&left_vec, &right_vec), HashMap::new());
    }

    #[test]
    fn reduce_equation_only_left(){
        let left_vec = vec![Term {
                coefficient: Coefficient::NumInt(2),
                degree: 1,
            },
            Term {
                coefficient: Coefficient::NumFloat(-1.2),
                degree: 0,
            }];
        let right_vec = Vec::new();
        let mut hash_map = HashMap::new();
        hash_map.insert(0, Term {
            coefficient: Coefficient::NumFloat(-1.2),
            degree: 0,
        });
        hash_map.insert(1, Term {
            coefficient: Coefficient::NumInt(2),
            degree: 1,
        });
        assert_eq!(reduce_equation(&left_vec, &right_vec), hash_map);
    }

    #[test]
    fn reduce_equation_only_right(){
        let left_vec = Vec::new();
        let right_vec = vec![Term {
                coefficient: Coefficient::NumInt(2),
                degree: 1,
            },
            Term {
                coefficient: Coefficient::NumFloat(-1.2),
                degree: 0,
        }];
        let mut hash_map = HashMap::new();
        hash_map.insert(0, Term {
            coefficient: Coefficient::NumFloat(1.2),
            degree: 0,
        });
        hash_map.insert(1, Term {
            coefficient: Coefficient::NumInt(-2),
            degree: 1,
        });
        assert_eq!(reduce_equation(&left_vec, &right_vec), hash_map);
    }

    #[test]
    fn reduce_equation_left_and_right(){
        let left_vec = vec![Term {
                coefficient: Coefficient::NumFloat(3.6),
                degree: 1,
            },
            Term {
                coefficient: Coefficient::NumFloat(-1.2),
                degree: 3,
        }];
        let right_vec = vec![Term {
                coefficient: Coefficient::NumInt(2),
                degree: 1,
            },
            Term {
                coefficient: Coefficient::NumFloat(-1.2),
                degree: 0,
        }];
        let mut hash_map = HashMap::new();
        hash_map.insert(0, Term {
            coefficient: Coefficient::NumFloat(1.2),
            degree: 0,
        });
        hash_map.insert(1, Term {
            coefficient: Coefficient::NumFloat(1.6),
            degree: 1,
        });
        hash_map.insert(3, Term {
            coefficient: Coefficient::NumFloat(-1.2),
            degree: 3,
        });
        assert_eq!(reduce_equation(&left_vec, &right_vec), hash_map);
    }

    // #[test]
    // fn check_degree_one_terms_zero() {
    //     let vec = Vec::new();
    //     assert_eq!(check_degree_one_terms(&vec), (0, 0));
    // }

    // #[test]
    // fn check_degree_one_terms_one() {
    //     let vec = vec![Term {coefficient: Coefficient::NumInt(1), degree: 1}];
    //     assert_eq!(check_degree_one_terms(&vec), (0, 1));
    // }

    // #[test]
    // fn check_degree_one_terms_two() {
    //     let vec = vec![Term {coefficient: Coefficient::NumInt(1), degree: 3}, Term {coefficient: Coefficient::NumInt(2), degree: 1}];
    //     assert_eq!(check_degree_one_terms(&vec), (0, 3));
    // }

    // #[test]
    // fn check_degree_zero() {
    //     let vec_left = Vec::new();
    //     let vec_right = Vec::new();
    //     assert_eq!(check_degree(&vec_left, &vec_right), (0, 0));
    // }

    // #[test]
    // fn check_degree_one_and_zero() {
    //     let vec_left = vec![Term {coefficient: Coefficient::NumInt(1), degree: 1}];;
    //     let vec_right = Vec::new();
    //     assert_eq!(check_degree(&vec_left, &vec_right), (0, 1));
    // }

    // #[test]
    // fn check_degree_one_and_two() {
    //     let vec_left = vec![Term {coefficient: Coefficient::NumInt(1), degree: 3}];;
    //     let vec_right = vec![Term {coefficient: Coefficient::NumInt(1), degree: -1}, Term {coefficient: Coefficient::NumInt(2), degree: 2}];
    //     assert_eq!(check_degree(&vec_left, &vec_right), (-1, 3));
    // }
}