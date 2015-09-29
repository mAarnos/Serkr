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
use cnf::simplify::simplify;

/// Converts a formula into an equivalent negation normal form.
pub fn nnf(f: Formula) -> Formula {
    let simplified_f = simplify(f);
    match simplified_f {
        Formula::True => Formula::True,
        Formula::False => Formula::False,
        _ => move_nots_inward(elim_imp_and_eq(simplified_f))
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

#[cfg(test)]
mod test {
    use super::{elim_imp_and_eq, move_nots_inward, move_nots_inward_not};
    use parser::formula::{Formula};
    
    #[test]
    fn elim_imp_and_eq_1() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        let pred2 = Formula::Predicate("Q".to_string(), Vec::new());
        let f = Formula::Implies(box pred.clone(), box pred2.clone()); 
        assert_eq!(elim_imp_and_eq(f), Formula::Or(box Formula::Not(box pred), box pred2));
    }
    
    #[test]
    fn elim_imp_and_eq_2() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        let pred2 = Formula::Predicate("Q".to_string(), Vec::new());
        let f = Formula::Equivalent(box pred.clone(), box pred2.clone()); 
        assert_eq!(elim_imp_and_eq(f), Formula::And(box Formula::Or(box pred.clone(), 
                                                                    box Formula::Not(box pred2.clone())), 
                                                    box Formula::Or(box Formula::Not(box pred),
                                                                    box pred2)));
    }
    
    #[test]
    fn elim_imp_and_eq_3() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        let pred2 = Formula::Predicate("Q".to_string(), Vec::new()); 
        
        // (P -> Q) <-> (~Q -> ~P)
        let f = Formula::Equivalent(box Formula::Implies(box pred.clone(), box pred2.clone()), 
                                    box Formula::Implies(box Formula::Not(box pred2.clone()), box Formula::Not(box pred.clone())));
        
        // ((~P \/ Q) \/ ~(~~Q \/ ~P)) /\ (~(~P \/ Q) \/ (~~Q \/ ~P))
        let correct_f_part_1 = Formula::Or(box Formula::Not(box pred.clone()), 
                                           box pred2.clone());
        let correct_f_part_4 = Formula::Or(box Formula::Not(box Formula::Not(box pred2.clone())), 
                                                            box Formula::Not(box pred));
        let correct_f_part_3 = Formula::Not(box correct_f_part_1.clone());
        let correct_f_part_2 = Formula::Not(box correct_f_part_4.clone());
        let correct_f = Formula::And(box Formula::Or(box correct_f_part_1, box correct_f_part_2),
                                     box Formula::Or(box correct_f_part_3, box correct_f_part_4));
        assert_eq!(elim_imp_and_eq(f), correct_f);                             
    }
    
    #[test]
    fn move_nots_inward_not_1() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        assert_eq!(move_nots_inward_not(Formula::Not(box pred.clone())), pred);
    }
    
    #[test]
    fn move_nots_inward_not_2() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        let pred2 = Formula::Predicate("Q".to_string(), Vec::new());
        let f = Formula::Or(box Formula::Not(box pred.clone()), box Formula::Not(box pred2.clone()));
        assert_eq!(move_nots_inward_not(Formula::And(box pred, box pred2)), f);
    }
    
    #[test]
    fn move_nots_inward_not_3() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        let pred2 = Formula::Predicate("Q".to_string(), Vec::new());
        let f = Formula::And(box Formula::Not(box pred.clone()), box Formula::Not(box pred2.clone()));
        assert_eq!(move_nots_inward_not(Formula::Or(box pred, box pred2)), f);
    }
    
    #[test]
    fn move_nots_inward_not_4() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        let f = Formula::Exists("x".to_string(), box Formula::Not(box pred.clone()));
        assert_eq!(move_nots_inward_not(Formula::Forall("x".to_string(), box pred)), f);
    }
    
    #[test]
    fn move_nots_inward_not_5() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        let f = Formula::Forall("x".to_string(), box Formula::Not(box pred.clone()));
        assert_eq!(move_nots_inward_not(Formula::Exists("x".to_string(), box pred)), f);
    }
    
    #[test]
    fn elim_imp_and_eq_4() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        let pred2 = Formula::Predicate("P".to_string(), Vec::new());
        
        // ((P -> Q) /\ P) -> Q
        let f = Formula::Equivalent(box Formula::And(box Formula::Implies(box pred.clone(), box pred2.clone()), box pred.clone()), box pred2.clone());
    }
}