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
use prover_full::literal::Literal;

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
}

impl Index<usize> for Clause {
    type Output = Literal;

    fn index(&self, index: usize) -> &Literal {
        &self.literals[index]
    }
}