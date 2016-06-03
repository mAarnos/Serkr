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
use prover::unification::matching::term_match;
use prover::indexing::top_symbol_hashing::TopSymbolHashIndex;

// Note: rewriting is probably incomplete as it is used currently. Fix that.

/// Does one rewrite step.
/// Returns true if something was rewritten.
fn normal_form_step(term_ordering: &TermOrdering, term_index: &TopSymbolHashIndex, t: &mut Term) -> bool {
    if let Some(iter) = term_index.possible_matches(t) {
        for &(ref l, ref r, _) in iter.filter(|x| x.2) {
            if let Some(sigma) = term_match(l, t) {
                let mut new_l = l.clone();
                let mut new_r = r.clone();
                new_l.subst(&sigma);
                new_r.subst(&sigma);
                if term_ordering.gt(&new_l, &new_r) {
                    *t = new_r;
                    return true;
                }
            }
        }
    }

    t.iter_mut().any(|t2| normal_form_step(term_ordering, term_index, t2))
}

/// Reduces a term into normal form with regards to the active clause set.
fn normal_form(term_ordering: &TermOrdering, term_index: &TopSymbolHashIndex, t: &mut Term) {
    while normal_form_step(term_ordering, term_index, t) {
    }
}

/// Rewrites a given clause into normal form with regards to the active clause set.
pub fn rewrite_literals(term_ordering: &TermOrdering, term_index: &TopSymbolHashIndex, cl: &mut Clause) {
    for l in cl.iter_mut() {
        normal_form(term_ordering, term_index, l.get_lhs_mut());
        normal_form(term_ordering, term_index, l.get_rhs_mut());
    }
}

#[cfg(test)]
mod test {}
