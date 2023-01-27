use std::env;

mod term;
mod parse_string;
mod elem_to_term;

use parse_string::parse_string;
use elem_to_term::elem_to_term;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please specify one argument");
        return;
    }

    let equation_string = &args[1];

    let parsed_equation = match parse_string(equation_string) {
        Ok(v) => v,
        Err(s) => {
            eprintln!("{}", s);
            return;
        }
    };

    let terms = elem_to_term(parsed_equation);
    println!("{:?}", terms)
}
