use std::{collections::HashMap, hash::Hash};

use crate::term::{Term, Coefficient, self};


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


fn update_terms_coefficient(terms: &mut HashMap<i64, Term>, term: &Term, is_right: bool) -> Result<(), String> {
    let value = terms.entry(term.degree)
        .or_insert(Term { coefficient: Coefficient::NumInt(0), degree: term.degree});
    let add_value = if is_right {term.coefficient.mul_minus()} else {term.coefficient.clone()};
    value.coefficient = value.coefficient.checked_add(&add_value)?;
    Ok(())
}


pub fn reduce_equation(left_terms: &Vec<Term>, right_terms: &Vec<Term>) -> Result<HashMap<i64, Term>, String> {
    let mut terms = HashMap::new();
    for term in left_terms {
        update_terms_coefficient(&mut terms, &term, false)?;
    }
    for term in right_terms {
        update_terms_coefficient(&mut terms, &term, true)?;
    }
    Ok(terms)

    // let (min_degree, max_degree) = check_degree(&left_terms, &right_terms);

    // for i in min_degree..=max_degree {
    //     terms.push(Term {coefficient: Coefficient::NumInt(0), degree: i})
    // }

    // for term in left_terms {
    //     let index = term.degree - min_degree;
    //     // terms[index as usize];
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduce_equation_empty(){
        let left_vec = Vec::new();
        let right_vec = Vec::new();
        assert_eq!(reduce_equation(&left_vec, &right_vec), Ok(HashMap::new()));
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