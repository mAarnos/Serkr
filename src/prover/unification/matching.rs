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

use prover::data_structures::term::Term;
use prover::unification::substitution::Substitution;

/// Tries to find a sigma so that s\sigma = t.
/// Returns the substitution if it exists.
/// Note that we treat variables of a pair of terms as distinct even if they have the same name.
/// This might cause us to have trivial mappings like x |-> x.
#[allow(dead_code)]
pub fn term_match(s: &Term, t: &Term) -> Option<Substitution> {
    term_match_general(Substitution::new(), vec![(s.clone(), t.clone())])
}

/// A version of term_match where we extend a given substitution.
pub fn term_match_with_subst(sigma: Substitution, s: &Term, t: &Term) -> Option<Substitution> {
    term_match_general(sigma, vec![(s.clone(), t.clone())])
}

/// A more general version of term_match.
/// We can pass in a substitution to expand.
/// We can also give more pairs of equations than just one.
pub fn term_match_general(mut substitution: Substitution,
                          mut eqs: Vec<(Term, Term)>)
                          -> Option<Substitution> {
    while let Some((s, t)) = eqs.pop() {
        if s.is_function() && t.is_function() {
            if s.get_id() == t.get_id() {
                eqs.extend(s.into_iter().zip(t.into_iter()));
            } else {
                return None;
            }
        } else if s.is_variable() {
            // Can't unify between two different sorts.
            if t.is_special_function() {
                return None;
            }
            
            // Check if there is a previous bind.
            if let Some(v) = substitution.get(&s) {
                // Check that it is not different.
                if *v != t {
                    return None;
                }
                continue;
            }
            // There wasn't, just bind.
            substitution.insert(s, t);
        } else {
            return None;
        }
    }

    Some(substitution)
}

#[cfg(test)]
mod test {
    use super::term_match;
    use prover::data_structures::term::Term;

    #[test]
    fn term_match_1() {
        // f(x) = f(g(x))
        // Unlike with unification, this should succeed.
        let x = Term::new_variable(-1);
        let f_x = Term::new_function(1, vec![x.clone()]);
        let f_g_x = Term::new_function(1, vec![Term::new_function(2, vec![x])]);
        assert!(term_match(&f_x, &f_g_x).is_some());
    }

    #[test]
    fn term_match_2() {
        // f(g(y)) = f(x)
        // With unification this would be trivial, but with matching there is no substitution.
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let f_g_y = Term::new_function(1, vec![Term::new_function(2, vec![y])]);
        let f_x = Term::new_function(1, vec![x]);
        assert!(term_match(&f_g_y, &f_x).is_none());
    }

    #[test]
    fn term_match_3() {
        // Like with unification, there should be no substitution.
        // f(x) = g(x)
        let x = Term::new_variable(-1);
        let f_x = Term::new_function(1, vec![x.clone()]);
        let g_x = Term::new_function(2, vec![x]);
        assert!(term_match(&f_x, &g_x).is_none());
    }
}
