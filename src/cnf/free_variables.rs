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

use parser::formula::{Term, Formula};
use utils::set::Set;

/// Used for checking if a term t is free in a formula f.
pub fn free_in(f: &Formula, t: &Term) -> bool {
    match *f {
        Formula::True | Formula::False => false,
        Formula::Predicate(_, ref args) => args.iter().any(|x| occurs_in(x, t)),
        Formula::Not(box ref p) => free_in(&p, t),
        Formula::And(box ref p, box ref q) | Formula::Or(box ref p, box ref q) | 
        Formula::Implies(box ref p, box ref q) | Formula::Equivalent(box ref p, box ref q) => free_in(&p, t) || free_in(&q, t),
        Formula::Forall(ref s, box ref p) | Formula::Exists(ref s, box ref p) => !occurs_in(&Term::Variable(s.clone()), t) && free_in(&p, t),
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
pub fn fv(f: Formula) -> Set<String> {
    match f {
        Formula::True | Formula::False => Set::new(),
        Formula::Predicate(_, params) => params.into_iter().flat_map(|p| fvt(p)).collect(),
        Formula::Not(box p) => fv(p),
        Formula::And(box p, box q) | Formula::Or(box p, box q) | 
        Formula::Implies(box p, box q) | Formula::Equivalent (box p, box q) => fv(p).union(&fv(q)).cloned().collect(),
        Formula::Forall(s, box p) | Formula::Exists(s, box p) => { let mut lhs = fv(p); lhs.remove(&s); lhs },
    }
}

/// Get the free variables of a term.
fn fvt(t: Term) -> Set<String> {
    match t {
        Term::Variable(s) => Set::singleton(s),
        Term::Function(_, params) => params.into_iter().flat_map(|p| fvt(p)).collect(),
    }
}

#[cfg(test)]
mod test {
    use super::fv;
    use parser::parser::parse;
    
    #[test]
    fn fv_1() {
        let f = parse("P(f(g(x, y), g(y, z)))").unwrap();
        let free_variables = fv(f);
        assert_eq!(free_variables.cardinality(), 3);
        assert!(free_variables.contains("x"));
        assert!(free_variables.contains("y"));
        assert!(free_variables.contains("z"));
    }
    
    #[test]
    fn fv_2() {
        let f = parse("exists v. (P(c2) /\\ forall x. exists y. ((P(f(x), g(y)) \\/ Q(c, f(y), g(x))) \\/ R(z)))").unwrap();
        let free_variables = fv(f);
        assert_eq!(free_variables.cardinality(), 3);
        assert!(free_variables.contains("c"));
        assert!(free_variables.contains("c2"));
        assert!(free_variables.contains("z"));
    }
}    