use crate::term::{Elem, Coefficient, Term};


fn check_and_push_term(elems: &Vec<Elem>, terms: &mut Vec<Term>) -> Result<(), String> {
    let coefficient = Coefficient::NumInt(1);
    for (i, elem) in elems.iter().enumerate() {
        
    }

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
                    // check elem stock
                    elem_stock.clear();
                    elem_stock.push(elem);
                }
            }
            Equal => {
                if is_left {
                    is_left = false;
                } else {
                    return Err("There were multiple equals".to_string());
                }
            },
            _ => elem_stock.push(elem),
        }
    }
    if !elem_stock.is_empty() {
        // check elem stock
    }
    if is_left {
        return Err("There was no equal".to_string());
    }
    return Ok((vec_left, vec_right));
}
