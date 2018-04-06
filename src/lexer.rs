use std::iter::Peekable;
use std::str::Chars;

/// Molecule syntax tokens recognizable by our Lexer
#[derive(Debug, Clone)]
pub enum Token {
    Bracket(char),
    Atom(String),
    Index(usize),
}

/// Lex a string slice into a Vector of Tokens
pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut result = Vec::new();

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            'A'...'Z' => {
                it.next();
                let n = lex_atom(c, &mut it);
                result.push(Token::Atom(n));
            }
            '0'...'9' => {
                it.next();
                let n = lex_index(c, &mut it);
                result.push(Token::Index(n));
            }
            '(' | ')' | '[' | ']' | '{' | '}' => {
                result.push(Token::Bracket(c));
                it.next();
            }
            _ => {
                return Err(format!("Unexpected character {}", c));
            }
        }
    }
    Ok(result)
}

/// Lex an atom
fn lex_atom(c: char, iter: &mut Peekable<Chars>) -> String {
    let mut atom = c.to_string();
    while let Some(Ok(character)) = iter.peek()
        .map(|c| {
            match *c {
                'a'...'z' => Ok(c.to_string()),
                _ => Err(())
            }
        }) {
        atom = format!("{}{}", atom, character);
        iter.next();
    }
    atom
}

/// Lex atom index
fn lex_index(c: char, iter: &mut Peekable<Chars>) -> usize {
    let mut number = c.to_string().parse().expect("Expected digit");
    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<usize>()) {
        number = number * 10 + digit;
        iter.next();
    }
    number
}