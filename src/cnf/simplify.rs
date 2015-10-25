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
use cnf::free_variables::free_in;

/// Simplifies a formula by performing some equivalence-preserving first-order simplifications.
/// After this function the formula is either only "true" or "false", or it no longer contains any instances of "true" and "false".
pub fn simplify_formula(f: Formula) -> Formula {
    let simplified_formula = simplify(f);
    assert!(is_simplified(&simplified_formula));
    simplified_formula
}

/// The main function for simplifying the formula from bottom up.
fn simplify(f: Formula) -> Formula {
    match f { 
        Formula::Not(box p) => simplify_not(p),
        Formula::And(box p, box q) => simplify_and(p, q),
        Formula::Or(box p, box q) => simplify_or(p, q),
        Formula::Implies(box p, box q) => simplify_implies(p, q),
        Formula::Equivalent(box p, box q) => simplify_equivalent(p, q),
        Formula::Forall(s, box p) => simplify_quantifier(s, p, true),
        Formula::Exists(s, box p) => simplify_quantifier(s, p, false),
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
        _ => Formula::Not(box simplified_f),
    }
}

/// "p and p" can be simplified to "p".
/// "p and not p" can be simplified to "false".
/// "p and true" can be simplified to "p"
/// "p and false" can be simplified to "false".
fn simplify_and(f1: Formula, f2: Formula) -> Formula {
    let simplified_f = (simplify(f1), simplify(f2));
    match simplified_f {
        (p, Formula::True) | (Formula::True, p) => p,
        (_, Formula::False) | (Formula::False, _) => Formula::False,
        (p, Formula::Not(box q)) | (Formula::Not(box q), p) => if p == q { Formula::False } else { Formula::And(box p, box Formula::Not(box q)) },
        (p, q) => if p == q { p } else { Formula::And(box p, box q) },
    }
}

/// "p or p" can be simplified to "p".
/// "p or not p" can be simplified to "true".
/// "p or true" can be simplified to "true"
/// "p or false" can be simplified to "p".
fn simplify_or(f1: Formula, f2: Formula) -> Formula {
    let simplified_f = (simplify(f1), simplify(f2));
    match simplified_f {
        (_, Formula::True) | (Formula::True, _) => Formula::True,
        (p, Formula::False) | (Formula::False, p) => p,
        (p, Formula::Not(box q)) | (Formula::Not(box q), p) => if p == q { Formula::True } else { Formula::Or(box p, box Formula::Not(box q)) },
        (p, q) => if p == q { p } else { Formula::Or(box p, box q) },
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
        (_, Formula::True) => Formula::True,
        (Formula::True, p) => p,
        (p, Formula::False) => simplify(Formula::Not(box p)),
        (Formula::False, _) => Formula::True,
        (p, q) => if p == q { Formula::True } else { Formula::Implies(box p, box q) },
    }
}

/// "p equivalent true" can be rewritten as "p".
/// "p equivalent false" can be rewritten as "not p".
/// "p equivalent p" can be rewritten as "true".
fn simplify_equivalent(f1: Formula, f2: Formula) -> Formula {
    let simplified_f = (simplify(f1), simplify(f2));
    match simplified_f {
        (p, Formula::True) | (Formula::True, p) => p,
        (p, Formula::False) | (Formula::False, p) => simplify(Formula::Not(box p)),
        (p, q) => if p == q { Formula::True } else { Formula::Equivalent(box p, box q) },
    }
}

/// "forall x. p" and "exists x. p" can be written as "p" if x does not occur free in p.
fn simplify_quantifier(s: String, f: Formula, forall: bool) -> Formula {
    let simplified_f = simplify(f);
    if free_in(&simplified_f, &Term::Variable(s.clone())) {
        if forall {
            Formula::Forall(s, box simplified_f)
        } else {
            Formula::Exists(s, box simplified_f)
        }   
    } else {
        simplified_f
    }
}

