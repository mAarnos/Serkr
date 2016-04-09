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

use std::collections::hash_set::HashSet;
use cnf::ast::{Term, Formula};

/// Used for checking if a term t is free in a formula f.
pub fn free_in(f: &Formula, t: &Term) -> bool {
    match *f {
        Formula::True | Formula::False => false,
        Formula::Predicate(_, ref args) => args.iter().any(|x| occurs_in(x, t)),
        Formula::Not(ref p) => free_in(p, t),
        Formula::And(ref l) | Formula::Or(ref l) => l.iter().any(|x| free_in(x, t)),
        Formula::Implies(ref p, ref q) | Formula::Equivalent(ref p, ref q) => {
            free_in(p, t) || free_in(q, t)
        }
        Formula::Forall(id, ref p) | Formula::Exists(id, ref p) => {
            !occurs_in(&Term::Variable(id), t) && free_in(p, t)
        }
    }
}

/// Used for checking if a term s occurs in a term t.
fn occurs_in(t: &Term, s: &Term) -> bool {
    t == s ||
    match *t {
        Term::Variable(_) => false,
        Term::Function(_, ref args) => args.iter().any(|x| occurs_in(x, s)),
    }
}

/// Get the free variables of a formula.
pub fn free_variables(f: &Formula) -> HashSet<i64> {
    let mut vars = HashSet::new();
    fv(f, &mut vars);
    vars
}

/// Perform the actual getting of variables.
fn fv(f: &Formula, vars: &mut HashSet<i64>) {
    match *f {
        Formula::Predicate(_, ref params) => {
            for x in params {
                fvt(x, vars);
            }
        }
        Formula::Not(ref p) => fv(p, vars),
        Formula::And(ref l) | Formula::Or(ref l) => {
            for x in l {
                fv(x, vars);
            }
        }
        Formula::Implies(ref p, ref q) | Formula::Equivalent(ref p, ref q) => {
            fv(p, vars);
            fv(q, vars);
        }
        Formula::Forall(id, ref p) | Formula::Exists(id, ref p) => {
            fv(p, vars);
            vars.remove(&id);
        }
        _ => (),
    }
}

/// Helper function for above.
fn fvt(t: &Term, vars: &mut HashSet<i64>) {
    match *t {
        Term::Variable(id) => {
            assert!(id < 0);
            vars.insert(id);
        }
        Term::Function(_, ref params) => {
            for x in params {
                fvt(x, vars);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{free_in, free_variables};
    use cnf::ast::{Term, Formula};

    #[test]
    fn free_in_1() {
        let x = Term::Variable(-1);
        let y = Term::Variable(-2);
        let z = Term::Variable(-3);
        let a = Term::Variable(-4); // not used in f
        let p_x_y = Formula::Predicate(1, vec![x.clone(), y.clone()]);
        let p_y_z = Formula::Predicate(1, vec![y.clone(), z.clone()]);
        let p_x_z = Formula::Predicate(1, vec![x.clone(), z.clone()]);
        let f_q = Formula::Implies(Box::new(Formula::And(vec![p_x_y, p_y_z])), Box::new(p_x_z));
        let f =
            Formula::Forall(-1,
                            Box::new(Formula::Forall(-2,
                                                     Box::new(Formula::Forall(-3,
                                                                              Box::new(f_q))))));

        assert!(!free_in(&f, &x));
        assert!(!free_in(&f, &y));
        assert!(!free_in(&f, &z));
        assert!(!free_in(&f, &a));
    }

    #[test]
    fn free_in_2() {
        let x = Term::Variable(-1);
        let y = Term::Variable(-2);
        let f_y = Term::Function(1, vec![y.clone()]);
        let p_x_f_y = Formula::Predicate(1, vec![x.clone(), f_y]);
        let f = Formula::Forall(-1, Box::new(Formula::Exists(-3, Box::new(p_x_f_y))));

        assert!(!free_in(&f, &x));
        assert!(free_in(&f, &y));
    }

    #[test]
    fn free_variables_1() {
        let x = Term::Variable(-1);
        let y = Term::Variable(-2);
        let z = Term::Variable(-3);
        let p_x_y_z = Formula::Predicate(1, vec![x.clone(), y.clone(), z.clone()]);
        let f = Formula::Forall(-1, Box::new(Formula::Exists(-3, Box::new(p_x_y_z))));
        let vars = free_variables(&f);

        assert_eq!(vars.len(), 1);
        assert!(vars.contains(&-2));
    }
}
