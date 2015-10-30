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
use prover::trivial::trivial;

/// Infers new clauses by factoring two literals in the input clause.
#[allow(needless_range_loop)]
pub fn factor(cl: Clause, unused: &mut Vec<Clause>) {
    for i in 0..cl.size() {
        for j in (i + 1)..cl.size() {
            if let Ok(theta) = mgu(cl.at(i), cl.at(j)) {
                let mut new_cl = cl.clone();
                new_cl.swap_remove(j);
                for l in new_cl.iter_mut() {
                    l.tsubst(&theta);
                }
                // TODO: add subsumption checks here?
                if !trivial(&new_cl) {
                    unused.push(new_cl);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    
} 

