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

use std::collections::hash_set::HashSet;
use cnf::ast::{Term, Formula};

/// Used for checking if a term t is free in a formula f.
pub fn free_in(f: &Formula, t: &Term) -> bool {
    match *f {
        Formula::True | Formula::False => false,
        Formula::Predicate(_, ref args) => args.iter().any(|x| occurs_in(x, t)),
        Formula::Not(ref p) => free_in(p, t),
        Formula::And(ref p, ref q) | Formula::Or(ref p, ref q) | 
        Formula::Implies(ref p, ref q) | Formula::Equivalent(ref p, ref q) => free_in(p, t) || free_in(q, t),
        Formula::Forall(id, ref p) | Formula::Exists(id, ref p) => !occurs_in(&Term::Variable(id), t) && free_in(p, t),
    }
}

/// Used for checking if a term s occurs as a subterm in a term t.
pub fn occurs_in(t: &Term, s: &Term) -> bool {
    t == s || match *t {
        Term::Variable(_) => false,
        Term::Function(_, ref args) => args.iter().any(|x| occurs_in(x, s)),
    }
}

/// Get the free variables of a formula.
pub fn fv(f: &Formula) -> HashSet<i64> {
    match *f {
        Formula::True | Formula::False => HashSet::new(),
        Formula::Predicate(_, ref params) => params.iter().flat_map(fvt).collect(),
        Formula::Not(ref p) => fv(p),
        Formula::And(ref p, ref q) | Formula::Or(ref p, ref q) | 
        Formula::Implies(ref p, ref q) | Formula::Equivalent (ref p, ref q) => fv(p).union(&fv(q)).cloned().collect(),
        Formula::Forall(id, ref p) | Formula::Exists(id, ref p) => { let mut lhs = fv(p); lhs.remove(&id); lhs },
    }
}

/// Get the free variables of a term.
pub fn fvt(t: &Term) -> HashSet<i64> {
    match *t {
        Term::Variable(id) => { let mut set = HashSet::new(); set.insert(id); set },
        Term::Function(_, ref params) => params.iter().flat_map(fvt).collect(),
    }
}

#[cfg(test)]
mod test {
    use super::fv;
    use cnf::ast_transformer_internal::internal_to_cnf_ast;
    
    // These tests aren't really that robust.
    // If they fail the most likely reason is some change to the parser.
    // TODO: add tests for free_in
    
    #[test]
    fn fv_1() {
        let (f, _) = internal_to_cnf_ast("P(f(g(x, y), g(y, z)))").unwrap();
        let free_variables = fv(&f);
        assert_eq!(free_variables.len(), 3);
        assert!(free_variables.contains(&-1));
        assert!(free_variables.contains(&-2));
        assert!(free_variables.contains(&-3));
    }
    
    #[test]
    fn fv_2() {
        let (f, _) = internal_to_cnf_ast("exists v. (P(c2) /\\ forall x. exists y. ((P(f(x), g(y)) \\/ Q(c, f(y), g(x))) \\/ R(z)))").unwrap();
        let free_variables = fv(&f);
        assert_eq!(free_variables.len(), 3);
        assert!(free_variables.contains(&-2));
        assert!(free_variables.contains(&-5));
        assert!(free_variables.contains(&-6));
    }
}    