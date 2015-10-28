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

use prover::literal::Literal;
use std::slice::Iter;

/// A single clause containing some amount of literals in no specific order.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Clause {
    literals: Vec<Literal>,
}

impl Clause {
    /// Creates an empty clause.
    pub fn new() -> Clause {
        Clause { literals: Vec::new() }
    }
    
    /// Get the amount of literals in the clause.
    pub fn size(&self) -> usize {
        self.literals.len()
    }
    
    /// Creates a new clause from a vector of literals.
    pub fn new_from_vec(lits: Vec<Literal>) -> Clause {
        Clause { literals: lits }
    }
      
    /// Used for checkign whether a clause is empty.
    pub fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }
    
    /// Removes a literal from the clause. Since we don't care about ordering we use swap_remove.
    /// Using this requires some care. A good example of correct use is found in factoring.rs
    pub fn swap_remove(&mut self, index: usize) {
        self.literals.swap_remove(index);
    }
    
    /// Used for iterating the literals of the clause.
    pub fn iter(&self) -> Iter<Literal> {
        self.literals.iter()
    }
    
    /// Add the literals in a given clause to this clause (without checking for duplicates).
    pub fn add_literals(&mut self, mut cl: Clause) {
        self.literals.append(&mut cl.literals);
    }
    
    /// Get the literal at a given index.
    pub fn at(&self, index: usize) -> Literal {
        self.literals[index].clone()
    }
}