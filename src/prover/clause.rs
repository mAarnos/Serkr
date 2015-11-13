/*
    Serkr - An automated theorem prover. Copyright (C) 2015 Mikko Aarnos.

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

use std::ops::Index;
use std::slice::{Iter, IterMut};
use std::collections::HashMap;
use std::cmp::Ordering;
use prover::term::Term;
use prover::literal::Literal;

/// A multiset containing literals.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Clause {
    literals: Vec<Literal>,
}

impl Clause {
    /// Creates a new clause.
    pub fn new(literals: Vec<Literal>) -> Clause {
        Clause { literals: literals }
    }
    
    /// Get the amount of literals in the clause.
    pub fn size(&self) -> usize {
        self.literals.len()
    }
      
    /// Used for checking if the clause is empty.
    pub fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }
    
    /// Used for iterating the literals of the clause.
    pub fn iter(&self) -> Iter<Literal> {
        self.literals.iter()
    }
    
    /// Used for iterating the literals of the clause with the option of mutating them.
    pub fn iter_mut(&mut self) -> IterMut<Literal> {
        self.literals.iter_mut()
    }
    
    /// Removes a literal from the clause. Since we don't care about ordering we use swap_remove.
    /// Using this requires some care. A good example of correct use is found in tautology_deletion.rs
    pub fn swap_remove(&mut self, index: usize) {
        self.literals.swap_remove(index);
    }
    
    /// Add the literal to the clause (without checking for duplicates).
    pub fn add_literal(&mut self, l: Literal) {
        self.literals.push(l);
    }
    
    /// Add the literals in a given clause to this clause (without checking for duplicates).
    pub fn add_literals(&mut self, mut cl: Clause) {
        self.literals.append(&mut cl.literals);
    }
    
    /// Substitutes terms in the clause according to the mapping.
    pub fn subst(&mut self, sfn: &HashMap<Term, Term>) {
        for l in &mut self.literals {
            l.subst(sfn);
        }
    }
    
    /// Get the amount of symbols in this clause.
    pub fn symbol_count(&self) -> usize {
        self.literals.iter().fold(0, |acc, l| acc + l.symbol_count())
    }
}

impl Index<usize> for Clause {
    type Output = Literal;

    fn index(&self, index: usize) -> &Literal {
        &self.literals[index]
    }
}

/// The priority queue depends on "Ord".
/// We order clauses in heuristic order, not in anything concrete.
impl Ord for Clause {
    fn cmp(&self, other: &Clause) -> Ordering {
        other.symbol_count().cmp(&self.symbol_count())
    }
}

/// "PartialOrd" needs to be implemented as well.
impl PartialOrd for Clause {
    fn partial_cmp(&self, other: &Clause) -> Option<Ordering> {
        Some(other.symbol_count().cmp(&self.symbol_count()))
    }
}