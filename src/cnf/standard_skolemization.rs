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
use cnf::rename::rename;
use cnf::free_variables::fv;

pub fn skolemize(f: Formula) -> Formula {
    let mut n1 = 0;
    let mut n2 = 0;
    let renamed_f = rename(f, &mut n1);
    let skolemized_f = skolemize1(renamed_f, &mut n2);
    
    assert!(!contains_existential_quantifiers(skolemized_f.clone()));
    
    skolemized_f
}

fn skolemize1(f: Formula, n: &mut isize) -> Formula {
    match f {
        Formula::And(box p, box q) => Formula::And(box skolemize1(p, n), box skolemize1(q, n)),
        Formula::Or(box p, box q) => Formula::Or(box skolemize1(p, n), box skolemize1(q, n)),
        Formula::Forall(s, box p) => Formula::Forall(s, box skolemize1(p, n)),
        Formula::Exists(s, box p) => skolemize_exists(s, p, n),
        _ => f,
    }
}

fn skolemize_exists(s: String, f: Formula, n: &mut isize) -> Formula {
    let xs = fv(Formula::Exists(s.clone(), box f.clone())); 
    let sf = Term::Function(format!("sf{}", *n), xs.into_iter().map(|x| Term::Variable(x)).collect()); 
    *n += 1;
    skolemize1(tsubst(f, &s, sf), n)
}

fn tsubst(f: Formula, from: &str, to: Term) -> Formula {
    match f {
        Formula::Predicate(s, terms) => Formula::Predicate(s, terms.into_iter().map(|term| tsubst_variable(term, from, to.clone())).collect()),
        Formula::Not(box p) => Formula::Not(box tsubst(p, from, to)),
        Formula::And(box p, box q) => Formula::And(box tsubst(p, from, to.clone()), box tsubst(q, from, to)),
        Formula::Or(box p, box q) => Formula::Or(box tsubst(p, from, to.clone()), box tsubst(q, from, to)),
        Formula::Forall(s, box p) => Formula::Forall(s, box tsubst(p, from, to)),
        Formula::Exists(s, box p) => Formula::Exists(s, box tsubst(p, from, to)),
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
fn contains_existential_quantifiers(f: Formula) -> bool {
    match f {
        Formula::Predicate(_, _) | Formula::Not(_) => false,
        Formula::And(box p, box q) | Formula::Or(box p, box q) => contains_existential_quantifiers(p) || contains_existential_quantifiers(q),
        Formula::Forall(_, box p) => contains_existential_quantifiers(p),
        Formula::Exists(_, _) => true,
        _ => panic!("These should have been eliminated previously, something is SERIOUSLY wrong somewhere!"),
    }
}

#[cfg(test)]
mod test {
    use super::skolemize1;
    use parser::parser::parse;

    #[test]
    fn skolemize1_1() {
        let mut n = 0;
        let f = parse("forall v0. (R(v0, v0) /\\ exists v1. (P(v1) \\/ forall v2. exists v3. (R(v2, v3) \\/ forall v4. Q(v4))))").unwrap(); 
        let correct_f = parse("forall v0. (R(v0, v0) /\\ (P(sf0()) \\/ forall v2. (R(v2, sf1(v2)) \\/ forall v4. Q(v4))))").unwrap(); 
        assert_eq!(skolemize1(f, &mut n), correct_f);
    }
}    
