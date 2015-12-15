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

use prover::clause::Clause;
use prover::unification::full_unification::mgu;
use prover::ordering::term_ordering::TermOrdering;

/// Infers new clauses by (ordered) equality resolution.
/// Time complexity is O(n) where n is the amount of literals in the clause.
/// Returns the amount of inferred clauses.
pub fn equality_resolution(term_ordering: &TermOrdering, cl: &Clause, generated: &mut Vec<Clause>) -> usize {
    let mut er_count = 0;
    
    for (i, l) in cl.iter().enumerate() {
        if l.is_negative() {
            if let Some(sigma) = mgu(l.get_lhs(), l.get_rhs()) {
                let mut new_cl = cl.clone();
                let mut new_l = l.clone();
                new_cl.swap_remove(i);
                new_cl.subst(&sigma);
                new_l.subst(&sigma);
                
                if new_cl.iter().all(|lit| !term_ordering.gt_lit(lit, &new_l)) {
                    generated.push(new_cl);
                    er_count += 1;
                } 
            }
        }
    }
    
    er_count
} 

#[cfg(test)]
mod test {
    
} 