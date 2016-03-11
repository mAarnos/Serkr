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

use std::collections::HashMap;
use std::iter::IntoIterator;
use std::slice::{Iter, IterMut};
use std::ops::{Index, IndexMut};
use std::fmt::{Debug, Formatter, Error};
use prover::unification::substitution::Substitution;

/// A single term.
/// Functions are given a positive id, variables a negative one.
/// The id zero is for a special function symbol representing truth.
#[derive(Eq, PartialEq, Clone, Hash)]
pub struct Term {
    id: i64,
    // Terms are in two sorts, VF and P, which cannot be unified together
    // TODO: move this info somewhere else?
    sort_predicate: bool,
    args: Vec<Term>,
}

impl Term {
    /// Creates a new normal function. Note that the ID passed in should be negative.
    pub fn new_function(id: i64, args: Vec<Term>) -> Term {
        assert!(id > 0);
        Term {
            id: id,
            sort_predicate: false,
            args: args,
        }
    }

    /// Creates a new special function. Used in the elimination of non-equality literals.
    pub fn new_special_function(id: i64, args: Vec<Term>) -> Term {
        assert!(id > 0);
        Term {
            id: id,
            sort_predicate: true,
            args: args,
        }
    }

    /// Creates a new variable. Note that the ID passed in should be negative.
    pub fn new_variable(id: i64) -> Term {
        assert!(id < 0);
        Term {
            id: id,
            sort_predicate: false,
            args: Vec::new(),
        }
    }

    /// Create a new term representing truth. Used in the elimination of non-equality literals.
    pub fn new_truth() -> Term {
        Term {
            id: 0,
            sort_predicate: true,
            args: Vec::new(),
        }
    }

    /// Get the id of the term.
    pub fn get_id(&self) -> i64 {
        self.id
    }

    /// Get the arity of the term.
    pub fn get_arity(&self) -> usize {
        self.args.len()
    }

    /// Checks if this term is a function.
    pub fn is_function(&self) -> bool {
        self.id >= 0
    }

    /// Check if the term is a special function.
    pub fn is_special_function(&self) -> bool {
        self.sort_predicate
    }

    /// Checks if this term is a variable.
    pub fn is_variable(&self) -> bool {
        self.id < 0
    }

    /// Checks if a given term occurs in the term.
    pub fn occurs(&self, term: &Term) -> bool {
        self == term || self.args.iter().any(|t| t.occurs(term))
    }

    /// Checks if a given variable or function occurs in the term as a subterm.
    pub fn occurs_proper(&self, term: &Term) -> bool {
        self.args.iter().any(|t| t.occurs(term))
    }

    /// Substitute all instances of the term s with a term t.
    pub fn subst_single(&mut self, s: &Term, t: &Term) {
        if self == s {
            *self = t.clone();
        } else {
            for arg in &mut self.args {
                arg.subst_single(s, t);
            }
        }
    }

    /// Substitutes according to the mapping.
    pub fn subst(&mut self, substitution: &Substitution) {
        if let Some(t) = substitution.get(&self) {
            *self = t.clone();
        } else {
            for x in &mut self.args {
                x.subst(substitution);
            }
        }
    }

    /// Rename all variables in a term so that it has no variables in common with other clauses.
    pub fn rename_no_common(&mut self, sfn: &mut HashMap<i64, i64>, var_cnt: &mut i64) {
        if self.is_variable() {
            if let Some(&t) = sfn.get(&self.id) {
                assert!(t < 0);
                self.id = t;
            } else {
                *var_cnt -= 1;
                sfn.insert(self.id, *var_cnt);
                self.id = *var_cnt;
            }
        } else {
            for arg in &mut self.args {
                arg.rename_no_common(sfn, var_cnt);
            }
        }
    }

    /// Get the amount of symbols in this term.
    // TODO: move somewhere else.
    pub fn symbol_count(&self) -> usize {
        self.args.iter().fold(1, |acc, t| acc + t.symbol_count())
    }

    /// Used for iterating the subterms of a term.
    pub fn iter(&self) -> Iter<Term> {
        self.args.iter()
    }

    /// Used for iterating the literals of the clause with the option of mutating them.
    pub fn iter_mut(&mut self) -> IterMut<Term> {
        self.args.iter_mut()
    }
}

impl Index<usize> for Term {
    type Output = Term;

    fn index(&self, index: usize) -> &Term {
        &self.args[index]
    }
}

impl IndexMut<usize> for Term {
    fn index_mut(&mut self, index: usize) -> &mut Term {
        &mut self.args[index]
    }
}

impl IntoIterator for Term {
    type Item = Term;
    type IntoIter = ::std::vec::IntoIter<Term>;

    /// Consumes the term and returns an iterator over the subterms.
    fn into_iter(self) -> Self::IntoIter {
        self.args.into_iter()
    }
}

impl Debug for Term {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        if self.get_id() == 0 {
            write!(formatter, "T")
        } else if self.is_function() {
            if self.get_arity() == 0 {
                write!(formatter, "c_{}", self.get_id())
            } else {
                try!(write!(formatter, "f_{}(", self.get_id()));
                for (i, st) in self.iter().enumerate() {
                    try!(write!(formatter, "{:?}", st));
                    if i != self.get_arity() - 1 {
                        try!(write!(formatter, ", "));
                    }
                }
                write!(formatter, ")")
            }
        } else {
            write!(formatter, "x_{}", -self.get_id())
        }
    }
}
