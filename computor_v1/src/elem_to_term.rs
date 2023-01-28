use crate::term::{Elem, Coefficient, Term};


#[derive(PartialEq, PartialOrd)]
enum TermStep {
    Start,
    PlusMinus,
    Value,
    Prod,
    Variable,
    Power,
    Degree,
}


fn check_and_push_term(elems: &Vec<Elem>, terms: &mut Vec<Term>) -> Result<(), String> {
    // Wip detailed syntax error
    // Wip flexible format support
    let mut coefficient = Coefficient::NumInt(1);
    let mut degree = 0;
    let mut is_plus = true;
    let mut term_step = TermStep::Start;
    for elem in elems {
        match elem {
            Elem::Minus | Elem::Plus => {
                if term_step >= TermStep::PlusMinus {
                    return Err("Incorrect syntax".to_string())
                }
                if let Elem::Minus = elem {
                    is_plus = false;
                    coefficient = Coefficient::NumInt(-1);
                }
                term_step = TermStep::PlusMinus;
            },
            Elem::NumInt(n) => {
                if term_step == TermStep::Power {
                    degree = *n;
                    term_step = TermStep::Degree;
                } else {
                    if term_step >= TermStep::Value {
                        return Err("Incorrect syntax".to_string())
                    }
                    let value = match n.checked_mul(if is_plus {1} else {-1}) {
                        Some(temp) => temp,
                        None => return Err(format!("Overflow value {} * {}", -1, *n)),
                    };
                    coefficient = Coefficient::NumInt(value);
                    term_step = TermStep::Value;
                }
            },
            Elem::NumFloat(n) => {
                if term_step >= TermStep::Value {
                    return Err("Incorrect syntax".to_string())
                }
                // wip overflow
                // let value = match n.checked_mul(if is_plus {1} else {-1}) {
                //     Some(temp) => temp,
                //     None => return Err(format!("Overflow value {} * {}", -1, *n)),
                // };
                let value = n * if is_plus {1 as f64} else {-1 as f64};
                coefficient = Coefficient::NumFloat(value);
                term_step = TermStep::Value;
            },
            Elem::Prod => {
                if term_step != TermStep::Value {
                    return Err("Incorrect syntax".to_string())
                }
                term_step = TermStep::Prod;
            },
            Elem::X => {
                if term_step >= TermStep::Variable {
                    return Err("Incorrect syntax".to_string())
                }
                degree = 1;
                term_step = TermStep::Variable;
            },
            Elem::Power => {
                if term_step != TermStep::Variable {
                    return Err("Incorrect syntax".to_string())
                }
                term_step = TermStep::Power;
            },
            _ => {},
        }
    }
    if term_step != TermStep::Value && term_step != TermStep::Variable && term_step != TermStep::Degree {
        return Err("Incorrect syntax".to_string())
    }
    terms.push(Term {coefficient, degree});
    return Ok(());
}


