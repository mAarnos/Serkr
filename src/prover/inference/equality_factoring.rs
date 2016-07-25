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

// TODO: clean this crap up

use prover::data_structures::term::Term;
use prover::data_structures::literal::Literal;
use prover::data_structures::clause::Clause;
use prover::unification::full_unification::mgu;
use prover::ordering::term_ordering::TermOrdering;
use prover::inference::maximality::literal_maximal_in;

/// Infers new clauses by equality factoring
/// Returns the amount of inferred clauses.
pub fn equality_factoring(term_ordering: &TermOrdering,
                          cl: &Clause,
                          generated: &mut Vec<Clause>)
                          -> usize {
    let mut ef_count = 0;

    for (i, l1) in cl.iter().enumerate() {
        if l1.is_negative() {
            continue;
        }

        for l2 in cl.iter().skip(i + 1) {
            if l2.is_negative() {
                continue;
            }

            // So we have found two equality literals. There are four ways to try to combine them.
            ef_count += equality_factoring_create_new(term_ordering,
                                                      cl,
                                                      generated,
                                                      l1.get_lhs(),
                                                      l1.get_rhs(),
                                                      l2.get_lhs(),
                                                      l2.get_rhs(),
                                                      i);
            ef_count += equality_factoring_create_new(term_ordering,
                                                      cl,
                                                      generated,
                                                      l1.get_lhs(),
                                                      l1.get_rhs(),
                                                      l2.get_rhs(),
                                                      l2.get_lhs(),
                                                      i);
            ef_count += equality_factoring_create_new(term_ordering,
                                                      cl,
                                                      generated,
                                                      l1.get_rhs(),
                                                      l1.get_lhs(),
                                                      l2.get_lhs(),
                                                      l2.get_rhs(),
                                                      i);
            ef_count += equality_factoring_create_new(term_ordering,
                                                      cl,
                                                      generated,
                                                      l1.get_rhs(),
                                                      l1.get_lhs(),
                                                      l2.get_rhs(),
                                                      l2.get_lhs(),
                                                      i);
        }
    }

    ef_count
}

#[cfg_attr(feature="clippy", allow(too_many_arguments))]
fn equality_factoring_create_new(term_ordering: &TermOrdering,
                                 cl: &Clause,
                                 generated: &mut Vec<Clause>,
                                 s: &Term,
                                 t: &Term,
                                 u: &Term,
                                 v: &Term,
                                 i: usize)
                                 -> usize {
    let mut ef_count = 0;

    if let Some(sigma) = mgu(s, u) {
        let mut l = Literal::new(false, s.clone(), t.clone());
        l.subst(&sigma);

        if !term_ordering.gt(l.get_rhs(), l.get_lhs()) {
            let mut new_cl = cl.clone();
            new_cl.swap_remove(i);
            new_cl.subst(&sigma);

            if literal_maximal_in(term_ordering, &new_cl, &l) {
                let mut new_ineq_lit = Literal::new(true, t.clone(), v.clone());
                new_ineq_lit.subst(&sigma);
                new_cl.add_literal(new_ineq_lit);
                generated.push(new_cl);
                ef_count += 1;
            }
        }
    }

    ef_count
}

#[cfg(test)]
mod test {}
