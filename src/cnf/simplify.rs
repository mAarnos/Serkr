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

use parser::formula::{Formula};

/// Simplifies a formula by performing some equivalence-preserving first-order simplifications.
/// After this function the formula either is only "true" or "false", or it no longer contains any instances of "true" and "false".
pub fn simplify(f: Formula) -> Formula {
    match f { 
        Formula::Not(box p) => simplify_not(p),
        Formula::And(box p, box q) => simplify_and(p, q),
        Formula::Or(box p, box q) => simplify_or(p, q),
        Formula::Implies(box p, box q) => simplify_implies(p, q),
        Formula::Equivalent(box p, box q) => simplify_equivalent(p, q),
        Formula::Forall(s, box p) => Formula::Forall(s, box simplify(p)),
        Formula::Exists(s, box p) => Formula::Exists(s, box simplify(p)),
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

#[cfg(test)]
mod test {
    use super::{simplify_not, simplify_and, simplify_or, simplify_implies, simplify_equivalent};
    use parser::formula::{Formula};
    
    #[test]
    fn simplify_not_1() {
        assert_eq!(simplify_not(Formula::False), Formula::True);
    }
    
    #[test]
    fn simplify_not_2() {
        assert_eq!(simplify_not(Formula::True), Formula::False);
    }
    
    #[test]
    fn simplify_not_3() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        assert_eq!(simplify_not(pred.clone()), Formula::Not(box pred));
    }
    
    #[test]
    fn simplify_and_1() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        assert_eq!(simplify_and(pred.clone(), pred.clone()), pred);
    }
    
    #[test]
    fn simplify_and_2() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        assert_eq!(simplify_and(pred.clone(), Formula::Not(box pred.clone())), Formula::False);
        assert_eq!(simplify_and(Formula::Not(box pred.clone()), pred), Formula::False);
    }
    
    #[test]
    fn simplify_and_3() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        assert_eq!(simplify_and(Formula::True, pred.clone()), pred);
        assert_eq!(simplify_and(pred.clone(), Formula::True), pred);
    }
    
    #[test]
    fn simplify_and_4() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        assert_eq!(simplify_and(Formula::False, pred.clone()), Formula::False);
        assert_eq!(simplify_and(pred, Formula::False), Formula::False);
    }
    
    #[test]
    fn simplify_and_5() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        let pred2 = Formula::Predicate("Q".to_string(), Vec::new());
        assert_eq!(simplify_and(pred.clone(), pred2.clone()), Formula::And(box pred, box pred2));
    }
    
    #[test]
    fn simplify_and_6() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        let pred2 = Formula::Predicate("Q".to_string(), Vec::new());
        assert_eq!(simplify_and(pred.clone(), Formula::Not(box pred2.clone())), Formula::And(box pred.clone(), 
                                                                                             box Formula::Not(box pred2.clone())));
        assert_eq!(simplify_and(Formula::Not(box pred2.clone()), pred.clone()), Formula::And(box pred, 
                                                                                             box Formula::Not(box pred2)));
    }
    
    #[test]
    fn simplify_or_1() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        assert_eq!(simplify_or(pred.clone(), pred.clone()), pred);
    }
    
    #[test]
    fn simplify_or_2() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        assert_eq!(simplify_or(pred.clone(), Formula::Not(box pred.clone())), Formula::True);
        assert_eq!(simplify_or(Formula::Not(box pred.clone()), pred), Formula::True);
    }
    
    #[test]
    fn simplify_or_3() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        assert_eq!(simplify_or(Formula::True, pred.clone()), Formula::True);
        assert_eq!(simplify_or(pred, Formula::True), Formula::True);
    }
    
    #[test]
    fn simplify_or_4() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        assert_eq!(simplify_or(Formula::False, pred.clone()), pred.clone());
        assert_eq!(simplify_or(pred.clone(), Formula::False), pred);
    }
    
    #[test]
    fn simplify_or_5() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        let pred2 = Formula::Predicate("Q".to_string(), Vec::new());
        assert_eq!(simplify_or(pred.clone(), pred2.clone()), Formula::Or(box pred, box pred2));
    }
    
    #[test]
    fn simplify_or_6() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        let pred2 = Formula::Predicate("Q".to_string(), Vec::new());
        assert_eq!(simplify_or(pred.clone(), Formula::Not(box pred2.clone())), Formula::Or(box pred.clone(), 
                                                                                           box Formula::Not(box pred2.clone())));
        assert_eq!(simplify_or(Formula::Not(box pred2.clone()), pred.clone()), Formula::Or(box pred, 
                                                                                           box Formula::Not(box pred2)));
    }
    
    #[test]
    fn simplify_implies_1() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        assert_eq!(simplify_implies(pred.clone(), pred.clone()), Formula::True);
    }
    
    #[test]
    fn simplify_implies_2() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        assert_eq!(simplify_implies(pred, Formula::True), Formula::True);
    }
    
    #[test]
    fn simplify_implies_3() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        assert_eq!(simplify_implies(pred.clone(), Formula::False), Formula::Not(box pred));
    }
    
    #[test]
    fn simplify_implies_4() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        assert_eq!(simplify_implies(Formula::True, pred.clone()), pred);
    }
    
    #[test]
    fn simplify_implies_5() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        assert_eq!(simplify_implies(Formula::False, pred), Formula::True);
    }
    
    #[test]
    fn simplify_implies_6() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        let pred2 = Formula::Predicate("Q".to_string(), Vec::new());
        assert_eq!(simplify_implies(pred.clone(), pred2.clone()), Formula::Implies(box pred, box pred2));
    }
    
    #[test]
    fn simplify_equivalent_1() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        assert_eq!(simplify_equivalent(pred.clone(), pred), Formula::True);
    }
    
    #[test]
    fn simplify_equivalent_2() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        assert_eq!(simplify_equivalent(pred.clone(), Formula::True), pred);
        assert_eq!(simplify_equivalent(Formula::True, pred.clone()), pred);
    }
    
    #[test]
    fn simplify_equivalent_3() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        assert_eq!(simplify_equivalent(pred.clone(), Formula::False), Formula::Not(box pred.clone()));
        assert_eq!(simplify_equivalent(Formula::False, pred.clone()), Formula::Not(box pred));
    }
    
    #[test]
    fn simplify_equivalent_4() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        let pred2 = Formula::Predicate("Q".to_string(), Vec::new());
        assert_eq!(simplify_equivalent(pred.clone(), pred2.clone()), Formula::Equivalent(box pred, box pred2));
    }
}
