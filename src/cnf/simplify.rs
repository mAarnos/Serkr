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

use std::collections::HashSet;
use cnf::ast::{Term, Formula};
use cnf::free_variables::free_in;

/// Simplifies a formula by performing some equivalence-preserving first-order simplifications.
/// After this the formula is either True or False, or doesn't contain True and False.
pub fn simplify_formula(f: Formula) -> Formula {
    let simplified_formula = simplify(f);
    assert!(is_simplified(&simplified_formula));
    simplified_formula
}

/// The main function for simplifying the formula from bottom up.
fn simplify(f: Formula) -> Formula {
    match f {
        Formula::Not(p) => simplify_not(*p),
        Formula::And(l) => simplify_and(l),
        Formula::Or(l) => simplify_or(l),
        Formula::Implies(p, q) => simplify_implies(*p, *q),
        Formula::Equivalent(p, q) => simplify_equivalent(*p, *q),
        Formula::Forall(id, p) => simplify_quantifier(id, *p, true),
        Formula::Exists(id, p) => simplify_quantifier(id, *p, false),
        _ => f,
    }
}

/// "not true" can be simplified to "false".
/// "not false" can be simplified to "true".
fn simplify_not(f: Formula) -> Formula {
    let simplified_f = simplify(f);
    match simplified_f {
        Formula::True => Formula::False,
        Formula::False => Formula::True,
        _ => Formula::Not(Box::new(simplified_f)),
    }
}

/// Negates a formula. Applies double negation elimination.
fn negate(f: Formula) -> Formula {
    match f {
        Formula::Not(p) => *p,
        _ => Formula::Not(Box::new(f)),
    }
}

/// "p and p" can be simplified to "p".
/// "p and not p" can be simplified to "false".
/// "p and true" can be simplified to "p"
/// "p and false" can be simplified to "false".
fn simplify_and(l: Vec<Formula>) -> Formula {
    // Duplicates are automatically removed thanks to us using a set.
    let simplified_l = l.into_iter()
                        .map(simplify)
                        .filter(|x| *x != Formula::True)
                        .collect::<HashSet<_>>();

    if simplified_l.contains(&Formula::False) {
        Formula::False
    } else {
        for p in simplified_l.iter() { 
            let p_negated = negate(p.clone());
            if simplified_l.contains(&p_negated) {
                return Formula::False;
            }
        }
        
        if simplified_l.is_empty() {
            Formula::True
        } else if simplified_l.len() == 1 {
            if let Some(f) = simplified_l.into_iter().next() {
                f
            } else {
                panic!("This shouldn't be possible, check the above asserts")
            }
        } else {
            Formula::And(simplified_l.into_iter().collect())
        }
    }
}

/// "p or p" can be simplified to "p".
/// "p or not p" can be simplified to "true".
/// "p or true" can be simplified to "true"
/// "p or false" can be simplified to "p".
fn simplify_or(l: Vec<Formula>) -> Formula {
    let simplified_l = l.into_iter()
                            .map(simplify)
                            .filter(|x| *x != Formula::False)
                            .collect::<HashSet<_>>();

    if simplified_l.contains(&Formula::True) {
        Formula::True
    } else {
        for p in simplified_l.iter() { 
            let p_negated = negate(p.clone());
            if simplified_l.contains(&p_negated) {
                return Formula::True;
            }
        }

        if simplified_l.is_empty() {
            Formula::False
        } else if simplified_l.len() == 1 {
            if let Some(f) = simplified_l.into_iter().next() {
                f
            } else {
                panic!("This shouldn't be possible, check the above asserts")
            }
        } else {
            Formula::Or(simplified_l.into_iter().collect())
        }
    }
}

/// "p implies p" can be simplified to "true".
/// "p implies true" can be simplified to "true".
/// "p implies false" can be simplified to "not p".
/// "true implies p" can be simplified to "p".
/// "false implies p" can be simplified to "true".
fn simplify_implies(f1: Formula, f2: Formula) -> Formula {
    let simplified_f = (simplify(f1), simplify(f2));
    match simplified_f {
        (_, Formula::True) | (Formula::False, _) => Formula::True,
        (Formula::True, p) => p,
        (p, Formula::False) => simplify(Formula::Not(Box::new(p))),
        (p, q) => {
            if p == q {
                Formula::True
            } else {
                Formula::Implies(Box::new(p), Box::new(q))
            }
        }
    }
}

/// "p equivalent true" can be rewritten as "p".
/// "p equivalent false" can be rewritten as "not p".
/// "p equivalent p" can be rewritten as "true".
fn simplify_equivalent(f1: Formula, f2: Formula) -> Formula {
    let simplified_f = (simplify(f1), simplify(f2));
    match simplified_f {
        (p, Formula::True) | (Formula::True, p) => p,
        (p, Formula::False) | (Formula::False, p) => simplify(Formula::Not(Box::new(p))),
        (p, q) => {
            if p == q {
                Formula::True
            } else {
                Formula::Equivalent(Box::new(p), Box::new(q))
            }
        }
    }
}

