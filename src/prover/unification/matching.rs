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
use prover::unification::substitution::Substitution;

/// Checks whether we can update the substitution given in so that for all equations given the lhs is equal to the rhs.
/// It must be noted that the substitution is only for lhs, unlike full unification.
/// Returns the new substitution if it exists.
pub fn term_match(mut substitution: Substitution, mut eqs: Vec<(Term, Term)>) -> Option<Substitution> {
    while let Some((eq1, eq2)) = eqs.pop() {
        if eq1.is_function() && eq2.is_function() {
            if eq1.get_id() == eq2.get_id() {
                for eq in eq1.into_iter().zip(eq2.into_iter()) {
                    eqs.push(eq);
                }
            } else {
                return None;
            }
        } else if eq1.is_variable() {
            // Can't unify between two different sorts.
            if eq2.get_sort_predicate() {
                return None;
            }
            
            // Suffers from the lexical scope borrow bug.
            let mut probe_success = false;
            if let Some(v) = substitution.get(&eq1) {
                if *v != eq2 {
                    return None;
                }
                probe_success = true;
            } 
            
            if !probe_success {
                substitution.insert(eq1, eq2);
            }
        } else {
            return None;
        }
    }
    
    Some(substitution)
}

/// Check if there is a sigma so that s\sigma = u and t\sigma = v (or with s and t switched).
pub fn match_term_pairs(s: &Term, t: &Term, u: &Term, v: &Term) -> bool {
    let eqs = vec!((s.clone(), u.clone()), (t.clone(), v.clone()));
    if term_match(Substitution::new(), eqs).is_some() {
        true
    } else {
        let eqs2 = vec!((t.clone(), u.clone()), (s.clone(), v.clone()));
        term_match(Substitution::new(), eqs2).is_some()
    }
}

#[cfg(test)]
mod test {

} 

