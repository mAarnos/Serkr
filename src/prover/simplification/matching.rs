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

/// Checks whether we can update the substitution given in so that for all equations given the lhs is equal to the rhs.
/// Returns true if it finds such a substitution.
/// Note that stuff might be added to the end of env.
pub fn term_match(env: &mut Vec<(Term, Term)>, mut eqs: Vec<(Term, Term)>) -> bool {
    while let Some((eq1, eq2)) = eqs.pop() {
        if eq1.is_function() && eq2.is_function() {
            if eq1.get_id() == eq2.get_id() {
                for eq in eq1.iter().cloned().zip(eq2.iter().cloned()) {
                    eqs.push(eq);
                }
            } else {
                return false;
            }
        } else if eq1.is_variable() {
            let mut success = false;
            for x in env.iter() {
                if x.0 == eq1 {
                    if x.1 != eq2 {
                        return false;
                    }
                    success = true;
                    break;
                }
            }
            
            if !success {
                // Can't unify between two different sorts.
                if eq2.get_sort_predicate() {
                    return false;
                }
                env.push((eq1, eq2));
            }
            
        } else {
            return false;
        }
    }
    
    true
}

/// Check if there is a sigma so that s\sigma = u and t\sigma = v (or with s and t switched).
pub fn match_term_pairs(s: &Term, t: &Term, u: &Term, v: &Term) -> bool {
    let mut env = Vec::new();
    let eqs = vec!((s.clone(), u.clone()), (t.clone(), v.clone()));
    if term_match(&mut env, eqs) {
        true
    } else {
        let eqs2 = vec!((t.clone(), u.clone()), (s.clone(), v.clone()));
        env.clear();
        term_match(&mut env, eqs2)
    }
}

#[cfg(test)]
mod test {

} 

