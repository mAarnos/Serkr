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

use utils::formula::{Term, Formula};
use cnf::rename::rename;
use cnf::free_variables::fv;

/// Eliminates existential quantifiers by replacing them with new skolem functions.
pub fn skolemize(f: Formula) -> Formula {
    let mut n1 = 0;
    let mut n2 = 0;
    let renamed_f = rename(f, &mut n1);
    let skolemized_f = skolemize1(renamed_f, &mut n2);
    assert!(!contains_existential_quantifiers(&skolemized_f));
    skolemized_f
}

fn skolemize1(f: Formula, n: &mut isize) -> Formula {
    match f {
        Formula::And(p, q) => Formula::And(Box::new(skolemize1(*p, n)), Box::new(skolemize1(*q, n))),
        Formula::Or(p, q) => Formula::Or(Box::new(skolemize1(*p, n)), Box::new(skolemize1(*q, n))),
        Formula::Forall(s, p) => Formula::Forall(s, Box::new(skolemize1(*p, n))),
        Formula::Exists(s, p) => skolemize_exists(s, *p, n),
        _ => f,
    }
}

fn skolemize_exists(s: String, f: Formula, n: &mut isize) -> Formula {
    let xs = fv(Formula::Exists(s.clone(), Box::new(f.clone()))); 
    let sf = Term::Function(format!("sf{}", *n), xs.into_iter().map(Term::Variable).collect()); 
    *n += 1;
    skolemize1(tsubst(f, &s, sf), n)
}

fn tsubst(f: Formula, from: &str, to: Term) -> Formula {
    match f {
        Formula::Predicate(s, terms) => Formula::Predicate(s, terms.into_iter().map(|term| tsubst_variable(term, from, to.clone())).collect()),
        Formula::Not(p) => Formula::Not(Box::new(tsubst(*p, from, to))),
        Formula::And(p, q) => Formula::And(Box::new(tsubst(*p, from, to.clone())), Box::new(tsubst(*q, from, to))),
        Formula::Or(p, q) => Formula::Or(Box::new(tsubst(*p, from, to.clone())), Box::new(tsubst(*q, from, to))),
        Formula::Forall(s, p) => Formula::Forall(s, Box::new(tsubst(*p, from, to))),
        Formula::Exists(s, p) => Formula::Exists(s, Box::new(tsubst(*p, from, to))),
        _ => f
    }
}

fn tsubst_variable(t: Term, from: &str, to: Term) -> Term {
    match t {
        Term::Variable(s) => if from == s { to } else { Term::Variable(s) },
        Term::Function(s, subterms) => Term::Function(s, subterms.into_iter().map(|term| tsubst_variable(term, from, to.clone())).collect())
    }
}

/// Checks if a given formula contains existential quantifiers. After skolemization this should not happen at all.
fn contains_existential_quantifiers(f: &Formula) -> bool {
    match *f {
        Formula::And(ref p, ref q) | Formula::Or(ref p, ref q) => contains_existential_quantifiers(&p) || contains_existential_quantifiers(&q),
        Formula::Forall(_, ref p) => contains_existential_quantifiers(&p),
        Formula::Exists(_, _) => true,
        _ => false
    }
}

#[cfg(test)]
mod test {
    use super::skolemize1;
    use parser::internal_parser::parse;

    #[test]
    fn skolemize1_1() {
        let mut n = 0;
        let f = parse("forall v0. (R(v0, v0) /\\ exists v1. (P(v1) \\/ forall v2. exists v3. (R(v2, v3) \\/ forall v4. Q(v4))))").unwrap(); 
        let correct_f = parse("forall v0. (R(v0, v0) /\\ (P(sf0()) \\/ forall v2. (R(v2, sf1(v2)) \\/ forall v4. Q(v4))))").unwrap(); 
        assert_eq!(skolemize1(f, &mut n), correct_f);
    }
}    
