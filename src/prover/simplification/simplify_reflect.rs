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
use prover::simplification::equality_subsumption::{matching_equation_exists, equation_subsumed};
use prover::indexing::top_symbol_hashing::TopSymbolHashIndex;

/// Simplifies a given clause by positive and negative simplify-reflect.
pub fn simplify_reflect(term_index: &TopSymbolHashIndex, cl: &mut Clause) {
    let mut i = 0;

    while i < cl.size() {
        if simplifiable(term_index, &cl[i]) {
            cl.swap_remove(i);
            continue;
        }

        i += 1;
    }
}

fn simplifiable(term_index: &TopSymbolHashIndex, l: &Literal) -> bool {
    if l.is_positive(){
        matching_equation_exists(term_index, l.get_lhs(), l.get_rhs(), false)
    } else {
        equation_subsumed(term_index, l.get_lhs(), l.get_rhs())
    }
}

#[cfg(test)]
mod test {}
