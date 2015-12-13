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

use prover::term::Term;
use prover::clause::Clause;
use prover::term_ordering::traits::TermOrdering;

/// Does one rewrite step.
/// Returns true if something was rewritten.
fn normal_form_step<T: TermOrdering + ?Sized>(term_ordering: &T, active: &[Clause], t: &mut Term) -> bool {
    for cl in active {
        if cl.is_unit() && cl[0].is_positive() {
            unimplemented!();
        }
    }
        
    t.iter_mut().any(|t2| normal_form_step(term_ordering, active, t2))
}

/// Reduces a term into normal form with regards to the active clause set.
fn normal_form<T: TermOrdering + ?Sized>(term_ordering: &T, active: &[Clause], t: &mut Term){
    while normal_form_step(term_ordering, active, t) {
    }
}

/// Rewrites a given clause into normal form with regards to the active clause set.
pub fn rewrite_literals<T: TermOrdering + ?Sized>(term_ordering: &T, active: &[Clause], cl: &mut Clause) {
    for l in cl.iter_mut() {
        normal_form(term_ordering, active, l.get_lhs_mut());
        normal_form(term_ordering, active, l.get_rhs_mut());
    }
}

#[cfg(test)]
mod test {

} 
