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
                }
                term_step = TermStep::PlusMinus;
            },
            Elem::NumInt(n) => {
                if term_step >= TermStep::Value {
                    return Err("Incorrect syntax".to_string())
                }
                let value = match n.checked_mul(if is_plus {1} else {-1}) {
                    Some(temp) => temp,
                    None => return Err(format!("Overflow value {} * {}", -1, *n)),
                };
                coefficient = Coefficient::NumInt(value);
                term_step = TermStep::Value;
            },
            _ => {},
        }
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
