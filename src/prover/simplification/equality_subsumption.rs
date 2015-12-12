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
use prover::literal::Literal;
use prover::clause::Clause;
use prover::simplification::matching::match_term_pairs;

/// Checks if the equation s = t equality subsumes u = v.
// TODO: remove pub, rename into something more suited.
pub fn eqn_subsumes_eqn(s: &Term, t: &Term, u: &Term, v: &Term) -> bool {
    if u == v {
        true
    } else if match_term_pairs(s, t, u, v) {
        true
    } else {
        if u.is_function() && u.get_id() == v.get_id() {
            assert_eq!(u.get_arity(), v.get_arity());
            
            for i in 0..u.get_arity() {
                if !eqn_subsumes_eqn(s, t, &u[i], &v[i]) {
                    return false;
                }
            }
            
            true
        } else {
            false
        }
    }
}

/// Checks if the equation s = t equality subsumes a given literal.
fn eqn_subsumes_literal(s: &Term, t: &Term, l: &Literal) -> bool {
    if l.is_positive() {
        eqn_subsumes_eqn(s, t, l.get_lhs(), l.get_rhs())
    } else {
        false
    }
}

/// Checks if the clause cl1 equality subsumes the clause cl2.
pub fn equality_subsumes_clause(cl1: &Clause, cl2: &Clause) -> bool {
    if cl1.size() == 1 && cl1[0].is_positive() {
        cl2.iter().any(|l| eqn_subsumes_literal(cl1[0].get_lhs(), cl1[0].get_rhs(), l))
    } else {
        false
    }
}

#[cfg(test)]
mod test {
    use super::equality_subsumes_clause;
    use prover::term::Term;
    use prover::literal::Literal;
    use prover::clause::Clause;
    
    #[test]
    fn equality_subsumes_clause_1() {
        // Check that f(x) = x subsumes g(f(a), f(b)) = g(a,b). 
        let x = Term::new(-1, false, Vec::new());
        let a = Term::new(1, false, Vec::new());
        let b = Term::new(2, false, Vec::new());  
        let g_f_a_f_b = Term::new(4, false, vec!(Term::new(3, false, vec!(a.clone())), Term::new(3, false, vec!(b.clone()))));
        let g_a_b = Term::new(4, false, vec!(a.clone(), b.clone()));
        
        let cl1 = Clause::new(vec!(Literal::new(false, Term::new(3, false, vec!(x.clone())), x)));
        let cl2 = Clause::new(vec!(Literal::new(false, g_f_a_f_b, g_a_b)));
    
        assert!(equality_subsumes_clause(&cl1, &cl2));
    }
    
    #[test]
    fn equality_subsumes_clause_2() {
        // Check that f(x, c) = x subsumes g(y) = g(f(y, c)). 
        let x = Term::new(-1, false, Vec::new());
        let y = Term::new(-2, false, Vec::new());
        let c = Term::new(1, false, Vec::new());
        let f_x_c = Term::new(2, false, vec!(x.clone(), c.clone()));
        let g_y = Term::new(3, false, vec!(y.clone()));
        let g_f_y_c = Term::new(3, false, vec!(Term::new(2, false, vec!(y, c))));
        
        let cl1 = Clause::new(vec!(Literal::new(false, f_x_c, x)));
        let cl2 = Clause::new(vec!(Literal::new(false, g_y, g_f_y_c)));
    
        assert!(equality_subsumes_clause(&cl1, &cl2));
    }
    
    #[test]
    fn equality_subsumes_clause_3() {
        // Check that f(x) = T subsumes T = f(x). 
        let x = Term::new(-1, false, Vec::new());
        let f_x = Term::new(1, true, vec!(x));
        
        let cl1 = Clause::new(vec!(Literal::new(false, f_x.clone(), Term::new_truth())));
        let cl2 = Clause::new(vec!(Literal::new(false, Term::new_truth(), f_x.clone())));
    
        assert!(equality_subsumes_clause(&cl1, &cl2));
    }
} 
