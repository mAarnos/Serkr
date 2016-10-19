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

use prover::data_structures::term::Term;
use prover::data_structures::clause::Clause;
use prover::ordering::term_ordering::TermOrdering;
use prover::data_structures::pd_tree::PDTree;

/// Rewrites a term into some normal form.
/// Tries to rewrite the leftmost and innermost terms first.
fn rewrite_to_normal_form(term_ordering: &TermOrdering, term_index: &PDTree, t: &mut Term, restrict: bool) {
    if t.is_function() {
        for sub_t in t.iter_mut() {
            rewrite_to_normal_form(term_ordering, term_index, sub_t, false);
        }

        if let Some(new_r) = normal_form_step_helper(term_ordering, term_index, t, restrict) {
            *t = new_r;
            rewrite_to_normal_form(term_ordering, term_index, t, restrict);
        }
    }
}

/// Helper for above. Gets around lifetime issues with the iterator.
fn normal_form_step_helper(term_ordering: &TermOrdering, term_index: &PDTree, t: &Term, restrict: bool) -> Option<Term> {
    for (_, r, sigma, oriented) in term_index.iter_generalizations(t, true) {
        if !restrict || !sigma.is_variable_renaming() {
            let mut new_r = r.clone();
            new_r.subst(&sigma);            
            if oriented || term_ordering.gt(t, &new_r) {
                return Some(new_r);
            }
        }
    }
   
    None
}

/// Returns a bit vector with the maximal literals marked.
fn maximal_literals(term_ordering: &TermOrdering, cl: &Clause) -> Vec<bool> {
    let mut bv = vec![true; cl.size()];
    
    for i in 0..cl.size() {
        if !bv[i] {
            continue;
        }
        
        for j in 0..cl.size() {
            if i != j && bv[j] {
                if term_ordering.gt_lit(&cl[i], &cl[j]) {
                    bv[j] = false;
                }
            }
        }
    }
    
    bv
}

/// Rewrites a given clause into normal form with regards to the active clause set.
pub fn rewrite_clause(term_ordering: &TermOrdering, term_index: &PDTree, cl: &mut Clause) {
    let bv = maximal_literals(term_ordering, cl);
    
    for i in 0..cl.size() {
        let l = &mut cl[i];
        let restricted = l.is_positive() && bv[i];
        if !restricted {
            rewrite_to_normal_form(term_ordering, term_index, l.get_lhs_mut(), false);
            rewrite_to_normal_form(term_ordering, term_index, l.get_rhs_mut(), false);
        } else {
            let s_t = term_ordering.gt(l.get_lhs(), l.get_rhs());
            let t_s = term_ordering.gt(l.get_rhs(), l.get_lhs());
            rewrite_to_normal_form(term_ordering, term_index, l.get_lhs_mut(), s_t);
            rewrite_to_normal_form(term_ordering, term_index, l.get_rhs_mut(), t_s);
        } 
    }
}

#[cfg(test)]
mod test {}
