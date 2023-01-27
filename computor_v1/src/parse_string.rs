use crate::term::Elem;


fn str_to_float(num_str: &String) -> Result<f64, String> {
    match num_str.parse() {
        Ok(n) => Ok(n),
        Err(_) => {
            Err("Conversion Failure".to_string())
        }
    }
}


fn str_to_int(num_str: &String) -> Result<i64, String> {
    match num_str.parse() {
        Ok(n) => Ok(n),
        Err(_) => {
            Err("Conversion Failure".to_string())
        }
    }
}


fn update_vec_str_to_num(is_float: &mut bool, num_str: &mut String, vec: &mut Vec<Elem>) -> Result<(), String> {
    if *is_float {
        *is_float = false;
        match str_to_float(&num_str) {
            Ok(n) => vec.push(Elem::NumFloat(n)),
            Err(s) => return Err(s),
        };
    } else {
        match str_to_int(&num_str) {
            Ok(n) => vec.push(Elem::NumInt(n)),
            Err(s) => return Err(s),
        };
    }
    num_str.clear();
    Ok(())
}


fn update_vec_char_to_elem_except_num(c: char, vec: &mut Vec<Elem>) -> Result<(), String> {
    match c {
        'X' => vec.push(Elem::X),
        '+' => vec.push(Elem::Plus),
        '-' => vec.push(Elem::Minus),
        '*' => vec.push(Elem::Prod),
        '^' => vec.push(Elem::Power),
        '=' => vec.push(Elem::Equal),
        ' ' => {},
        _ => {
            return Err(format!("Unsupported characters: {}", c));
        }
    };
    Ok(())
}


pub fn parse_string(equation: &String) -> Result<Vec<Elem>, String> {
    let mut vec = Vec::new();
    let mut num_str = String::new();
    let mut is_float = false;

    for c in equation.chars() {
        match c {
            '0'..='9' => num_str.push(c),
            '.' => {
                if is_float {
                    return Err("With multiple decimal places".to_string());
                }
                is_float = true;
                num_str.push(c);
            },
            _ => {
                if num_str.len() > 0 {
                    update_vec_str_to_num(&mut is_float, &mut num_str, &mut vec)?;
                }
                update_vec_char_to_elem_except_num(c, &mut vec)?;
            }
        }
    }
    if num_str.len() > 0 {
        update_vec_str_to_num(&mut is_float, &mut num_str, &mut vec)?;
    }
    return Ok(vec);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_to_float_normal() {
        assert_eq!(str_to_float(&"123.456".to_string()), Ok(123.456));
    }

    #[test]
    fn str_to_float_not_number() {
        assert_eq!(str_to_float(&"a".to_string()), Err("Conversion Failure".to_string()));
    }

    #[test]
    fn str_to_int_normal() {
        assert_eq!(str_to_int(&"123".to_string()), Ok(123));
    }

    #[test]
    fn str_to_int_float() {
        assert_eq!(str_to_int(&"123.456".to_string()), Err("Conversion Failure".to_string()));
    }

    #[test]
    fn str_to_int_not_number() {
        assert_eq!(str_to_int(&"a".to_string()), Err("Conversion Failure".to_string()));
    }

    #[test]
    fn str_to_int_maximum_plus() {
        assert_eq!(str_to_int(&"9223372036854775808".to_string()), Err("Conversion Failure".to_string()));
    }

    #[test]
    fn str_to_int_minimum_minus() {
        assert_eq!(str_to_int(&"-9223372036854775809".to_string()), Err("Conversion Failure".to_string()));
    }

    #[test]
    fn parse_string_normal() {
        use Elem::*;
        assert_eq!(parse_string(&"5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0".to_string()), Ok(vec![
            NumInt(5), Prod, X, Power, NumInt(0), Plus, NumInt(4), Prod, X, Power, NumInt(1), Minus, NumFloat(9.3), Prod, X, Power, NumInt(2), Equal, NumInt(1), Prod, X, Power, NumInt(0)]));
    }

    #[test]
    fn parse_string_unacceptable_value() {
        assert_eq!(parse_string(&"123a".to_string()), Err("Unsupported characters: a".to_string()));
    }

    #[test]
    fn parse_string_int_maximum_plus() {
        assert_eq!(parse_string(&"9223372036854775808".to_string()), Err("Conversion Failure".to_string()));
    }

    #[test]
    fn parse_string_two_dot() {
        assert_eq!(parse_string(&"123..456".to_string()), Err("With multiple decimal places".to_string()));
    }
}
