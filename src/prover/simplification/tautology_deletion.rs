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

use prover::clause::Clause;

/// Checks if a clause is a syntactical tautology and as such can be eliminated completely.
pub fn trivial(cl: &Clause) -> bool {
    td1(cl) || td2(cl)
}

/// Checks if a clause contains a literal of the form "s = s".
/// Time complexity is O(n) where n is the amount of literals.
fn td1(cl: &Clause) -> bool {
    cl.iter().any(|l| l.is_positive() && l.get_lhs() == l.get_rhs())
}

/// Checks if a clause contains a literal and its negation.
/// Time complexity is O(n^2) where n is the amount of literals, but usually the clauses are rather short.
fn td2(cl: &Clause) -> bool {
    for (i, l1) in cl.iter().enumerate() {
        for l2 in cl.iter().skip(i + 1) {
            if !l1.polarity_equal(l2) && l1.terms_equal(l2) {
                return true;
            }
        }
    }  
    false
}

#[cfg(test)]
mod test {
    use super::{td1, td2};
    use prover::term::Term;
    use prover::literal::Literal;
    use prover::clause::Clause;
    
    #[test]
    fn td1_1() {
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let z = Term::new_variable(-3);
        let l1 = Literal::new(false, x.clone(), y.clone());
        let l2 = Literal::new(true, z.clone(), x.clone());
        let l3 = Literal::new(false, y.clone(), y.clone());
        let l4 = Literal::new(true, y, z);
        let cl = Clause::new(vec!(l1, l2, l3, l4));
        
        assert!(td1(&cl));
    }
    
    #[test]
    fn td1_2() {
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let z = Term::new_variable(-3);
        let l1 = Literal::new(true, x.clone(), z.clone());
        let l2 = Literal::new(true, z.clone(), x.clone());
        let l3 = Literal::new(false, y, z);
        let cl = Clause::new(vec!(l1, l2, l3));
        
        assert!(!td1(&cl));
    }
    
    #[test]
    fn td1_3() {
        let cl = Clause::new(Vec::new());       
        assert!(!td1(&cl));
    }
    
    #[test]
    fn td2_1() {
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let z = Term::new_variable(-3);
        let l1 = Literal::new(true, x.clone(), y.clone());
        let l2 = Literal::new(false, z, x.clone());
        let l3 = Literal::new(false, y, x);
        let cl = Clause::new(vec!(l1, l2, l3));
        
        assert!(td2(&cl));
    }
    
    #[test]
    fn td2_2() {
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let l1 = Literal::new(true, x.clone(), y.clone());
        let l2 = Literal::new(true, x, y);
        let cl = Clause::new(vec!(l1, l2));
        
        assert!(!td2(&cl));
    }
    
    #[test]
    fn td2_3() {
        let cl = Clause::new(Vec::new());       
        assert!(!td2(&cl));
    }
} 

