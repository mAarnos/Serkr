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

/// A single (possibly negated) Literal, or simply, a literal.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Literal {
    lhs: Term,
    rhs: Term,
    negated: bool
}

impl Literal {
    /// Creates a new Literal.
    pub fn new(negated: bool, lhs: Term, rhs: Term) -> Literal {
        Literal { lhs: lhs, rhs: rhs, negated: negated }
    }
    
    /// Used for checking if the Literal is positive.
    pub fn is_positive(&self) -> bool {
        !self.negated
    }
    
    /// Get a reference to the left hand side of the Literal.
    pub fn get_lhs(&self) -> &Term {
        &self.lhs
    }
    
    /// Get a reference to the right hand side of the Literal.
    pub fn get_rhs(&self) -> &Term {
        &self.rhs
    }
}

/// Checks if the lhs and rhs of the two given Literals match, taking into account symmetry.
pub fn terms_equal(l1: &Literal, l2: &Literal) -> bool {
    (l1.get_lhs() == l2.get_lhs() && l1.get_rhs() == l2.get_rhs()) ||
    (l1.get_lhs() == l2.get_rhs() && l1.get_rhs() == l2.get_lhs())
}

