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

// Note: rewriting is probably incomplete as it is used currently. Fix that.
// To be specific, we rewrite maximal literals in clauses with no restriction.
// I'd like to see a concrete example where it causes problems first though.

use prover::data_structures::term::Term;
use prover::data_structures::clause::Clause;
use prover::ordering::term_ordering::TermOrdering;
use prover::data_structures::pd_tree::PDTree;

/// Rewrites a term into some normal form.
/// Tries to rewrite the leftmost and innermost terms first.
fn rewrite_to_normal_form(term_ordering: &TermOrdering, term_index: &PDTree, t: &mut Term) {
    if t.is_function() {
        for sub_t in t.iter_mut() {
            rewrite_to_normal_form(term_ordering, term_index, sub_t);
        }
        
        if let Some(new_r) = normal_form_step_helper(term_ordering, term_index, t) {
            *t = new_r;
            rewrite_to_normal_form(term_ordering, term_index, t);
        }
    }
}

/// Helper for above. Gets around lifetime issues with the iterator.
fn normal_form_step_helper(term_ordering: &TermOrdering, term_index: &PDTree, t: &Term) -> Option<Term> {
    for (_, r, sigma, oriented) in term_index.iter_generalizations(t, true) {
        let mut new_r = r.clone();
        new_r.subst(&sigma);
        if oriented || term_ordering.gt(t, &new_r) {
            return Some(new_r);
        }
    }
    
    None
}

/// Rewrites a given clause into normal form with regards to the active clause set.
pub fn rewrite_clause(term_ordering: &TermOrdering, term_index: &PDTree, cl: &mut Clause) {
    for l in cl.iter_mut() {
        rewrite_to_normal_form(term_ordering, term_index, l.get_lhs_mut());
        rewrite_to_normal_form(term_ordering, term_index, l.get_rhs_mut());
    }
}

#[cfg(test)]
mod test {}
