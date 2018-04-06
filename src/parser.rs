use lexer::Token;
use model::{Atom, Molecule, add_atoms, mul_molecule};

/// Grammar nodes supported by our syntax tree
#[derive(Debug,Clone)]
enum Grammar {
    Atom(Atom),
    Index(usize)
}

/// Tree structure that represents a molecule
#[derive(Debug,Clone)]
struct ParseNode {
    children: Vec<ParseNode>,
    entry: Grammar,
}

/// Tree structure to represent a molecule
impl ParseNode {
    pub fn new(children: Vec<ParseNode>, entry: Grammar) -> ParseNode {
        ParseNode {
            children,
            entry,
        }
    }

    /// Convert the structure into a vector of Atoms (Molecule)
    pub fn flatten(&self) -> Molecule {
        match self.entry {
            Grammar::Index(ref index) => {
                let molecule: Molecule = self.children
                    .iter()
                    .cloned()
                    .fold(vec![], |molecule, child| {
                        [&molecule[..], &child.clone().flatten()[..]].concat()
                    });
                mul_molecule(&molecule, *index)
            },
            Grammar::Atom(ref atom) => vec![atom.clone()]
        }
    }
}

/// After flattening the tree we can end up with a result similar to this
/// ```
/// [("K", 4), ("O", 2), ("N", 2), ("S", 4), ("O", 12)]
/// ```
/// where oxygen has two entries.
///
/// After running this function we end up with:
/// ```
/// [("K", 4), ("O", 14), ("N", 2), ("S", 4)]
/// ```
fn merge_atoms(molecule: &Molecule) -> Molecule {
    let mut deduplicated_atoms: Molecule = vec![];
    for atom in molecule {
        let mut add = true;
        deduplicated_atoms = deduplicated_atoms.iter()
            .map(|new_atom|{
                if atom.0 == new_atom.0 {
                    add = false;
                    add_atoms(&atom, new_atom.1)
                } else {
                    new_atom.clone()
                }
            })
            .collect();
        if add {
            deduplicated_atoms.push(atom.clone())
        }
    }
    deduplicated_atoms
}

/// Build a ParseNode tree from tokens
fn parse_atoms(tokens: &[Token], pos: usize) -> Result<(ParseNode, usize), String> {
    let mut cur_pos = pos;
    let mut children = vec![];

    loop {
        let (parse_node, new_pos) = match tokens.get(cur_pos) {
            Some(&Token::Atom(_)) => {
                parse_atom(&tokens, cur_pos)?
            },
            Some(&Token::Bracket('(')) | Some(&Token::Bracket('[')) | Some(&Token::Bracket('{')) => {
                parse_group(&tokens, cur_pos)?
            }
            _ => break
        };

        children.push(parse_node);
        cur_pos = new_pos;
        if cur_pos == tokens.len() {
            break;
        }
    }
    let result = ParseNode::new(children, Grammar::Index(1));
    Ok((result, cur_pos))
}

/// Parse exactly one atom and its index (if present)
///
/// ```
/// Example K4[ON(SO3)2]2
///            ^^ ^ These are three atoms without indices
///         ^^     ^^ These atoms have indices
/// ```
fn parse_atom(tokens: &[Token], pos: usize) -> Result<(ParseNode, usize), String> {
    if let Some(&Token::Atom(ref a)) = tokens.get(pos) {
        parse_index(tokens, pos + 1).and_then(|(index, next_pos)| {
            let atom = Grammar::Atom((a.clone(), index));
            let parse_node = ParseNode::new(vec![], atom);
            Ok((parse_node, next_pos))
        })
    } else {
        Err(format!("Unexpected token {:?}", tokens.get(pos)))
    }
}

/// Parse a group of atoms that start with an opening bracket and end with either a closing bracket
/// or with an index after the closing bracket
///
/// ```
/// Example K4[ON(SO3)2]2
///              ^^^^^^ This is a group of atoms
///           ^^^^^^^^^^^ This is also a group that contains another group inside of it
/// ```
fn parse_group(tokens: &[Token], pos: usize) -> Result<(ParseNode, usize), String> {
    parse_open_bracket(tokens, pos).and_then(|(c, next_pos)| {
        parse_atoms(tokens, next_pos).and_then(|(node, next_pos)| {
            parse_close_bracket(tokens, next_pos, c).and_then(|next_pos| {
                parse_index(tokens, next_pos).and_then(|(index, next_pos)| {
                    let parse_node = ParseNode::new(vec![node], Grammar::Index(index));
                    Ok((parse_node, next_pos))
                })
            })
        })
    })
}

