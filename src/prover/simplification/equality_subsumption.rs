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
use prover::data_structures::literal::Literal;
use prover::data_structures::clause::Clause;
use prover::unification::matching::term_match_with_subst;
use prover::data_structures::pd_tree::PDTree;

/// Check if the term index contains a matching literal without taking into account symmetricity.
fn matching_equation_exists_asymmetric(term_index: &PDTree, s: &Term, t: &Term, pos: bool) -> bool {
    for (_, r, sigma) in term_index.iter_generalizations(s, pos) {
        if term_match_with_subst(sigma, r, t).is_some() {
            return true;
        }
    }
    
    false
}

// TODO: figure out a better name
pub fn matching_equation_exists(term_index: &PDTree, s: &Term, t: &Term, pos: bool) -> bool {
    matching_equation_exists_asymmetric(term_index, s, t, pos) ||
    matching_equation_exists_asymmetric(term_index, t, s, pos)
}

/// Checks if we can make s = t by using some unit clause in the term index.
// TODO: remove pub, rename into something more suited.
pub fn equation_subsumed(term_index: &PDTree, s: &Term, t: &Term) -> bool {
    if s == t || matching_equation_exists(term_index, s, t, true) {
        true
    } else if s.is_function() && s.get_id() == t.get_id() {
        assert_eq!(s.get_arity(), t.get_arity());

        // Only one different location is allowed.
        let mut diff_index = None;
        for (i, s_i) in s.iter().enumerate() {
            if *s_i != t[i] {
                if diff_index.is_some() {
                    return false;
                }
                diff_index = Some(i);
            }
        }
        
        if let Some(i) = diff_index {
            equation_subsumed(term_index, &s[i], &t[i])
        } else {
            panic!("At least one location should be different");
        }
    } else {
        false
    }
}

/// Checks if a given literal is equality subsumed by some unit clause in the term index.
fn literal_subsumed(term_index: &PDTree, l: &Literal) -> bool {
    if l.is_positive() {
        equation_subsumed(term_index, l.get_lhs(), l.get_rhs())
    } else {
        // Positive simplify-reflect could be done here
        false
    }
}

/// Checks if a given clause is equality subsumed by some unit clause in the term index.
pub fn forward_equality_subsumed(term_index: &PDTree, cl: &Clause) -> bool {
    cl.iter().any(|l| literal_subsumed(term_index, l))
}

#[cfg(out_of_order)]
mod test {
    use super::equality_subsumes_clause;
    use prover::data_structures::term::Term;
    use prover::data_structures::literal::Literal;
    use prover::data_structures::clause::Clause;

    #[test]
    fn equality_subsumes_clause_1() {
        // Check that f(x) = x subsumes g(f(a), f(b)) = g(a, b).
        let x = Term::new_variable(-1);
        let a = Term::new_function(1, Vec::new());
        let b = Term::new_function(2, Vec::new());
        let g_f_a_f_b = Term::new_function(4,
                                           vec![Term::new_function(3, vec![a.clone()]),
                                                Term::new_function(3, vec![b.clone()])]);
        let g_a_b = Term::new_function(4, vec![a.clone(), b.clone()]);

        let cl1 = Clause::new(vec![Literal::new(false, Term::new_function(3, vec![x.clone()]), x)]);
        let cl2 = Clause::new(vec![Literal::new(false, g_f_a_f_b, g_a_b)]);

        assert!(equality_subsumes_clause(&cl1, &cl2));
    }

    #[test]
    fn equality_subsumes_clause_2() {
        // Check that f(x, c) = x subsumes g(y) = g(f(y, c)).
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let c = Term::new_function(1, Vec::new());
        let f_x_c = Term::new_function(2, vec![x.clone(), c.clone()]);
        let g_y = Term::new_function(3, vec![y.clone()]);
        let g_f_y_c = Term::new_function(3, vec![Term::new_function(2, vec![y, c])]);

        let cl1 = Clause::new(vec![Literal::new(false, f_x_c, x)]);
        let cl2 = Clause::new(vec![Literal::new(false, g_y, g_f_y_c)]);

        assert!(equality_subsumes_clause(&cl1, &cl2));
    }

    #[test]
    fn equality_subsumes_clause_3() {
        // Check that f(x) = T subsumes T = f(x).
        let x = Term::new_variable(-1);
        let f_x = Term::new_function(1, vec![x]);

        let cl1 = Clause::new(vec![Literal::new(false, f_x.clone(), Term::new_truth())]);
        let cl2 = Clause::new(vec![Literal::new(false, Term::new_truth(), f_x.clone())]);

        assert!(equality_subsumes_clause(&cl1, &cl2));
    }
}
