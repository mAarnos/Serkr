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

use cnf::ast::Formula;
use cnf::simplify::simplify_formula;
use cnf::formula_renaming::rename_formula;
use cnf::renaming_info::RenamingInfo;

/// Converts a formula into an equivalent negation normal form.
pub fn nnf(f: Formula, renaming_info: &mut RenamingInfo, renaming_limit: u64) -> Formula {
    let simplified_f = simplify_formula(f);
    if simplified_f == Formula::True || simplified_f == Formula::False {
        simplified_f
    } else {
        let renamed_f = rename_formula(simplified_f, renaming_info, renaming_limit);
        let nnf_formula = move_nots_inward(elim_imp_and_eq(renamed_f, true));
        assert!(is_in_nnf(&nnf_formula));
        nnf_formula
    }
}

/// Eliminates all implications and equivalences in a formula.
fn elim_imp_and_eq(f: Formula, polarity: bool) -> Formula {
    match f {
        Formula::Not(p) => Formula::Not(Box::new(elim_imp_and_eq(*p, !polarity))),
        Formula::And(l) => {
            Formula::And(l.into_iter().map(|x| elim_imp_and_eq(x, polarity)).collect())
        }
        Formula::Or(l) => {
            Formula::Or(l.into_iter().map(|x| elim_imp_and_eq(x, polarity)).collect())
        }
        Formula::Implies(p, q) => {
            Formula::Or(vec![Formula::Not(Box::new(elim_imp_and_eq(*p, !polarity))),
                             elim_imp_and_eq(*q, polarity)])
        }
        Formula::Equivalent(p, q) => eliminate_equivalence(*p, *q, polarity),
        Formula::Forall(s, p) => Formula::Forall(s, Box::new(elim_imp_and_eq(*p, polarity))),
        Formula::Exists(s, p) => Formula::Exists(s, Box::new(elim_imp_and_eq(*p, polarity))),
        _ => f,
    }
}

/// Eliminates an equivalence.
/// Returns different results depending on polarity to avoid unnecessary clauses.
fn eliminate_equivalence(p: Formula, q: Formula, polarity: bool) -> Formula {
    if polarity {
        Formula::And(vec![Formula::Or(vec![elim_imp_and_eq(p.clone(), polarity),
                                           Formula::Not(Box::new(elim_imp_and_eq(q.clone(),
                                                                                 !polarity)))]),
                          Formula::Or(vec![Formula::Not(Box::new(elim_imp_and_eq(p, !polarity))),
                                           elim_imp_and_eq(q, polarity)])])
    } else {
        Formula::Or(vec![Formula::And(vec![elim_imp_and_eq(p.clone(), polarity),
                                           elim_imp_and_eq(q.clone(), polarity)]),
                         Formula::And(vec![Formula::Not(Box::new(elim_imp_and_eq(p, !polarity))),
                                           Formula::Not(Box::new(elim_imp_and_eq(q, !polarity)))])])
    }
}

/// Moves all NOTs inward by repeatedly applying De Morgan's laws and double negation elimination.
fn move_nots_inward(f: Formula) -> Formula {
    match f {
        Formula::Not(p) => move_nots_inward_not(*p),
        Formula::And(l) => Formula::And(l.into_iter().map(move_nots_inward).collect()),
        Formula::Or(l) => Formula::Or(l.into_iter().map(move_nots_inward).collect()),
        Formula::Forall(s, p) => Formula::Forall(s, Box::new(move_nots_inward(*p))),
        Formula::Exists(s, p) => Formula::Exists(s, Box::new(move_nots_inward(*p))),
        _ => f,
    }
}

/// "not not p" can be rewritten to "p".
/// "not (p and q)" can be rewritten to "not p or not q".
/// "not (p or q)" can be rewritten to "not p and not q".
/// "not forall x. p" can be rewritten to "exists x. not p".
/// "not exists x. p" can be rewritten to "forall x. not p".
fn move_nots_inward_not(f: Formula) -> Formula {
    match f {
        Formula::Not(p) => move_nots_inward(*p),
        Formula::And(l) => {
            Formula::Or(l.into_iter()
                         .map(|x| move_nots_inward(Formula::Not(Box::new(x))))
                         .collect())
        }
        Formula::Or(l) => {
            Formula::And(l.into_iter()
                          .map(|x| move_nots_inward(Formula::Not(Box::new(x))))
                          .collect())
        }
        Formula::Forall(s, p) => Formula::Exists(s, Box::new(move_nots_inward(Formula::Not(p)))),
        Formula::Exists(s, p) => Formula::Forall(s, Box::new(move_nots_inward(Formula::Not(p)))),
        _ => Formula::Not(Box::new(move_nots_inward(f))),
    }
}

