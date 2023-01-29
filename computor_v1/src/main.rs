use std::env;

mod term;
mod parse_string;
mod elem_to_term;
mod reduce_equation;

use parse_string::parse_string;
use elem_to_term::elem_to_term;
use reduce_equation::reduce_equation;


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

    println!("{:?}", parsed_equation);

    let mut equation_terms = match elem_to_term(parsed_equation) {
        Ok(v) => v,
        Err(s) => {
            eprintln!("{}", s);
            return;
        }
    };

    println!("{:?}", equation_terms);

    let terms = reduce_equation(&equation_terms.0, &equation_terms.1);

    println!("{:?}", terms);
}
