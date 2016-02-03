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

use cnf::ast::Formula;
use cnf::simplify::simplify_formula;

/// Converts a formula into an equivalent negation normal form.
pub fn nnf(f: Formula) -> Formula {
    let simplified_f = simplify_formula(f);
    if simplified_f == Formula::True || simplified_f == Formula::False {
        simplified_f
    } else {
        let nnf_formula = move_nots_inward(elim_imp_and_eq(simplified_f, true));
        assert!(is_in_nnf(&nnf_formula));
        nnf_formula
    }
}

/// Eliminates all implications and equivalences in a formula.
fn elim_imp_and_eq(f: Formula, polarity: bool) -> Formula {
    match f {
        Formula::Not(p) => Formula::Not(Box::new(elim_imp_and_eq(*p, !polarity))),
        Formula::And(p, q) => Formula::And(Box::new(elim_imp_and_eq(*p, polarity)), Box::new(elim_imp_and_eq(*q, polarity))),
        Formula::Or(p, q) => Formula::Or(Box::new(elim_imp_and_eq(*p, polarity)), Box::new(elim_imp_and_eq(*q, polarity))),
        Formula::Implies(p, q) => Formula::Or(Box::new(Formula::Not(Box::new(elim_imp_and_eq(*p, !polarity)))), 
                                              Box::new(elim_imp_and_eq(*q, polarity))),
        Formula::Equivalent(p, q) => eliminate_equivalence(*p, *q, polarity),
        Formula::Forall(s, p) => Formula::Forall(s, Box::new(elim_imp_and_eq(*p, polarity))),
        Formula::Exists(s, p) => Formula::Exists(s, Box::new(elim_imp_and_eq(*p, polarity))),
        _ => f,
    }
}

/// Eliminates an equivalence. Returns different results depending on polarity to avoid unnecessary clauses.
fn eliminate_equivalence(p: Formula, q: Formula, polarity: bool) -> Formula {
    if polarity {
        Formula::And(Box::new(Formula::Or(Box::new(elim_imp_and_eq(p.clone(), polarity)), 
                                          Box::new(Formula::Not(Box::new(elim_imp_and_eq(q.clone(), !polarity)))))), 
                     Box::new(Formula::Or(Box::new(Formula::Not(Box::new(elim_imp_and_eq(p, !polarity)))), 
                                          Box::new(elim_imp_and_eq(q, polarity)))))
    } else {
        Formula::Or(Box::new(Formula::And(Box::new(elim_imp_and_eq(p.clone(), polarity)), 
                                          Box::new(elim_imp_and_eq(q.clone(), polarity)))), 
                    Box::new(Formula::And(Box::new(Formula::Not(Box::new(elim_imp_and_eq(p, !polarity)))), 
                                          Box::new(Formula::Not(Box::new(elim_imp_and_eq(q, !polarity)))))))
    }
}

/// Moves all NOTs inward by repeatedly applying De Morgan's laws and double negation elimination.
fn move_nots_inward(f: Formula) -> Formula {
    match f {
        Formula::Not(p) => move_nots_inward_not(*p),
        Formula::And(p, q) => Formula::And(Box::new(move_nots_inward(*p)), Box::new(move_nots_inward(*q))),
        Formula::Or(p, q) => Formula::Or(Box::new(move_nots_inward(*p)), Box::new(move_nots_inward(*q))),
        Formula::Forall(s, p) => Formula::Forall(s, Box::new(move_nots_inward(*p))),
        Formula::Exists(s, p) => Formula::Exists(s, Box::new(move_nots_inward(*p))),
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
        Formula::Not(p) => move_nots_inward(*p),
        Formula::And(p, q) => Formula::Or(Box::new(move_nots_inward(Formula::Not(p))), 
                                          Box::new(move_nots_inward(Formula::Not(q)))),
        Formula::Or(p, q) => Formula::And(Box::new(move_nots_inward(Formula::Not(p))), 
                                          Box::new(move_nots_inward(Formula::Not(q)))),
        Formula::Forall(s, p) => Formula::Exists(s, Box::new(move_nots_inward(Formula::Not(p)))),
        Formula::Exists(s, p) => Formula::Forall(s, Box::new(move_nots_inward(Formula::Not(p)))),
        _ => Formula::Not(Box::new(move_nots_inward(f))),
    }
}