/// Checks whether a given formula is in NNF or not.
fn is_in_nnf(f: &Formula) -> bool {
    match *f {
        Formula::Predicate(_, _) => true,
        Formula::Not(ref p) => {
            match **p {
                Formula::Predicate(_, _) => true,
                _ => false,
            }
        }
        Formula::And(ref l) |
        Formula::Or(ref l) => l.iter().all(is_in_nnf),
        Formula::Forall(_, ref p) |
        Formula::Exists(_, ref p) => is_in_nnf(p),
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use super::{elim_imp_and_eq, move_nots_inward_not};
    use cnf::ast::Formula;

    #[test]
    fn elim_imp_and_eq_1() {
        let p = Formula::Predicate(1, vec![]);
        let q = Formula::Predicate(2, vec![]);
        let f = Formula::Implies(Box::new(p.clone()), Box::new(q.clone()));
        let correct_f = Formula::Or(vec![Formula::Not(Box::new(p)), q]);
        assert_eq!(elim_imp_and_eq(f.clone(), false), correct_f);
        assert_eq!(elim_imp_and_eq(f, true), correct_f);
    }

    #[test]
    fn elim_imp_and_eq_2() {
        let p = Formula::Predicate(1, vec![]);
        let q = Formula::Predicate(2, vec![]);
        let f = Formula::Equivalent(Box::new(p.clone()), Box::new(q.clone()));
        let correct_f = Formula::And(vec![Formula::Or(vec![p.clone(),
                                                           Formula::Not(Box::new(q.clone()))]),
                                          Formula::Or(vec![Formula::Not(Box::new(p)), q])]);
        assert_eq!(elim_imp_and_eq(f, true), correct_f);
    }

    #[test]
    fn elim_imp_and_eq_3() {
        let p = Formula::Predicate(1, vec![]);
        let q = Formula::Predicate(2, vec![]);
        let f = Formula::Equivalent(Box::new(p.clone()), Box::new(q.clone()));
        let correct_f = Formula::Or(vec![Formula::And(vec![p.clone(), q.clone()]),
                                         Formula::And(vec![Formula::Not(Box::new(p)),
                                                           Formula::Not(Box::new(q))])]);
        assert_eq!(elim_imp_and_eq(f, false), correct_f);
    }

    #[test]
    fn move_nots_inward_not_1() {
        let p = Formula::Predicate(1, vec![]);
        let f = Formula::Not(Box::new(p.clone()));
        assert_eq!(move_nots_inward_not(f), p);
    }

    #[test]
    fn move_nots_inward_not_2() {
        let p = Formula::Predicate(1, vec![]);
        let q = Formula::Predicate(2, vec![]);
        let f = Formula::And(vec![p.clone(), q.clone()]);
        let correct_f = Formula::Or(vec![Formula::Not(Box::new(p)), Formula::Not(Box::new(q))]);
        assert_eq!(move_nots_inward_not(f), correct_f);
    }

    #[test]
    fn move_nots_inward_not_3() {
        let p = Formula::Predicate(1, vec![]);
        let q = Formula::Predicate(2, vec![]);
        let f = Formula::Or(vec![Formula::Not(Box::new(p.clone())),
                                 Formula::Not(Box::new(q.clone()))]);
        let correct_f = Formula::And(vec![p, q]);
        assert_eq!(move_nots_inward_not(f), correct_f);
    }

    #[test]
    fn move_nots_inward_not_4() {
        let p = Formula::Predicate(1, vec![]);
        let f = Formula::Exists(-1, Box::new(Formula::Not(Box::new(p.clone()))));
        let correct_f = Formula::Forall(-1, Box::new(p));
        assert_eq!(move_nots_inward_not(f), correct_f);
    }

    #[test]
    fn move_nots_inward_not_5() {
        let p = Formula::Predicate(1, vec![]);
        let f = Formula::Forall(-1, Box::new(p.clone()));
        let correct_f = Formula::Exists(-1, Box::new(Formula::Not(Box::new(p))));
        assert_eq!(move_nots_inward_not(f), correct_f);
    }
}
