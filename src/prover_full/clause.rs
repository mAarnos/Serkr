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

use prover_full::equation::Equation;

/// A single clause containing some amount of equations in no specific order.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Clause {
    equations: Vec<Equation>,
}

impl Clause {
    /// Creates an empty clause.
    pub fn new() -> Clause {
        Clause { equations: Vec::new() }
    }
    
    /// Get the amount of equations in the clause.
    pub fn size(&self) -> usize {
        self.equations.len()
    }
      
    /// Used for checking if the is empty.
    pub fn is_empty(&self) -> bool {
        self.equations.is_empty()
    }
}