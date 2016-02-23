/*
    Serkr - An automated theorem prover. Copyright (C) 2015-2016 Mikko Aarnos.

    Serkr is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Serkr is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Serkr. If not, see <http://www.gnu.org/licenses/>.
*/

/// All occurences of String are replaced with i64 for convenience.
/// We could theoretically have different ASTs for different phases of the CNF transformation.
/// However, it seems to me that some invariants are impossible to capture.

/// Data type for FOL terms. 
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
#[allow(missing_docs)]
#[allow(variant_size_differences)]
pub enum Term {
    Variable(i64),
    Function(i64, Vec<Term>)
}

/// Data type for FOL formulas. 
/// Notice how And and Or are basically vectors instead of a pair like Implies and Equivalent.
/// If we don't do this, large TPTP input files with a conjuction of 1000+ clauses cause stack overflows.
/// Notice how we do not provide a way to hash the Formula. 
/// Due to the need for And and Or to be commutative, there would have to be some serious hacks to the hashing to make everything work.
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub enum Formula {
    True,
    False,
    Predicate(i64, Vec<Term>),
    Not(Box<Formula>),
    And(Vec<Formula>),
    Or(Vec<Formula>),
    Implies(Box<Formula>, Box<Formula>),
    Equivalent(Box<Formula>, Box<Formula>),
    Forall(i64, Box<Formula>),
    Exists(i64, Box<Formula>),
}

impl PartialEq for Formula {
    fn eq(&self, other: &Formula) -> bool {
        match (self, other) {
            (&Formula::True, &Formula::True) | 
            (&Formula::False, &Formula::False) => true,
            (&Formula::Predicate(id1, ref args1), &Formula::Predicate(id2, ref args2)) => id1 == id2 && args1 == args2,
            (&Formula::And(ref l1), &Formula::And(ref l2)) |
            (&Formula::Or(ref l1), &Formula::Or(ref l2)) => formula_slice_equal_up_to_permutation(l1, l2),
            (&Formula::Not(ref f1), &Formula::Not(ref f2)) => f1 == f2,
            (&Formula::Implies(ref f_l_1, ref f_r_1), &Formula::Implies(ref f_l_2, ref f_r_2)) => f_l_1 == f_l_2 && f_r_1 == f_r_2,
            (&Formula::Equivalent(ref f_l_1, ref f_r_1), &Formula::Equivalent(ref f_l_2, ref f_r_2)) => (f_l_1 == f_l_2 && f_r_1 == f_r_2) || (f_l_1 == f_r_2 && f_r_1 == f_l_2),
            (&Formula::Forall(id1, ref f1), &Formula::Forall(id2, ref f2)) |
            (&Formula::Exists(id1, ref f1), &Formula::Exists(id2, ref f2)) => id1 == id2 && f1 == f2,
            (_, _) => false
        }
    }
}

/// Checks if two slices contain the same elements.
fn formula_slice_equal_up_to_permutation(l1: &[Formula], l2: &[Formula]) -> bool {
    if l1.len() != l2.len() {
        return false;
    }
    
    let mut used_flags = vec!(false; l1.len());
    
    for x in l1.iter() {
        let mut found_match = false;
    
        for (i, y) in l2.iter().enumerate() {
            if !used_flags[i] && x == y {
                used_flags[i] = true;
                found_match = true;
                break;
            }
        }
        
        if !found_match {
            return false;
        }
    }
    
    true
}

impl Eq for Formula {
}
