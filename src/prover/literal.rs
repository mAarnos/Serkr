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

use std::collections::HashMap;
use prover::term::Term;

/// A single (possibly negated) equation, or simply, a literal.
/// Note: has custom PartialEq and Eq: if you add stuff to here remember to change them too.
#[derive(Debug, Clone)]
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
    
    /// Used for checking if the literal is positive.
    pub fn is_positive(&self) -> bool {
        !self.negated
    }
    
    /// Used for checking if the literal is negative.
    pub fn is_negative(&self) -> bool {
        self.negated
    }
    
    /// Get a reference to the left hand side of the Literal.
    pub fn get_lhs(&self) -> &Term {
        &self.lhs
    }
    
    /// Get a reference to the right hand side of the Literal.
    pub fn get_rhs(&self) -> &Term {
        &self.rhs
    }
    
    pub fn rename_no_common(&mut self, sfn: &mut HashMap<i64, i64>, var_cnt: &mut i64) {
        self.lhs.rename_no_common(sfn, var_cnt);
        self.rhs.rename_no_common(sfn, var_cnt);
    }
    
    /// Substitutes terms in the literal according to the mapping.
    pub fn subst(&mut self, sfn: &HashMap<Term, Term>) {
        self.lhs.subst_general(sfn);
        self.rhs.subst_general(sfn);
    }
    
    /// Get the amount of symbols in this literal
    pub fn symbol_count(&self) -> usize {
        1 + self.lhs.symbol_count() + self.rhs.symbol_count()
    }
    
    /// 
    pub fn to_string(&self) -> String {
        let eqn_sign = if self.is_positive() { " = " } else { " <> " }; 
        self.get_lhs().to_string() + eqn_sign + &self.get_rhs().to_string()
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Literal) -> bool {
        self.negated == other.negated && terms_equal(self, other)
    }
}

impl Eq for Literal {
}

/// Checks if the lhs and rhs of the two given Literals match, taking into account symmetry.
pub fn terms_equal(l1: &Literal, l2: &Literal) -> bool {
    (l1.get_lhs() == l2.get_lhs() && l1.get_rhs() == l2.get_rhs()) ||
    (l1.get_lhs() == l2.get_rhs() && l1.get_rhs() == l2.get_lhs())
}

