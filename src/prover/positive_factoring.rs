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
use prover::unification::mgu_literals;
use prover::tautology_deletion::trivial;

/// Infers new clauses by positive factoring
/// Time complexity is O(n^2) where n is the amount of literals, but usually the clauses are rather short.
// TODO: see how much time is spent here.
pub fn positive_factoring(cl: &Clause, factors: &mut Vec<Clause>) {
    for (i, l) in cl.iter().enumerate() {
        if !l.is_positive() {
            continue;
        }
        
        for j in (i + 1)..cl.size() {
            if !cl[j].is_positive() {
                continue;
            }
            
            if let Ok(theta) = mgu_literals(l, &cl[j]) {
                let mut new_cl = cl.clone();
                new_cl.swap_remove(j);
                new_cl.subst(&theta);
                
                assert!(new_cl.size() + 1 == cl.size());
                
                if !trivial(&new_cl) {
                    factors.push(new_cl);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    
} 