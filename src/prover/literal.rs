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

use utils::formula::Term;

/// A single, possibly negated, literal.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Literal {
    id: i64,
    args: Vec<Term>,
}

impl Literal {
    /// Creates a new dummy literal.
    pub fn new() -> Literal {
        Literal { id: 0, args: Vec::new() }
    }
    
    /// Creates a literal with a specific id and arguments.
    pub fn new_from_id_and_args(id: i64, args: Vec<Term>) -> Literal {
        Literal { id: id, args: args }
    }
    
    /// Creates a literal with a specific id. 
    /// Using this for anything is not recommended at all.
    pub fn new_from_id(id: i64) -> Literal {
        Literal { id: id, args: Vec::new() }
    }
    
    /// Used for checking if a literal is positive.
    pub fn is_positive(&self) -> bool {
        self.id > 0
    }
    
    /// Returns a negated version of the literal (taking into account double negation elimination).
    pub fn negate(&self) -> Literal {
        let mut lit = self.clone();
        lit.id = -lit.id;
        lit
    }
}