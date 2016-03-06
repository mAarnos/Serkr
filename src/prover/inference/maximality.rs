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

use prover::data_structures::literal::Literal;
use prover::data_structures::clause::Clause;
use prover::ordering::term_ordering::TermOrdering;

/// Checks if a given literal is maximal in a clause according to a given term ordering.
pub fn literal_maximal_in(term_ordering: &TermOrdering, cl: &Clause, l: &Literal) -> bool {
    cl.iter().all(|l2| !term_ordering.gt_lit(l2, l))
}

/// Checks if a given literal is strictly maximal in a clause according to a given term ordering.
pub fn literal_strictly_maximal_in(term_ordering: &TermOrdering, cl: &Clause, l: &Literal) -> bool {
    cl.iter().all(|l2| !term_ordering.ge_lit(l2, l))
}