/// "forall x. p" and "exists x. p" can be written as "p" if x does not occur free in p.
fn simplify_quantifier(id: i64, f: Formula, forall: bool) -> Formula {
    let simplified_f = simplify(f);
    if free_in(&simplified_f, &Term::Variable(id)) {
        if forall {
            Formula::Forall(id, Box::new(simplified_f))
        } else {
            Formula::Exists(id, Box::new(simplified_f))
        }
    } else {
        simplified_f
    }
}

/// Checks if a formula has been simplified.
/// That is, it is either True or False or doesn't contain True or False.
fn is_simplified(f: &Formula) -> bool {
    if *f == Formula::True || *f == Formula::False {
        true
    } else {
        !contains_true_or_false(f)
    }
}

/// Used for checking if a formula contains "true" or "false".
fn contains_true_or_false(f: &Formula) -> bool {
    match *f {
        Formula::True | Formula::False => true,
        Formula::Predicate(_, _) => false,
        Formula::And(ref l) | Formula::Or(ref l) => l.iter().any(|x| contains_true_or_false(x)),
        Formula::Implies(ref p, ref q) | Formula::Equivalent(ref p, ref q) => {
            contains_true_or_false(p) || contains_true_or_false(q)
        }
        Formula::Not(ref p) | Formula::Forall(_, ref p) | Formula::Exists(_, ref p) => {
            contains_true_or_false(p)
        }
    }
}

#[cfg(test)]
mod test {
    use cnf::ast::{Term, Formula};
    use super::{simplify_not, simplify_and, simplify_or, simplify_implies, simplify_equivalent,
                simplify_quantifier};

    #[test]
    fn simplify_not_1() {
        assert_eq!(simplify_not(Formula::True), Formula::False);
    }

    #[test]
    fn simplify_not_2() {
        assert_eq!(simplify_not(Formula::False), Formula::True);
    }

    #[test]
    fn simplify_not_3() {
        let f = Formula::Predicate(1, vec![]);
        let correct_f = Formula::Not(Box::new(f.clone()));
        assert_eq!(simplify_not(f), correct_f);
    }

    #[test]
    fn simplify_and_1() {
        let f1 = Formula::Predicate(1, vec![]);
        let f2 = Formula::Predicate(2, vec![]);
        // Nondeterministic tests are a bitch.
        // There are some others in this file, but beyond that there shouldn't be any.
        let correct_f_1 = Formula::And(vec![f2.clone(), f1.clone()]);
        let correct_f_2 = Formula::And(vec![f1.clone(), f2.clone()]);
        let s1 = simplify_and(vec![f1.clone(), f2.clone(), Formula::True]);
        let s2 = simplify_and(vec![f1.clone(), Formula::True, f2.clone()]);
        let s3 = simplify_and(vec![Formula::True, f1, f2]);
        assert!(s1 == correct_f_1 || s1 == correct_f_2);
        assert!(s2 == correct_f_1 || s2 == correct_f_2);
        assert!(s3 == correct_f_1 || s3 == correct_f_2);
    }

    #[test]
    fn simplify_and_2() {
        let f1 = Formula::Predicate(1, vec![]);
        let f2 = Formula::Predicate(2, vec![]);
        assert_eq!(simplify_and(vec![f1.clone(), f2.clone(), Formula::False]),
                   Formula::False);
        assert_eq!(simplify_and(vec![f1.clone(), Formula::False, f2.clone()]),
                   Formula::False);
        assert_eq!(simplify_and(vec![Formula::False, f1, f2]), Formula::False);
    }

    #[test]
    fn simplify_and_3() {
        let f = Formula::Predicate(1, vec![]);
        let not_f = Formula::Not(Box::new(f.clone()));
        assert_eq!(simplify_and(vec![f.clone(), not_f.clone()]), Formula::False);
        assert_eq!(simplify_and(vec![not_f, f]), Formula::False);
    }

    #[test]
    fn simplify_and_4() {
        let f = Formula::Predicate(1, vec![]);
        assert_eq!(simplify_and(vec![f.clone(), f.clone(), f.clone()]), f);
    }

    #[test]
    fn simplify_and_5() {
        let f1 = Formula::Predicate(1, vec![]);
        let f2 = Formula::Predicate(2, vec![]);
        // Nondeterminism again.
        let correct_f_1 = Formula::And(vec![f2.clone(), f1.clone()]);
        let correct_f_2 = Formula::And(vec![f1.clone(), f2.clone()]);
        let s = simplify_and(vec![f1, f2]);
        assert!(s == correct_f_1 || s == correct_f_2);
    }

    #[test]
    fn simplify_or_1() {
        let f1 = Formula::Predicate(1, vec![]);
        let f2 = Formula::Predicate(2, vec![]);
        assert_eq!(simplify_or(vec![f1.clone(), f2.clone(), Formula::True]),
                   Formula::True);
        assert_eq!(simplify_or(vec![f1.clone(), Formula::True, f2.clone()]),
                   Formula::True);
        assert_eq!(simplify_or(vec![Formula::True, f1, f2]), Formula::True);
    }

