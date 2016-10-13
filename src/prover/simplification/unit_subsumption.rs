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

use prover::data_structures::pd_tree::PDTree;
use prover::data_structures::clause::Clause;
use prover::simplification::equality_subsumption::matching_equation_exists;

/// Checks if a clause is unit-subsumed by the active set.
pub fn unit_subsumed(term_index: &PDTree, cl: &Clause) -> bool {
    cl.iter()
      .any(|l| matching_equation_exists(term_index, l.get_lhs(), l.get_rhs(), l.is_positive()))
}

#[cfg(test)]
mod test {

}
