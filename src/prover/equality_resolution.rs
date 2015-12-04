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
use prover::unification::mgu;
use prover::tautology_deletion::trivial;
use prover::term_ordering::TermOrdering;

/// Infers new clauses by (ordered) equality resolution.
/// Time complexity is O(n) where n is the amount of literals in the clause.
pub fn equality_resolution<T: TermOrdering + ?Sized>(term_ordering: &T, cl: &Clause, resolvents: &mut Vec<Clause>) {
    for (i, l) in cl.iter().enumerate() {
        if l.is_negative() {
            if let Some(theta) = mgu(l.get_lhs(), l.get_rhs()) {
                let mut new_cl = cl.clone();
                let mut new_l = l.clone();
                new_cl.swap_remove(i);
                new_cl.subst(&theta);
                new_l.subst(&theta);
                
                if !trivial(&new_cl) && new_cl.iter().all(|lit| !term_ordering.gt_lit(lit, &new_l)) {
                    resolvents.push(new_cl);
                } 
            }
        }
    }
} 

#[cfg(test)]
mod test {
    
} 