/// Parse an opening bracket for a group of atoms or returns an error.
///
/// ```
/// Example K4[ON(SO3)2]2
///           ^  ^ These two are opening brackets
/// ```
fn parse_open_bracket(tokens: &[Token], pos: usize) -> Result<(char, usize), String> {
    if let Some(&Token::Bracket(c)) = tokens.get(pos) {
        match c {
            '(' | '[' | '{' => Ok((c, pos + 1)),
            _ => Err(format!("Expected opening bracket at {} but found {:?}", pos, c))
        }
    } else {
        Err(format!("Unexpected token {:?}", tokens.get(pos)))
    }
}

/// Parse a closing bracket for a group of atoms or returns an error.
///
/// ```
/// Example K4[ON(SO3)2]2
///                  ^ ^ These two are closing brackets
/// ```
fn parse_close_bracket(tokens: &[Token], pos: usize, c: char) -> Result<usize, String> {
    if let Some(&Token::Bracket(c2)) = tokens.get(pos) {
        if c2 == matching(c) {
            Ok(pos + 1)
        } else {
            Err(format!("Expected {} but found {} at {}", matching(c), c2, pos))
        }
    } else {
        Err(format!("Expected closing bracket at {} but found {:?}", pos, tokens.get(pos)))
    }
}

/// Find matching bracket
fn matching(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '(' => ')',
        '[' => ']',
        '{' => '}',
        _ => panic!("Expected parenthesis, but found {}", c),
    }
}

/// Parse the index of an atom or a group of atoms.
///
/// ```
/// Example K4[ON(SO3)2]2
///                   ^ ^ These are group indices
///          ^      ^ These are atom indices
/// ```
/// If there is one we parse it. Otherwise assume it is 1.
fn parse_index(tokens: &[Token], pos: usize) -> Result<(usize, usize), String> {
    if let Some(&Token::Index(n)) = tokens.get(pos) {
        Ok((n, pos + 1))
    } else {
        Ok((1, pos))
    }
}

pub fn parse_molecule(s: &str) -> Result<Molecule, String> {
    let tokens = ::lexer::lex(s)?;
    let (atoms, pos) = parse_atoms(&tokens, 0)?;
    if pos != tokens.len() {
        return Err("Not all tokens were parsed".to_string());
    }
    Ok(merge_atoms(&atoms.flatten()))
}


#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_parse {
        ($molecule:expr, $atoms:expr, $name:ident) => {
            #[test]
            fn $name() {
                assert_parse($molecule, $atoms.to_vec());
            }
        };
    }

    assert_parse!("H", [("H",1)], hydrogen);
    assert_parse!("O2", [("O",2)], oxygen);
    assert_parse!("H2O", [("H",2),("O",1)], water);
    assert_parse!("K4[ON(SO3)2]2", [("K",4),("O",14),("N",2),("S",4)], fremys_salt);
    assert_parse!("Mg(OH)2", [("Mg",1),("O",2),("H",2)], magnesium_hydroxide);

    #[test]
    fn test_fails() {
        assert_fail("pie");
        assert_fail("Mg(OH");
        assert_fail("Mg(OH}2");
    }

    fn assert_parse(molecule: &str, expected: Vec<(&str, usize)>) {
        let atoms = parse_molecule(molecule);
        assert_eq!(molecules_compare(atoms.as_ref().unwrap(), &expected), true);
    }

    fn molecules_compare(va: &Vec<(String, usize)>, vb: &Vec<(&str, usize)>) -> bool {
        (va.len() == vb.len()) &&
            va.iter()
                .zip(vb)
                .all(|(a,b)| a.0 == b.0 && a.1 == b.1)
    }

    fn assert_fail(molecule: &str) {
        let atoms = parse_molecule(molecule);
        assert!(atoms.is_err());
    }
}