pub fn elem_to_term(elems: Vec<Elem>) -> Result<(Vec<Term>, Vec<Term>), String> {
    let mut vec_left = Vec::new();
    let mut vec_right = Vec::new();
    let mut elem_stock = Vec::new();
    let mut is_left = true;

    for elem in elems {
        match elem {
            Elem::Plus | Elem::Minus => {
                if elem_stock.len() == 0 {
                    elem_stock.push(elem);
                } else {
                    check_and_push_term(&elem_stock,
                        if is_left {&mut vec_left} else {&mut vec_right})?;
                    elem_stock.clear();
                    elem_stock.push(elem);
                }
            }
            Elem::Equal => {
                if is_left {
                    check_and_push_term(&elem_stock,
                        if is_left {&mut vec_left} else {&mut vec_right})?;
                    elem_stock.clear();
                    elem_stock.push(elem);
                    is_left = false;
                } else {
                    return Err("There were multiple equals".to_string());
                }
            },
            _ => elem_stock.push(elem),
        }
    }
    if !elem_stock.is_empty() {
        check_and_push_term(&elem_stock, &mut vec_right)?;
    }
    if is_left {
        return Err("There was no equal".to_string());
    }
    return Ok((vec_left, vec_right));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_and_push_term_x() {
        let elems = vec![Elem::X];
        let mut terms = Vec::new();
        let result = check_and_push_term(&elems, &mut terms);
        assert_eq!(result, Ok(()));
        assert_eq!(terms[0], Term {coefficient: Coefficient::NumInt(1), degree: 1});
    }

    #[test]
    fn check_and_push_term_int() {
        let elems = vec![Elem::NumInt(2)];
        let mut terms = Vec::new();
        let result = check_and_push_term(&elems, &mut terms);
        assert_eq!(result, Ok(()));
        assert_eq!(terms[0], Term {coefficient: Coefficient::NumInt(2), degree: 0});
    }

    #[test]
    fn check_and_push_term_float() {
        let elems = vec![Elem::NumFloat(2.0)];
        let mut terms = Vec::new();
        let result = check_and_push_term(&elems, &mut terms);
        assert_eq!(result, Ok(()));
        assert_eq!(terms[0], Term {coefficient: Coefficient::NumFloat(2.0), degree: 0});
    }

    #[test]
    fn check_and_push_term_plus_int() {
        let elems = vec![Elem::Plus, Elem::NumInt(2)];
        let mut terms = Vec::new();
        let result = check_and_push_term(&elems, &mut terms);
        assert_eq!(result, Ok(()));
        assert_eq!(terms[0], Term {coefficient: Coefficient::NumInt(2), degree: 0});
    }

    #[test]
    fn check_and_push_term_minus_int() {
        let elems = vec![Elem::Minus, Elem::NumInt(2)];
        let mut terms = Vec::new();
        let result = check_and_push_term(&elems, &mut terms);
        assert_eq!(result, Ok(()));
        assert_eq!(terms[0], Term {coefficient: Coefficient::NumInt(-2), degree: 0});
    }

    #[test]
    fn check_and_push_term_plus_float() {
        let elems = vec![Elem::Plus, Elem::NumFloat(2.0)];
        let mut terms = Vec::new();
        let result = check_and_push_term(&elems, &mut terms);
        assert_eq!(result, Ok(()));
        assert_eq!(terms[0], Term {coefficient: Coefficient::NumFloat(2.0), degree: 0});
    }

    #[test]
    fn check_and_push_term_minus_float() {
        let elems = vec![Elem::Minus, Elem::NumFloat(2.0)];
        let mut terms = Vec::new();
        let result = check_and_push_term(&elems, &mut terms);
        assert_eq!(result, Ok(()));
        assert_eq!(terms[0], Term {coefficient: Coefficient::NumFloat(-2.0), degree: 0});
    }

    #[test]
    fn check_and_push_term_plus_x() {
        let elems = vec![Elem::Plus, Elem::X];
        let mut terms = Vec::new();
        let result = check_and_push_term(&elems, &mut terms);
        assert_eq!(result, Ok(()));
        assert_eq!(terms[0], Term {coefficient: Coefficient::NumInt(1), degree: 1});
    }

    #[test]
    fn check_and_push_term_minus_x() {
        let elems = vec![Elem::Minus, Elem::X];
        let mut terms = Vec::new();
        let result = check_and_push_term(&elems, &mut terms);
        assert_eq!(result, Ok(()));
        assert_eq!(terms[0], Term {coefficient: Coefficient::NumInt(-1), degree: 1});
    }

    #[test]
    fn check_and_push_term_prod_int() {
        let elems = vec![Elem::Minus, Elem::NumInt(2), Elem::Prod, Elem::X];
        let mut terms = Vec::new();
        let result = check_and_push_term(&elems, &mut terms);
        assert_eq!(result, Ok(()));
        assert_eq!(terms[0], Term {coefficient: Coefficient::NumInt(-2), degree: 1});
    }

    #[test]
    fn check_and_push_term_prod_float() {
        let elems = vec![Elem::Minus, Elem::NumFloat(2.0), Elem::Prod, Elem::X];
        let mut terms = Vec::new();
        let result = check_and_push_term(&elems, &mut terms);
        assert_eq!(result, Ok(()));
        assert_eq!(terms[0], Term {coefficient: Coefficient::NumFloat(-2.0), degree: 1});
    }

    #[test]
    fn check_and_push_term_non_prod_int() {
        let elems = vec![Elem::Minus, Elem::NumInt(2), Elem::X];
        let mut terms = Vec::new();
        let result = check_and_push_term(&elems, &mut terms);
        assert_eq!(result, Ok(()));
        assert_eq!(terms[0], Term {coefficient: Coefficient::NumInt(-2), degree: 1});
    }

    #[test]
    fn check_and_push_term_non_prod_float() {
        let elems = vec![Elem::Minus, Elem::NumFloat(2.0), Elem::X];
        let mut terms = Vec::new();
        let result = check_and_push_term(&elems, &mut terms);
        assert_eq!(result, Ok(()));
        assert_eq!(terms[0], Term {coefficient: Coefficient::NumFloat(-2.0), degree: 1});
    }
    
    #[test]
    fn check_and_push_term_power() {
        let elems = vec![Elem::X, Elem::Power, Elem::NumInt(2)];
        let mut terms = Vec::new();
        let result = check_and_push_term(&elems, &mut terms);
        assert_eq!(result, Ok(()));
        assert_eq!(terms[0], Term {coefficient: Coefficient::NumInt(1), degree: 2});
    }

    #[test]
    fn check_and_push_term_full() {
        let elems = vec![Elem::Minus, Elem::NumFloat(2.0), Elem::Prod, Elem::X, Elem::Power, Elem::NumInt(2)];
        let mut terms = Vec::new();
        let result = check_and_push_term(&elems, &mut terms);
        assert_eq!(result, Ok(()));
        assert_eq!(terms[0], Term {coefficient: Coefficient::NumFloat(-2.0), degree: 2});
    }

    #[test]
    fn check_and_push_term_error_only_plus() {
        let elems = vec![Elem::Plus];
        let mut terms = Vec::new();
        let result = check_and_push_term(&elems, &mut terms);
        assert_eq!(result, Err("Incorrect syntax".to_string()));
    }

    #[test]
    fn check_and_push_term_error_only_minus() {
        let elems = vec![Elem::Minus];
        let mut terms = Vec::new();
        let result = check_and_push_term(&elems, &mut terms);
        assert_eq!(result, Err("Incorrect syntax".to_string()));
    }

    #[test]
    fn check_and_push_term_error_only_prod() {
        let elems = vec![Elem::Prod];
        let mut terms = Vec::new();
        let result = check_and_push_term(&elems, &mut terms);
        assert_eq!(result, Err("Incorrect syntax".to_string()));
    }

    #[test]
    fn check_and_push_term_error_only_power() {
        let elems = vec![Elem::Power];
        let mut terms = Vec::new();
        let result = check_and_push_term(&elems, &mut terms);
        assert_eq!(result, Err("Incorrect syntax".to_string()));
    }

    #[test]
    fn check_and_push_term_error_two_plus() {
        let elems = vec![Elem::Plus, Elem::Plus];
        let mut terms = Vec::new();
        let result = check_and_push_term(&elems, &mut terms);
        assert_eq!(result, Err("Incorrect syntax".to_string()));
    }
}