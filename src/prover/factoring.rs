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
// use prover::unification::mgu;

/// Factors a clause if possible. Returns true if something was factored.
pub fn factor(cl: &mut Clause) -> bool {
    let factored_something = false;
    let mut i = 0;
    
    while i < cl.size() {
        let mut j = i + 1;
        
        while j < cl.size() {
            /*
            if let Ok(theta) = mgu(vec!(cl[i].clone(), cl[j].clone())) {
                cl.swap_remove(j);
                for l in cl.iter_mut() {
                    *l = tsubst(l.clone(), &theta);
                }
                factored_something = true;
                continue;
            }
            */
            j += 1;
        }
        
        i += 1;
    }
    
    factored_something
}

#[cfg(test)]
mod test {
    
} 

