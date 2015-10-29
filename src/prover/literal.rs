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
use cnf::free_variables::fvt;
use utils::formula::Term;
use utils::set::Set;

/// A single, possibly negated, literal.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Literal {
    id: i64,
    args: Vec<Term>,
}

impl Literal {
    /// Creates a literal with a specific id. 
    pub fn new_from_id(id: i64) -> Literal {
        Literal { id: id, args: Vec::new() }
    }

    /// Creates a literal with a specific id and arguments.
    pub fn new_from_id_and_args(id: i64, args: Vec<Term>) -> Literal {
        Literal { id: id, args: args }
    }
    
    /// Used for checking if a literal is positive.
    pub fn is_positive(&self) -> bool {
        self.id > 0
    }
    
    /// Get the id of the literal.
    pub fn get_id(&self) -> i64 {
        self.id
    }
    
    /// Get the arity of the literal.
    pub fn get_arity(&self) -> usize {
        self.args.len()
    }
    
    /// Get the arguments of the literal.
    pub fn get_arguments(&self) -> Vec<Term> {
        self.args.clone()
    }
    
    /// Returns a negated version of the literal (taking into account double negation elimination).
    pub fn negate(&self) -> Literal {
        let mut lit = self.clone();
        lit.id = -lit.id;
        lit
    }
    
    /// Substitute terms in a literal according to a given mapping.
    pub fn tsubst(&mut self, sfn: &HashMap<Term, Term>) {
        for t in &mut self.args {
            *t = tsubst_variable(t.clone(), sfn);
        }
    }
    
    /// Get the variables used in the literal.
    pub fn variables(&self) -> Set<String> {
        self.args.iter().flat_map(|t| fvt(t.clone())).collect()
    }
}

// TODO: do something about this
fn tsubst_variable(t: Term, sfn: &HashMap<Term, Term>) -> Term {
    if let Some(t2) = sfn.get(&t) {
        t2.clone()
    } else {
        match t {
            Term::Variable(_) => t,
            Term::Function(s, args) => Term::Function(s, args.into_iter().map(|arg| tsubst_variable(arg, sfn)).collect())
        }
    }
}
