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

use std::slice::{Iter, IterMut};
use prover_full::equation::Equation;

/// A multiset containing equations.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Clause {
    equations: Vec<Equation>,
}

impl Clause {
    /// Creates a new clause.
    pub fn new(equations: Vec<Equation>) -> Clause {
        Clause { equations: equations }
    }
    
    /// Get the amount of equations in the clause.
    pub fn size(&self) -> usize {
        self.equations.len()
    }
      
    /// Used for checking if the clause is empty.
    pub fn is_empty(&self) -> bool {
        self.equations.is_empty()
    }
    
    /// Used for iterating the equations of the clause.
    pub fn iter(&self) -> Iter<Equation> {
        self.equations.iter()
    }
    
    /// Used for iterating the equations of the clause with the option of mutating them.
    pub fn iter_mut(&mut self) -> IterMut<Equation> {
        self.equations.iter_mut()
    }
}