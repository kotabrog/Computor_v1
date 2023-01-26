use crate::elem::Elem;


pub fn parse_string(equation: &String) -> Vec<Elem> {
    let mut vec = Vec::new();
    let mut num_str = String::new();
    let mut is_float = false;

    for c in equation.chars() {
        match c {
            '0'..='9' => num_str.push(c),
            '.' => {
                if is_float {
                    // error処理
                    panic!("With multiple decimal places");
                }
                is_float = true;
                num_str.push(c);
            },
            _ => {
                if num_str.len() > 0 {
                    if is_float {
                        is_float = false;
                        let num: f64 = match num_str.parse() {
                            Ok(n) => n,
                            Err(_) => {
                                // error処理
                                panic!("Conversion Failure");
                            }
                        };
                        vec.push(Elem::NumFloat(num));
                    } else {
                        let num: i64 = match num_str.parse() {
                            Ok(n) => n,
                            Err(_) => {
                                // error処理
                                panic!("Conversion Failure");
                            }
                        };
                        vec.push(Elem::NumInt(num));
                    }
                    num_str.clear();
                }
                match c {
                    'X' => vec.push(Elem::X),
                    '+' => vec.push(Elem::Plus),
                    '-' => vec.push(Elem::Minus),
                    '*' => vec.push(Elem::Prod),
                    '^' => vec.push(Elem::Power),
                    '=' => vec.push(Elem::Equal),
                    ' ' => {},
                    _ => {
                        // error処理
                        panic!("Unsupported characters: {}", c);
                    }
                }
            }
        }
    }

    return vec;
}