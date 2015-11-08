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

/// A single term.
/// Functions are given a positive id, variables a negative one. 
/// The id zero is for a special function symbol representing truth.
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Term {
    id: i64,
    args: Vec<Term>,
}

impl Term {
    /// Creates a new term.
    pub fn new(id: i64, args: Vec<Term>) -> Term {
        Term { id: id, args: args }
    }
    
    /// Create a new term representing truth. 
    /// Used in the transformation of first order logic to pure equational logic.
    pub fn new_truth() -> Term {
        Term { id: 0, args: Vec::new() }
    }
    
    /// Get the id of the term.
    pub fn get_id(&self) -> i64 {
        self.id
    }

    /// Checks if this term is a function.
    pub fn is_function(&self) -> bool {
        self.id > 0
    }
    
    /// Checks if this term is a variable.
    pub fn is_variable(&self) -> bool {
        self.id < 0
    }
    
    /// Get the arguments of the term.
    pub fn get_args(&self) -> Vec<Term> {
        self.args.clone()
    }
    
    /// Checks if a given variable or function occurs in the term.
    pub fn occurs(&self, id: i64) -> bool {
        self.id == id || self.args.iter().any(|t| t.occurs(id))
    }
    
    /// Substitute all instances of the variable x with a given term.
    pub fn subst(&mut self, x: i64, t: &Term) {
        assert!(x < 0);
        if self.id == x {
            *self = t.clone();
        } else {
            for arg in &mut self.args {
                arg.subst(x, t);
            }
        }
    }
    
    /// Rename all variables in a term so that it has no variables in common with any other clause that the one it is a part of.
    pub fn rename_no_common(&mut self, sfn: &mut HashMap<i64, i64>, var_cnt: &mut i64) {
        if self.is_variable() {
            if let Some(&t) = sfn.get(&self.id) {
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
}