/// Checks if a formula has been simplified, i.e. it is either "true" or "false" or doesn't contain "true" or "false".
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
        Formula::True => true,
        Formula::False => true,
        Formula::Predicate(_, _) => false,
        Formula::Not(box ref p) => contains_true_or_false(p),
        Formula::And(box ref p, box ref q) | 
        Formula::Or(box ref p, box ref q) |
        Formula::Implies(box ref p, box ref q) | 
        Formula::Equivalent(box ref p, box ref q) => contains_true_or_false(p) || contains_true_or_false(q), 
        Formula::Forall(_, box ref p) | 
        Formula::Exists(_, box ref p) => contains_true_or_false(p),
    }
}

#[cfg(test)]
mod test {
    use super::{simplify_formula, simplify_quantifier, simplify_not, simplify_and, simplify_or, simplify_implies, simplify_equivalent};
     use utils::formula::Formula;
    use parser::internal_parser::parse;
    
    #[test]
    fn simplify_1() {
        let f = parse("((P <=> P) \\/ Q(x, y))").unwrap();
        assert_eq!(simplify_formula(f), Formula::True);
    }
    
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
        let f = parse("P(x, f(y), g(f(z)))").unwrap();
        let correct_f = parse("~P(x, f(y), g(f(z)))").unwrap();
        assert_eq!(simplify_not(f), correct_f);
    }
    
    #[test]
    fn simplify_and_1() {
        let f = parse("P(x)").unwrap();
        assert_eq!(simplify_and(Formula::True, f.clone()), f);
        assert_eq!(simplify_and(f.clone(), Formula::True), f);
    }
    
    #[test]
    fn simplify_and_2() {
        let f = parse("P(x)").unwrap();
        assert_eq!(simplify_and(Formula::False, f.clone()), Formula::False);
        assert_eq!(simplify_and(f, Formula::False), Formula::False);
    }
    
    #[test]
    fn simplify_and_3() {
        let f1 = parse("P(x, y)").unwrap();
        let f2 = parse("~P(x, y)").unwrap();
        assert_eq!(simplify_and(f1.clone(), f2.clone()), Formula::False);
        assert_eq!(simplify_and(f2, f1), Formula::False);
    }
     
    #[test]
    fn simplify_and_4() {
        let f1 = parse("P(x)").unwrap();
        let f2 = parse("~Q(x)").unwrap();
        let correct_f = parse("(P(x) /\\ ~Q(x))").unwrap();
        assert_eq!(simplify_and(f1.clone(), f2.clone()), correct_f);
        assert_eq!(simplify_and(f2, f1), correct_f);
    }
        
    #[test]
    fn simplify_and_5() {
        let f = parse("P(x, y)").unwrap();
        assert_eq!(simplify_and(f.clone(), f.clone()), f);
    }
    
    #[test]
    fn simplify_and_6() {
        let f1 = parse("P(x)").unwrap();
        let f2 = parse("Q(x)").unwrap();
        let correct_f = parse("(P(x) /\\ Q(x))").unwrap();
        assert_eq!(simplify_and(f1, f2), correct_f);
    }
     
    #[test]
    fn simplify_or_1() {
        let f = parse("P(x)").unwrap();
        assert_eq!(simplify_or(Formula::True, f.clone()), Formula::True);
        assert_eq!(simplify_or(f, Formula::True), Formula::True);
    }
    
    #[test]
    fn simplify_or_2() {
        let f = parse("P(x)").unwrap();
        assert_eq!(simplify_or(Formula::False, f.clone()), f.clone());
        assert_eq!(simplify_or(f.clone(), Formula::False), f);
    }
    
    #[test]
    fn simplify_or_3() {
        let f1 = parse("P(x)").unwrap();
        let f2 = parse("~P(x)").unwrap();
        assert_eq!(simplify_or(f1.clone(), f2.clone()), Formula::True);
        assert_eq!(simplify_or(f2, f1), Formula::True);
    }
    
    #[test]
    fn simplify_or_4() {
        let f1 = parse("P").unwrap();
        let f2 = parse("~Q").unwrap();
        let correct_f = parse("(P \\/ ~Q)").unwrap();
        assert_eq!(simplify_or(f1.clone(), f2.clone()), correct_f);
        assert_eq!(simplify_or(f2, f1), correct_f);
    }
    
    #[test]
    fn simplify_or_5() {
        let f = parse("P(x)").unwrap();
        assert_eq!(simplify_or(f.clone(), f.clone()), f);
    }
    
    #[test]
    fn simplify_or_6() {
        let f1 = parse("P").unwrap();
        let f2 = parse("Q").unwrap();
        let correct_f = parse("(P \\/ Q)").unwrap();
        assert_eq!(simplify_or(f1, f2), correct_f);
    }
    
    #[test]
    fn simplify_implies_1() {
        let f = parse("P(x, y, z, w)").unwrap();
        assert_eq!(simplify_implies(f.clone(), f), Formula::True);
    }
    
    #[test]
    fn simplify_implies_2() {
        let f = parse("P(x)").unwrap();
        assert_eq!(simplify_implies(f, Formula::True), Formula::True);
    }
    
    #[test]
    fn simplify_implies_3() {
        let f = parse("P(c)").unwrap();
        let correct_f = parse("~P(c)").unwrap();
        assert_eq!(simplify_implies(f.clone(), Formula::False), correct_f);
    }
    
    #[test]
    fn simplify_implies_4() {
        let f = parse("P(c, f(x))").unwrap();
        assert_eq!(simplify_implies(Formula::True, f.clone()), f);
    }
    
    #[test]
    fn simplify_implies_5() {
        let f = parse("P").unwrap();
        assert_eq!(simplify_implies(Formula::False, f), Formula::True);
    }
    
    #[test]
    fn simplify_implies_6() {
        let f1 = parse("P").unwrap();
        let f2 = parse("Q").unwrap();
        let correct_f = parse("(P ==> Q)").unwrap();
        assert_eq!(simplify_implies(f1.clone(), f2.clone()), correct_f);
    }
    
    #[test]
    fn simplify_equivalent_1() {
        let f = parse("P(x, y)").unwrap();
        assert_eq!(simplify_equivalent(f.clone(), f), Formula::True);
    }
    
    #[test]
    fn simplify_equivalent_2() {
        let f = parse("P(x)").unwrap();
        assert_eq!(simplify_equivalent(f.clone(), Formula::True), f);
        assert_eq!(simplify_equivalent(Formula::True, f.clone()), f);
    }
    
    #[test]
    fn simplify_equivalent_3() {
        let f = parse("P(x)").unwrap();
        let correct_f = parse("~P(x)").unwrap();
        assert_eq!(simplify_equivalent(f.clone(), Formula::False), correct_f);
        assert_eq!(simplify_equivalent(Formula::False, f), correct_f);
    }
    
    #[test]
    fn simplify_equivalent_4() {
        let f1 = parse("P(x)").unwrap();
        let f2 = parse("Q(x, y)").unwrap();
        let correct_f = parse("(P(x) <=> Q(x, y))").unwrap();
        assert_eq!(simplify_equivalent(f1, f2), correct_f);
    }
    
    #[test]
    fn simplify_quantifier_1() {
        let f = parse("exists y. (Odd(n) \\/ Even(n))").unwrap();
        let correct_f = parse("(Odd(n) \\/ Even(n))").unwrap();
        assert_eq!(simplify_quantifier("x".to_string(), f, true), correct_f);
    }
    
    #[test]
    fn simplify_quantifier_2() {
        let f = parse("exists y. (Odd(n) \\/ Even(n))").unwrap();
        let correct_f = parse("forall n. (Odd(n) \\/ Even(n))").unwrap();
        assert_eq!(simplify_quantifier("n".to_string(), f, true), correct_f);
    }
    
    #[test]
    fn simplify_quantifier_3() {
        let f = parse("P(x)").unwrap();
        let correct_f = parse("exists x. P(x)").unwrap();
        assert_eq!(simplify_quantifier("x".to_string(), f, false), correct_f);
    }
}
