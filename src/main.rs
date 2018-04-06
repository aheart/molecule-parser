use std::env;

mod lexer;
mod model;
mod parser;

fn main() {
    let molecule = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please provide molecule as argument");
        ::std::process::exit(1);
    });

    println!("Atoms: {:?}", parser::parse_molecule(&molecule));
}
