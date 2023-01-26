use std::env;

mod elem;
mod parse_string;

use parse_string::parse_string;


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let equation_string = &args[1];

    let parsed_equation = parse_string(equation_string);
    println!("{:?}", parsed_equation);

    // for (i, c) in equation_string.chars().enumerate() {
    //     println!("{}: {}", i, c);
    // }

    // let mut index = 0;

    // while index < equation_string.len() {
    //     println!("{:?}", equation_string.chars().nth(index).unwrap());
    //     index += 1;
    // }
}
