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

/// A single term.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Term {
    id: i64,
    args: Vec<Term>,
}

impl Term {
    /// Checks if this term is a function.
    pub fn is_function(&self) -> bool {
        self.id > 0
    }
    
    /// Checks if this term is a variable.
    pub fn is_variable(&self) -> bool {
        self.id < 0
    }
}

