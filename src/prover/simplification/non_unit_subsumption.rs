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

use std::collections::hash_map::HashMap;
use prover::data_structures::literal::Literal;
use prover::data_structures::clause::Clause;
use prover::data_structures::term::Term;
use prover::unification::matching::term_match_general;
use prover::unification::substitution::Substitution;

fn match_literals(substitution: Substitution,
                  p: &Literal,
                  q: &Literal,
                  mixed: bool)
                  -> Option<Substitution> {
    let eqs = if mixed {
        vec![(p.get_rhs().clone(), q.get_lhs().clone()), (p.get_lhs().clone(), q.get_rhs().clone())]
    } else {
        vec![(p.get_lhs().clone(), q.get_lhs().clone()), (p.get_rhs().clone(), q.get_rhs().clone())]
    };
    term_match_general(substitution, eqs)
}

fn subsumes_clause(substitution: Substitution,
                   exclusion: &mut Vec<bool>,
                   cl1: &Clause,
                   cl2: &Clause,
                   n: usize)
                   -> bool {
    if n >= cl1.size() {
        true
    } else {
        let l1 = &cl1[n];

        for (i, l2) in cl2.iter().enumerate() {
            if !exclusion[i] && l1.polarity_equal(l2) {
                // First one way...
                if let Some(new_subst) = match_literals(substitution.clone(), l1, l2, false) {
                    exclusion[i] = true;
                    let res = subsumes_clause(new_subst, exclusion, cl1, cl2, n + 1);
                    exclusion[i] = false;
                    if res {
                        return true;
                    }
                }

                // ...and then the other way due to symmetry.
                if let Some(new_subst) = match_literals(substitution.clone(), l1, l2, true) {
                    exclusion[i] = true;
                    let res = subsumes_clause(new_subst, exclusion, cl1, cl2, n + 1);
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

/// Checks that a number of preconditions are fulfilled for cl1 subsuming cl2.
fn fulfills_preconditions(cl1: &Clause, cl2: &Clause) -> bool {
    // Since this is non-unit subsumption we do not care about unit clauses.
    if cl1.size() <= 1 {
        return false;
    }

    // Obviously cl1 cannot have more literals than cl2.
    if cl1.size() > cl2.size() {
        return false;
    } 
    
    let cl1_pos_size = cl1.positive_size();
    let cl2_pos_size = cl2.positive_size();
    
    // Similarly for positive literals.
    if cl1_pos_size > cl2_pos_size {
        return false;
    }
    
    // Similarly for negative literals.
    if cl1.size() - cl1_pos_size > cl2.size() - cl2_pos_size {
        return false;
    }
    
    // Also cl1 cannot have more function symbols than cl2.
    if cl1.symbol_count(1, 0) > cl2.symbol_count(1, 0) {
        return false;
    }
    
    not_more_function_symbols(cl1, cl2, true) && not_more_function_symbols(cl1, cl2, false)
}

/// Checks that cl1 doesn't have more (positive or negative) function symbols than cl2.
fn not_more_function_symbols(cl1: &Clause, cl2: &Clause, pos_lit: bool) -> bool {
    let mut counts = HashMap::new();
    update_function_symbol_count(&mut counts, cl1, 1, pos_lit);
    update_function_symbol_count(&mut counts, cl2, -1, pos_lit);
    counts.values().all(|&count| count <= 0)
}

/// Helper function for updating.
fn update_function_symbol_count(counts: &mut HashMap<i64, i64>, cl: &Clause, weight: i64, pos_lit: bool) {
    for l in cl.iter() {
        if l.is_positive() == pos_lit {
            update_function_symbol_count_in_term(counts, l.get_lhs(), weight);
            update_function_symbol_count_in_term(counts, l.get_rhs(), weight);
        }
    }
}

/// Helper function for updating.
fn update_function_symbol_count_in_term(counts: &mut HashMap<i64, i64>, t: &Term, weight: i64) {
    if t.is_function() {
        // And once again lexical borrows hit.
        {
            let v = counts.entry(t.get_id()).or_insert(0);
            *v += weight;
        }
        
        for x in t.iter() {
            update_function_symbol_count_in_term(counts, x, weight);
        }
    }
}

/// Checks if a clause is non-unit subsumed by the active set.
/// We use multiset subsumption instead of set subsumption to prevent some undesirable effects.
/// An example is the possibility of a clause subsuming its factors.
/// Time complexity is O(n! * 2^n) which is kinda ridiculous. In practice n is small (<=5) though.
pub fn non_unit_subsumed(active: &[Clause], cl: &Clause) -> bool {
    for act_cl in active {
        if fulfills_preconditions(act_cl, cl) {
            let mut exclusion = vec![false; cl.size()];
            if subsumes_clause(Substitution::new(), &mut exclusion, act_cl, cl, 0) {
                return true;
            }
        }
    }

    false
}

#[cfg(out_of_order)]
mod test {
    use super::subsumed;
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
        let cl1 = Clause::new(vec![l1, l2]);
        let cl2 = Clause::new(vec![l3]);

        assert!(!subsumed(&cl1, &cl2));
        assert!(subsumed(&cl2, &cl1));
    }

    #[test]
    fn subsumes_clause_2() {
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let z = Term::new_variable(-3);
        let f_x = Term::new_function(1, vec![x]);
        let f_f_x = Term::new_function(1, vec![f_x.clone()]);
        let l1 = Literal::new(false, f_x, y);
        let l2 = Literal::new(false, z, f_f_x);
        let cl1 = Clause::new(vec![l1]);
        let cl2 = Clause::new(vec![l2]);

        assert!(subsumed(&cl1, &cl2));
        assert!(!subsumed(&cl2, &cl1));
    }

    #[test]
    fn subsumes_clause_3() {
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let z = Term::new_variable(-3);
        let w = Term::new_variable(-4);
        let f_x = Term::new_function(1, vec![x]);
        let l1 = Literal::new(false, z, w);
        let l2 = Literal::new(false, f_x, y);
        let cl1 = Clause::new(vec![l1]);
        let cl2 = Clause::new(vec![l2]);

        assert!(subsumed(&cl1, &cl2));
        assert!(!subsumed(&cl2, &cl1));
    }

    #[test]
    fn subsumes_clause_4() {
        let x1 = Term::new_variable(-1);
        let x2 = Term::new_variable(-2);
        let cl_l1 = Literal::new(false,
                                 Term::new_function(1, vec![x1.clone()]),
                                 Term::new_truth());
        let cl1_l2 = Literal::new(true,
                                  Term::new_special_function(2, vec![x1]),
                                  Term::new_truth());
        let cl2_l2 = Literal::new(true,
                                  Term::new_special_function(2, vec![x2]),
                                  Term::new_truth());

        let cl1 = Clause::new(vec![cl_l1.clone(), cl1_l2]);
        let cl2 = Clause::new(vec![cl_l1, cl2_l2]);

        assert!(!subsumed(&cl1, &cl2));
        assert!(subsumed(&cl2, &cl1));
    }

    #[test]
    fn subsumes_clause_5() {
        // A clause always subsumes itself.
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let c = Term::new_function(1, Vec::new());
        let f_c_x = Term::new_function(2, vec![c.clone(), x.clone()]);
        let f_y_c = Term::new_function(2, vec![y.clone(), c]);
        let l1 = Literal::new(false, f_c_x, f_y_c);
        let l2 = Literal::new(true, x, y);

        let cl = Clause::new(vec![l1, l2]);

        assert!(subsumed(&cl, &cl));
    }
}
