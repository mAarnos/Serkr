/*
    Serkr - An automated theorem prover. Copyright (C) 2015-2016 Mikko Aarnos.

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

/// Tries to find a sigma so that s\sigma = t.
/// Returns the substitution if it exists.
/// It must be noted that we treat variables of a pair of to terms as distinct even if they have the same name.
/// This might cause us to have trivial mappings like x |-> x.
pub fn term_match(s: &Term, t: &Term) -> Option<Substitution> {
    term_match_general(Substitution::new(), vec!((s.clone(), t.clone())))
}

/// A more general version of term_match.
/// The only difference is that we can pass in a substitution to expand, and we can give more pairs of equations than just one.
pub fn term_match_general(mut substitution: Substitution, mut eqs: Vec<(Term, Term)>) -> Option<Substitution> {
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
            if eq2.is_special_function() {
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
    if term_match_general(Substitution::new(), eqs).is_some() {
        true
    } else {
        let eqs2 = vec!((t.clone(), u.clone()), (s.clone(), v.clone()));
        term_match_general(Substitution::new(), eqs2).is_some()
    }
}

#[cfg(test)]
mod test {
    use super::term_match;
    use prover::term::Term;

    #[test]
    fn term_match_1() {
        // f(x) = f(g(x))
        // Unlike with unification, this should succeed.
        let x = Term::new_variable(-1);
        let f_x = Term::new_function(1, vec!(x.clone()));
        let f_g_x = Term::new_function(1, vec!(Term::new_function(2, vec!(x))));
        assert!(term_match(&f_x, &f_g_x).is_some());
    }
    
    #[test]
    fn term_match_2() {
        // f(g(y)) = f(x)
        // With unification this would be trivial, but with matching there is no substitution.
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let f_g_y = Term::new_function(1, vec!(Term::new_function(2, vec!(y))));
        let f_x = Term::new_function(1, vec!(x));
        assert!(term_match(&f_g_y, &f_x).is_none());
    }
    
    #[test]
    fn term_match_3() {
        // Like with unification, there should be no substitution.
        // f(x) = g(x)
        let x = Term::new_variable(-1);
        let f_x = Term::new_function(1, vec!(x.clone()));
        let g_x = Term::new_function(2, vec!(x));
        assert!(term_match(&f_x, &g_x).is_none());
    }
} 