/// Checks whether a given formula is in NNF or not.
fn is_in_nnf(f: &Formula) -> bool {
    match *f {
        Formula::Predicate(_, _) => true,
        Formula::Not(ref p) => match **p {
                                   Formula::Predicate(_, _) => true,
                                   _ => false,
                               },
        Formula::And(ref p, ref q) |
        Formula::Or(ref p, ref q) => is_in_nnf(p) && is_in_nnf(q),
        Formula::Forall(_, ref p) | 
        Formula::Exists(_, ref p) => is_in_nnf(p),
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use super::{nnf, elim_imp_and_eq, move_nots_inward, move_nots_inward_not};
    use cnf::ast_transformer_internal::internal_to_cnf_ast;
    
    #[test]
    fn nnf_1() {
        let (f, _) = internal_to_cnf_ast("(P ==> Q) <=> (~Q ==> ~P)").unwrap();
        let (correct_f, _) = internal_to_cnf_ast("(~P \\/ Q \\/ (~Q /\\ P)) /\\ ((P /\\ ~Q) \\/ (Q \\/ ~P))").unwrap();
        assert_eq!(nnf(f), correct_f);                             
    }
          
    #[test]
    fn nnf_2() {
        let (f, _) = internal_to_cnf_ast("~(P <=> Q)").unwrap();
        let (correct_f, _) = internal_to_cnf_ast("(~P \\/ ~Q) /\\ (P \\/ Q)").unwrap();
        assert_eq!(nnf(f), correct_f);
    }
    
    #[test]
    fn elim_imp_and_eq_1() {
        let (f, _) = internal_to_cnf_ast("P ==> Q").unwrap();
        let (correct_f, _) = internal_to_cnf_ast("~P \\/ Q").unwrap();
        assert_eq!(elim_imp_and_eq(f, true), correct_f);
    }
    
    #[test]
    fn elim_imp_and_eq_2() {
        let (f, _) = internal_to_cnf_ast("P <=> Q").unwrap();
        let (correct_f, _) = internal_to_cnf_ast("(P \\/ ~Q) /\\ (~P \\/ Q)").unwrap();
        assert_eq!(elim_imp_and_eq(f, true), correct_f);
    }
    
    #[test]
    fn elim_imp_and_eq_3() {
        let (f, _) = internal_to_cnf_ast("P <=> Q").unwrap();
        let (correct_f, _) = internal_to_cnf_ast("(P /\\ Q) \\/ (~P /\\ ~Q)").unwrap();
        assert_eq!(elim_imp_and_eq(f, false), correct_f);
    }
          
    #[test]
    fn elim_imp_and_eq_4() {
        let (f, _) = internal_to_cnf_ast("((P ==> Q) /\\ P) <=> Q").unwrap();
        let (correct_f, _) = internal_to_cnf_ast("(((~P \\/ Q) /\\ P) \\/ ~Q) /\\ (~((~P \\/ Q) /\\ P) \\/ Q)").unwrap();
        assert_eq!(elim_imp_and_eq(f, true), correct_f);
    }
        
    #[test]
    fn move_nots_inward_not_1() {
        let (f, _) = internal_to_cnf_ast("~P").unwrap();
        let (correct_f, _) = internal_to_cnf_ast("P").unwrap();
        assert_eq!(move_nots_inward_not(f), correct_f);
    }
    
    #[test]
    fn move_nots_inward_not_2() {
        let (f, _) = internal_to_cnf_ast("P /\\ Q").unwrap();
        let (correct_f, _) = internal_to_cnf_ast("~P \\/ ~Q").unwrap();
        assert_eq!(move_nots_inward_not(f), correct_f);
    }
    
    #[test]
    fn move_nots_inward_not_3() {
        let (f, _) = internal_to_cnf_ast("P \\/ Q").unwrap();
        let (correct_f, _) = internal_to_cnf_ast("~P /\\ ~Q").unwrap();
        assert_eq!(move_nots_inward_not(f), correct_f);
    }
    
    #[test]
    fn move_nots_inward_not_4() {
        let (f, _) = internal_to_cnf_ast("exists x. ~P(x)").unwrap();
        let (correct_f, _) = internal_to_cnf_ast("forall x. P(x)").unwrap();
        assert_eq!(move_nots_inward_not(f), correct_f);
    }
    
    #[test]
    fn move_nots_inward_not_5() {
        let (f, _) = internal_to_cnf_ast("forall x. ~P(x)").unwrap();
        let (correct_f, _) = internal_to_cnf_ast("exists x. P(x)").unwrap();
        assert_eq!(move_nots_inward_not(f), correct_f);
    }
    
    #[test]
    fn move_nots_inward_1() {
        let (f, _) = internal_to_cnf_ast("~((P \\/ Q) /\\ ~~(~P /\\ Q))").unwrap();
        let (correct_f, _) = internal_to_cnf_ast("~P /\\ ~Q \\/ (P \\/ ~Q)").unwrap();
        assert_eq!(move_nots_inward(f), correct_f);
    }
}