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
use cnf::simplify::simplify_formula;

/// Converts a formula into an equivalent negation normal form.
pub fn nnf(f: Formula) -> Formula {
    let simplified_f = simplify_formula(f);
    if simplified_f == Formula::True || simplified_f == Formula::False {
        simplified_f
    } else {
        let nnf_formula = move_nots_inward(elim_imp_and_eq(simplified_f));
        assert!(is_in_nnf(&nnf_formula));
        nnf_formula
    }
}

/// Eliminates all implications and equivalences in a formula.
fn elim_imp_and_eq(f: Formula) -> Formula {
    match f {
        Formula::Not(box p) => Formula::Not(box elim_imp_and_eq(p)),
        Formula::And(box p, box q) => Formula::And(box elim_imp_and_eq(p), box elim_imp_and_eq(q)),
        Formula::Or(box p, box q) => Formula::Or(box elim_imp_and_eq(p), box elim_imp_and_eq(q)),
        Formula::Implies(box p, box q) => Formula::Or(box Formula::Not(box elim_imp_and_eq(p)), 
                                                      box elim_imp_and_eq(q)),
        Formula::Equivalent(box p, box q) => Formula::And(box Formula::Or(box elim_imp_and_eq(p.clone()), 
                                                                          box Formula::Not(box elim_imp_and_eq(q.clone()))), 
                                                          box Formula::Or(box Formula::Not(box elim_imp_and_eq(p)), 
                                                                          box elim_imp_and_eq(q))),
        Formula::Forall(s, box p) => Formula::Forall(s, box elim_imp_and_eq(p)),
        Formula::Exists(s, box p) => Formula::Exists(s, box elim_imp_and_eq(p)),
        _ => f,
    }
}

/// Moves all NOTs inward by repeatedly applying De Morgan's laws and double negation elimination.
fn move_nots_inward(f: Formula) -> Formula {
    match f {
        Formula::Not(box p) => move_nots_inward_not(p),
        Formula::And(box p, box q) => Formula::And(box move_nots_inward(p), box move_nots_inward(q)),
        Formula::Or(box p, box q) => Formula::Or(box move_nots_inward(p), box move_nots_inward(q)),
        Formula::Forall(s, box p) => Formula::Forall(s, box move_nots_inward(p)),
        Formula::Exists(s, box p) => Formula::Exists(s, box move_nots_inward(p)),
        _ => f
    }
}

/// "not not p" can be rewritten to "p".
/// "not (p and q)" can be rewritten to "not p or not q".
/// "not (p or q)" can be rewritten to "not p and not q".
/// "not forall x. p" can be rewritten to "exists x. not p".
/// "not exists x. p" can be rewritten to "forall x. not p".
fn move_nots_inward_not(f: Formula) -> Formula {
    match f {
        Formula::Not(box p) => move_nots_inward(p),
        Formula::And(box p, box q) => Formula::Or(box move_nots_inward(Formula::Not(box p)), 
                                                  box move_nots_inward(Formula::Not(box q))),
        Formula::Or(box p, box q) => Formula::And(box move_nots_inward(Formula::Not(box p)), 
                                                  box move_nots_inward(Formula::Not(box q))),
        Formula::Forall(s, box p) => Formula::Exists(s, box move_nots_inward(Formula::Not(box p))),
        Formula::Exists(s, box p) => Formula::Forall(s, box move_nots_inward(Formula::Not(box p))),
        _ => Formula::Not(box move_nots_inward(f)),
    }
}

/// Checks whether a given formula is in NNF or not.
fn is_in_nnf(f: &Formula) -> bool {
    match *f {
        Formula::Predicate(_, _) => true,
        Formula::Not(box Formula::Predicate(_, _)) => true,
        Formula::And(box ref p, box ref q) |
        Formula::Or(box ref p, box ref q) => is_in_nnf(p) && is_in_nnf(q),
        Formula::Forall(_, box ref p) | 
        Formula::Exists(_, box ref p) => is_in_nnf(p),
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use super::{nnf, elim_imp_and_eq, move_nots_inward, move_nots_inward_not};
    use parser::internal_parser::parse;
    
    #[test]
    fn nnf_1() {
        let f = parse("((P ==> Q) <=> (~Q ==> ~P))").unwrap();
        let correct_f = parse("(((~P \\/ Q) \\/ (~Q /\\ P)) /\\ ((P /\\ ~Q) \\/ (Q \\/ ~P)))").unwrap();
        assert_eq!(nnf(f), correct_f);                             
    }
    
    #[test]
    fn elim_imp_and_eq_1() {
        let f = parse("(P ==> Q)").unwrap();
        let correct_f = parse("(~P \\/ Q)").unwrap();
        assert_eq!(elim_imp_and_eq(f), correct_f);
    }
    
    #[test]
    fn elim_imp_and_eq_2() {
        let f = parse("(P <=> Q)").unwrap();
        let correct_f = parse("((P \\/ ~Q) /\\ (~P \\/ Q))").unwrap();
        assert_eq!(elim_imp_and_eq(f), correct_f);
    }
          
    #[test]
    fn elim_imp_and_eq_3() {
        let f = parse("(((P ==> Q) /\\ P) <=> Q)").unwrap();
        let correct_f = parse("((((~P \\/ Q) /\\ P) \\/ ~Q) /\\ (~((~P \\/ Q) /\\ P) \\/ Q))").unwrap();
        assert_eq!(elim_imp_and_eq(f), correct_f);
    }
        
    #[test]
    fn move_nots_inward_not_1() {
        let f = parse("~P").unwrap();
        let correct_f = parse("P").unwrap();
        assert_eq!(move_nots_inward_not(f), correct_f);
    }
    
    #[test]
    fn move_nots_inward_not_2() {
        let f = parse("(P /\\ Q)").unwrap();
        let correct_f = parse("(~P \\/ ~Q)").unwrap();
        assert_eq!(move_nots_inward_not(f), correct_f);
    }
    
    #[test]
    fn move_nots_inward_not_3() {
        let f = parse("(P \\/ Q)").unwrap();
        let correct_f = parse("(~P /\\ ~Q)").unwrap();
        assert_eq!(move_nots_inward_not(f), correct_f);
    }
    
    #[test]
    fn move_nots_inward_not_4() {
        let f = parse("exists x. ~P(x)").unwrap();
        let correct_f = parse("forall x. P(x)").unwrap();
        assert_eq!(move_nots_inward_not(f), correct_f);
    }
    
    #[test]
    fn move_nots_inward_not_5() {
        let f = parse("forall x. ~P(x)").unwrap();
        let correct_f = parse("exists x. P(x)").unwrap();
        assert_eq!(move_nots_inward_not(f), correct_f);
    }
    
    #[test]
    fn move_nots_inward_1() {
        let f = parse("~((P \\/ Q) /\\ ~~(~P /\\ Q))").unwrap();
        let correct_f = parse("((~P /\\ ~Q) \\/ (P \\/ ~Q))").unwrap();
        assert_eq!(move_nots_inward(f), correct_f);
    }
}