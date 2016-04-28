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
use prover::unification::matching::match_term_pairs;
use prover::simplification::equality_subsumption::eqn_subsumes_eqn;

/// Simplifies cl2 by positive and negative simplify-reflect if possible.
pub fn simplify_reflect(cl1: &Clause, cl2: &mut Clause) {
    if cl1.is_unit() {
        let l = &cl1[0];
        if l.is_positive() {
            positive_simplify_reflect(l.get_lhs(), l.get_rhs(), cl2);
        } else {
            negative_simplify_reflect(l.get_lhs(), l.get_rhs(), cl2);
        }
    }
}

/// Simplifies the clause with positive simplify-reflect.
fn positive_simplify_reflect(s: &Term, t: &Term, cl: &mut Clause) {
    let mut i = 0;

    while i < cl.size() {
        if cl[i].is_negative() && eqn_subsumes_eqn(s, t, cl[i].get_lhs(), cl[i].get_rhs()) {
            cl.swap_remove(i);
            continue;
        }

        i += 1;
    }
}

/// Simplifies the clause with negative simplify-reflect.
fn negative_simplify_reflect(s: &Term, t: &Term, cl: &mut Clause) {
    let mut i = 0;

    while i < cl.size() {
        if cl[i].is_positive() && match_term_pairs(s, t, cl[i].get_lhs(), cl[i].get_rhs()) {
            cl.swap_remove(i);
            continue;
        }

        i += 1;
    }
}

#[cfg(test)]
mod test {}
