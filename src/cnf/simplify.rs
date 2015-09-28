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
    use super::simplify_not;
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
}
