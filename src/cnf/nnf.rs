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

/// Converts a formula into an equivalent negation normal form.
pub fn nnf(f: Formula) -> Formula {
    move_nots_inward(elim_imp_and_eq(f))
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

/// Moves all NOTs inward by repeatedly applying De Morgan's laws.
fn move_nots_inward(f: Formula) -> Formula {
    match f {
        Formula::Not(box Formula::Not(box p)) => move_nots_inward(p),
        Formula::Not(box Formula::And(box ref p, box ref q)) => Formula::Or(box move_nots_inward(Formula::Not(box p.clone())), box move_nots_inward(Formula::Not(box q.clone()))),
        Formula::Not(box Formula::Or(box ref p, box ref q)) => Formula::And(box move_nots_inward(Formula::Not(box p.clone())), box move_nots_inward(Formula::Not(box q.clone()))),
        Formula::Not(box Formula::Forall(ref s, box ref p)) => Formula::Exists(s.clone(), box move_nots_inward(Formula::Not(box p.clone()))),
        Formula::Not(box Formula::Exists(ref s, box ref p)) => Formula::Forall(s.clone(), box move_nots_inward(Formula::Not(box p.clone()))),
        Formula::Not(box p) => Formula::Not(box move_nots_inward(p)),
        Formula::And(box p, box q) => Formula::And(box move_nots_inward(p), box move_nots_inward(q)),
        Formula::Or(box p, box q) => Formula::Or(box move_nots_inward(p), box move_nots_inward(q)),
        Formula::Forall(s, box p) => Formula::Forall(s, box move_nots_inward(p)),
        Formula::Exists(s, box p) => Formula::Exists(s, box move_nots_inward(p)),
        _ => f
    }
}

#[cfg(test)]
mod test {
    use super::{elim_imp_and_eq};
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
}