    #[test]
    fn simplify_or_2() {
        let f1 = Formula::Predicate(1, vec![]);
        let f2 = Formula::Predicate(2, vec![]);
        // Nondeterminism yet again.
        let correct_f_1 = Formula::Or(vec![f1.clone(), f2.clone()]);
        let correct_f_2 = Formula::Or(vec![f2.clone(), f1.clone()]);
        let s1 = simplify_or(vec![f1.clone(), f2.clone(), Formula::False]);
        let s2 = simplify_or(vec![f1.clone(), Formula::False, f2.clone()]);
        let s3 = simplify_or(vec![Formula::False, f1, f2]);
        assert!(s1 == correct_f_1 || s1 == correct_f_2);
        assert!(s2 == correct_f_1 || s2 == correct_f_2);
        assert!(s3 == correct_f_1 || s3 == correct_f_2);
    }

    #[test]
    fn simplify_or_3() {
        let f = Formula::Predicate(1, vec![]);
        let not_f = Formula::Not(Box::new(f.clone()));
        assert_eq!(simplify_or(vec![f.clone(), not_f.clone()]), Formula::True);
        assert_eq!(simplify_or(vec![not_f, f]), Formula::True);
    }

    #[test]
    fn simplify_or_4() {
        let f = Formula::Predicate(1, vec![]);
        assert_eq!(simplify_or(vec![f.clone(), f.clone(), f.clone()]), f);
    }

    #[test]
    fn simplify_or_5() {
        let f1 = Formula::Predicate(1, vec![]);
        let f2 = Formula::Predicate(2, vec![]);
        // And yet again.
        let correct_f_1 = Formula::Or(vec![f1.clone(), f2.clone()]);
        let correct_f_2 = Formula::Or(vec![f2.clone(), f2.clone()]);
        let s = simplify_or(vec![f1, f2]);
        assert!(s == correct_f_1 || s == correct_f_2);
    }

    #[test]
    fn simplify_implies_1() {
        let f = Formula::Predicate(1, vec![]);
        assert_eq!(simplify_implies(f.clone(), f), Formula::True);
    }

    #[test]
    fn simplify_implies_2() {
        let f = Formula::Predicate(1, vec![]);
        assert_eq!(simplify_implies(f, Formula::True), Formula::True);
    }

    #[test]
    fn simplify_implies_3() {
        let f = Formula::Predicate(1, vec![]);
        assert_eq!(simplify_implies(Formula::False, f), Formula::True);
    }

    #[test]
    fn simplify_implies_4() {
        let f = Formula::Predicate(1, vec![]);
        let correct_f = Formula::Not(Box::new(f.clone()));
        assert_eq!(simplify_implies(f, Formula::False), correct_f);
    }

    #[test]
    fn simplify_implies_5() {
        let f = Formula::Predicate(1, vec![]);
        assert_eq!(simplify_implies(Formula::True, f.clone()), f);
    }

    #[test]
    fn simplify_implies_6() {
        let f1 = Formula::Predicate(1, vec![]);
        let f2 = Formula::Predicate(2, vec![]);
        let correct_f = Formula::Implies(Box::new(f1.clone()), Box::new(f2.clone()));
        assert_eq!(simplify_implies(f1, f2), correct_f);
    }

    #[test]
    fn simplify_equivalent_1() {
        let f = Formula::Predicate(1, vec![]);
        assert_eq!(simplify_equivalent(f.clone(), f), Formula::True);
    }

    #[test]
    fn simplify_equivalent_2() {
        let f = Formula::Predicate(1, vec![]);
        assert_eq!(simplify_equivalent(f.clone(), Formula::True), f);
        assert_eq!(simplify_equivalent(Formula::True, f.clone()), f);
    }

    #[test]
    fn simplify_equivalent_3() {
        let f = Formula::Predicate(1, vec![]);
        let correct_f = Formula::Not(Box::new(f.clone()));
        assert_eq!(simplify_equivalent(f.clone(), Formula::False), correct_f);
        assert_eq!(simplify_equivalent(Formula::False, f), correct_f);
    }

    #[test]
    fn simplify_equivalent_4() {
        let f1 = Formula::Predicate(1, vec![]);
        let f2 = Formula::Predicate(2, vec![]);
        let correct_f = Formula::Equivalent(Box::new(f1.clone()), Box::new(f2.clone()));
        assert_eq!(simplify_equivalent(f1, f2), correct_f);
    }

    #[test]
    fn simplify_quantifier_1() {
        let n = Term::Variable(-1);
        let odd = Formula::Predicate(1, vec![n.clone()]);
        let f = Formula::Exists(-1, Box::new(odd.clone()));
        assert_eq!(simplify_quantifier(-2, odd.clone(), false), odd);
        assert_eq!(simplify_quantifier(-1, odd, false), f);
    }

    #[test]
    fn simplify_quantifier_2() {
        let x = Term::Variable(-1);
        let f = Formula::Predicate(1, vec![x]);
        let correct_f = Formula::Forall(-1, Box::new(f.clone()));
        assert_eq!(simplify_quantifier(-1, f, true), correct_f);
    }
}
