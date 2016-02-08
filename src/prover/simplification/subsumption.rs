/*
    Serkr - An automated theorem prover. Copyright (C) 2015-2016 Mikko Aarnos.

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

use prover::data_structures::literal::Literal;
use prover::data_structures::clause::Clause;
use prover::unification::matching::term_match_general;
use prover::unification::substitution::Substitution;

fn match_literals(substitution: Substitution, p: &Literal, q: &Literal, mixed: bool) -> Option<Substitution> {
    let eqs = if !mixed { vec!((p.get_lhs().clone(), q.get_lhs().clone()), 
                               (p.get_rhs().clone(), q.get_rhs().clone())) }
              else { vec!((p.get_rhs().clone(), q.get_lhs().clone()), 
                          (p.get_lhs().clone(), q.get_rhs().clone())) };
    term_match_general(substitution, eqs)
}

fn subsumes_clause0(substitution: Substitution, exclusion: &mut Vec<bool>, cl1: &Clause, cl2: &Clause, n: usize) -> bool {
    if n >= cl1.size() {
        true 
    } else { 
        let l1 = &cl1[n];

        for (i, l2) in cl2.iter().enumerate() {
            if !exclusion[i] && l1.polarity_equal(l2) {
                // First one way... 
                if let Some(new_substitution) = match_literals(substitution.clone(), l1, l2, false) {
                    exclusion[i] = true;
                    let res = subsumes_clause0(new_substitution, exclusion, cl1, cl2, n + 1);
                    exclusion[i] = false;
                    if res {
                        return true;
                    }
                }
                
                // ...and then the other way due to symmetry.
                if let Some(new_substitution) = match_literals(substitution.clone(), l1, l2, true) {
                    exclusion[i] = true;
                    let res = subsumes_clause0(new_substitution, exclusion, cl1, cl2, n + 1);
                    exclusion[i] = false;
                    if res {
                        return true;
                    }
                }
            }
        }
        
        false
    }
}

/// A clause cannot subsume another clause unless the top function symbols subsume the top function symbols in the other clause.
// TODO: clean this crap up
fn function_symbols_subsume(cl1: &Clause, cl2: &Clause) -> bool {
    let mut exclusion = vec![false; cl2.size()];
    for l1 in cl1.iter() {
        let l1_f_l = l1.get_lhs().is_function();
        let l1_f_r = l1.get_rhs().is_function();
        
        if !l1_f_l && !l1_f_r {
            continue;
        }
    
        let mut found_match = false;
        for (i, l2) in cl2.iter().enumerate() {
            if !exclusion[i] && l1.polarity_equal(l2) {
                let l_id_l = l1.get_lhs().get_id();
                let r_id_l = l2.get_lhs().get_id();
                let l_id_r = l1.get_rhs().get_id();
                let r_id_r = l2.get_rhs().get_id();
                
                let functions_subsumed = if l1_f_l && l1_f_r {
                                            (l_id_l == r_id_l && l_id_r == r_id_r) || (l_id_l == r_id_r && l_id_r == r_id_l)
                                         } else if l1_f_l {
                                            (l_id_l == r_id_l || l_id_l == r_id_r)
                                         } else {
                                            (l_id_r == r_id_l || l_id_r == r_id_r)
                                         };
                                         
                if functions_subsumed {
                    exclusion[i] = true;
                    found_match = true;
                    break;
                }
            }    
        }
        
        if !found_match {
            return false;
        }
    }
    
    true
}

/// Checks if the clause cl1 subsumes the clause cl2.
/// We use multiset subsumption instead of set subsumption to prevent some undesirable effects like a clause subsuming its factors.
/// Time complexity is O(n! * 2^n) which is kinda ridiculous. In practice n is small (<=5) though.
pub fn subsumes_clause(cl1: &Clause, cl2: &Clause) -> bool {
    if cl1.size() <= cl2.size() {
        if function_symbols_subsume(cl1, cl2) {
            let mut exclusion = vec![false; cl2.size()];
            subsumes_clause0(Substitution::new(), &mut exclusion, cl1, cl2, 0)
        } else {
            false
        }    
    } else {
        false
    }
}

#[cfg(test)]
mod test {
    use super::subsumes_clause;
    use prover::data_structures::term::Term;
    use prover::data_structures::literal::Literal;
    use prover::data_structures::clause::Clause;
    
    #[test]
    fn subsumes_clause_1() {
        // A clause with more literals cannot subsume a clause with less literals
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let z = Term::new_variable(-3);
        let w = Term::new_variable(-4);   
        let l1 = Literal::new(false, x, y.clone());
        let l2 = Literal::new(false, z, w.clone());
        let l3 = Literal::new(false, y.clone(), w.clone());
        let cl1 = Clause::new(vec!(l1, l2));
        let cl2 = Clause::new(vec!(l3));
        
        assert!(!subsumes_clause(&cl1, &cl2));
        assert!(subsumes_clause(&cl2, &cl1));
    }
    
    #[test]
    fn subsumes_clause_2() {
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let z = Term::new_variable(-3);
        let f_x = Term::new_function(1, vec!(x));
        let f_f_x = Term::new_function(1, vec!(f_x.clone()));
        let l1 = Literal::new(false, f_x, y);
        let l2 = Literal::new(false, z, f_f_x);
        let cl1 = Clause::new(vec!(l1));
        let cl2 = Clause::new(vec!(l2));
        
        assert!(subsumes_clause(&cl1, &cl2));
        assert!(!subsumes_clause(&cl2, &cl1));
    }
    
    #[test]
    fn subsumes_clause_3() {
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let z = Term::new_variable(-3);
        let w = Term::new_variable(-4); 
        let f_x = Term::new_function(1, vec!(x));
        let l1 = Literal::new(false, z, w);
        let l2 = Literal::new(false, f_x, y);
        let cl1 = Clause::new(vec!(l1));
        let cl2 = Clause::new(vec!(l2));
        
        assert!(subsumes_clause(&cl1, &cl2));
        assert!(!subsumes_clause(&cl2, &cl1));
    }
    
    #[test]
    fn subsumes_clause_4() {
        let x1 = Term::new_variable(-1);
        let x2 = Term::new_variable(-2);
        let cl_l1 = Literal::new(false, Term::new_function(1, vec!(x1.clone())), Term::new_truth());
        let cl1_l2 = Literal::new(true, Term::new_special_function(2, vec!(x1)), Term::new_truth());
        let cl2_l2 = Literal::new(true, Term::new_special_function(2, vec!(x2)), Term::new_truth());
        
        let cl1 = Clause::new(vec!(cl_l1.clone(), cl1_l2));
        let cl2 = Clause::new(vec!(cl_l1, cl2_l2));
        
        assert!(!subsumes_clause(&cl1, &cl2));
        assert!(subsumes_clause(&cl2, &cl1));
    }
    
    #[test]
    fn subsumes_clause_5() {
        // A clause always subsumes itself.
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let c = Term::new_function(1, Vec::new());
        let f_c_x = Term::new_function(2, vec!(c.clone(), x.clone()));
        let f_y_c = Term::new_function(2, vec!(y.clone(), c));
        let l1 = Literal::new(false, f_c_x, f_y_c);
        let l2 = Literal::new(true, x, y);
        
        let cl = Clause::new(vec!(l1, l2));
        
        assert!(subsumes_clause(&cl, &cl));
    }
} 

