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

use prover_full::term::Term;

/// A single, possibly negated, equation.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Equation {
    lhs: Term,
    rhs: Term,
    negated: bool
}

impl Equation {
    /// Creates a new equation.
    pub fn new(negated: bool, lhs: Term, rhs: Term) -> Equation {
        Equation { lhs: lhs, rhs: rhs, negated: negated }
    }
    
    /// Used for checking if the equation is positive.
    pub fn is_positive(&self) -> bool {
        !self.negated
    }
}
