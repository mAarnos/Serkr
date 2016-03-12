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

use cnf::ast::{Term, Formula};
use cnf::free_variables::free_in;

/// Simplifies a formula by performing some equivalence-preserving first-order simplifications.
/// After this the formula is either True or False, or doesn't contain True and False.
pub fn simplify_formula(f: Formula) -> Formula {
    let simplified_formula = simplify(f);
    assert!(is_simplified(&simplified_formula));
    simplified_formula
}

fn negate(f: Formula) -> Formula {
    match f {
        Formula::Not(p) => *p,
        _ => Formula::Not(Box::new(f)),
    }
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

/// "p and p" can be simplified to "p".
/// "p and not p" can be simplified to "false".
/// "p and true" can be simplified to "p"
/// "p and false" can be simplified to "false".
fn simplify_and(l: Vec<Formula>) -> Formula {
    let mut simplified_l = l.into_iter()
                            .map(simplify)
                            .filter(|x| *x != Formula::True)
                            .collect::<Vec<_>>();

    if simplified_l.iter().any(|x| *x == Formula::False) {
        Formula::False
    } else {
        // Combines duplicate deletion and detection for 'p or not p'.
        let mut i = 0;
        while i < simplified_l.len() {
            let mut j = i + 1;
            let negated_f = negate(simplified_l[i].clone());

            while j < simplified_l.len() {
                if negated_f == simplified_l[j] {
                    return Formula::False;
                }

                if simplified_l[i] == simplified_l[j] {
                    simplified_l.swap_remove(j);
                    continue;
                }
                j += 1;
            }

            i += 1;
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
            Formula::And(simplified_l)
        }
    }
}

/// "p or p" can be simplified to "p".
/// "p or not p" can be simplified to "true".
/// "p or true" can be simplified to "true"
/// "p or false" can be simplified to "p".
fn simplify_or(l: Vec<Formula>) -> Formula {
    let mut simplified_l = l.into_iter()
                            .map(simplify)
                            .filter(|x| *x != Formula::False)
                            .collect::<Vec<_>>();

    if simplified_l.iter().any(|x| *x == Formula::True) {
        Formula::True
    } else {
        // Combines duplicate deletion and detection for 'p or not p'.
        let mut i = 0;
        while i < simplified_l.len() {
            let mut j = i + 1;
            let negated_f = negate(simplified_l[i].clone());

            while j < simplified_l.len() {
                if negated_f == simplified_l[j] {
                    return Formula::True;
                }

                if simplified_l[i] == simplified_l[j] {
                    simplified_l.swap_remove(j);
                    continue;
                }
                j += 1;
            }

            i += 1;
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
            Formula::Or(simplified_l)
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
mod test {}
