use std::env;

mod term;
mod parse_string;
mod elem_to_term;
mod reduce_equation;
mod utility;
mod solution;
mod fraction;
mod math_utility;

use parse_string::parse_string;
use elem_to_term::elem_to_term;
use reduce_equation::reduce_equation;
use utility::{
    hash_terms_to_sorted_vec,
    make_reduced_form_string,
    evaluate_degree_of_terms,
};
use solution::solution;


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

    let equation_terms = match elem_to_term(parsed_equation) {
        Ok(v) => v,
        Err(s) => {
            eprintln!("{}", s);
            return;
        }
    };

    let terms = reduce_equation(&equation_terms.0, &equation_terms.1);

    let terms = hash_terms_to_sorted_vec(terms);

    println!("Reduced form: {}", make_reduced_form_string(&terms));

    let degree = evaluate_degree_of_terms(&terms);

    println!("Polynomial degree: {}", degree);

    if degree >= 3 {
        println!("The polynomial degree is strictly greater than 2, I can't solve.");
        return;
    }

    let solved_string = solution(&terms, degree);

    println!("{}", solved_string);
}
