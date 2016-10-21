// Serkr - An automated theorem prover. Copyright (C) 2015-2016 Mikko Aarnos.
//
// Serkr is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Serkr is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Serkr. If not, see <http://www.gnu.org/licenses/>.
//

use std::fmt::{Debug, Formatter, Error};
use std::iter::Iterator;
use prover::data_structures::term::Term;
use prover::unification::substitution::Substitution;

/// A single (possibly negated) equation, or simply, a literal.
/// Note that this has custom `PartialEq` and `Eq`. 
/// If we add stuff to here remember to change them too.
#[derive(Clone)]
pub struct Literal {
    lhs: Term,
    rhs: Term,
    negated: bool,
}

impl Literal {
    /// Creates a new literal.
    pub fn new(negated: bool, lhs: Term, rhs: Term) -> Literal {
        Literal {
            lhs: lhs,
            rhs: rhs,
            negated: negated,
        }
    }

    /// Checks if the literal is positive.
    pub fn is_positive(&self) -> bool {
        !self.negated
    }

    /// Checks if the literal is negative.
    pub fn is_negative(&self) -> bool {
        self.negated
    }

    /// Get a reference to the left hand side of the literal.
    pub fn get_lhs(&self) -> &Term {
        &self.lhs
    }

    /// Get a reference to the right hand side of the literal.
    pub fn get_rhs(&self) -> &Term {
        &self.rhs
    }

    /// Get a mutable reference to the left hand side of the literal.
    pub fn get_lhs_mut(&mut self) -> &mut Term {
        &mut self.lhs
    }

    /// Get a mutable reference to the right hand side of the literal.
    pub fn get_rhs_mut(&mut self) -> &mut Term {
        &mut self.rhs
    }

    /// Substitutes variables in the literal according to the substitution.
    pub fn subst(&mut self, substitution: &Substitution) {
        self.lhs.subst(substitution);
        self.rhs.subst(substitution);
    }
    
    /// Calculates the symbol count with given weights to function and variable symbols.
    pub fn symbol_count(&self, f_value: u64, v_value: u64) -> u64 {
        self.get_lhs().symbol_count(f_value, v_value) + 
        self.get_rhs().symbol_count(f_value, v_value)
    }

    /// Checks if the given literal has the same polarity as this one.
    pub fn polarity_equal(&self, l: &Literal) -> bool {
        self.is_positive() == l.is_positive()
    }

    /// Checks if the given literal has the same terms, taking into account symmetry.
    pub fn terms_equal(&self, l: &Literal) -> bool {
        (self.lhs == l.lhs && self.rhs == l.rhs) || (self.lhs == l.rhs && self.rhs == l.lhs)
    }

    /// Used for iterating through the lhs and rhs of the literal.
    pub fn iter(&self) -> Iter {
        Iter {
            literal: self,
            index: 0,
        }
    }
}

pub struct Iter<'a> {
    literal: &'a Literal,
    index: u8,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Term;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => Some(self.literal.get_lhs()),
            1 => Some(self.literal.get_rhs()),
            _ => None,
        };
        self.index += 1;

        result
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Literal) -> bool {
        self.polarity_equal(other) && self.terms_equal(other)
    }
}

impl Eq for Literal {}

impl Debug for Literal {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        let eqn_sign = if self.is_positive() {
            "="
        } else {
            "<>"
        };
        write!(formatter, "{:?} {} {:?}", self.lhs, eqn_sign, self.rhs)
    }
}
