pub type Atom = (String, usize);
pub type Molecule = Vec<Atom>;

/// Increase the index by a number
/// add_atoms((H, 2), 1) -> (H, 3)
pub fn add_atoms(atom: &Atom, number: usize) -> Atom {
    (atom.0.clone(), atom.1 + number)
}

/// Multiply the index by a number
/// mul_atoms((H, 2), 2) -> (H, 4)
fn mul_atoms(atom: &Atom, multiplier: usize) -> Atom {
    (atom.0.clone(), atom.1 * multiplier)
}

/// Multiply the indices of all atoms in a vector of Atoms (Molecule)
pub fn mul_molecule(molecule: &Molecule, multiplier: usize) -> Molecule {
    molecule.iter()
        .map(|atom| {
            mul_atoms(atom, multiplier)
        })
        .collect()
}