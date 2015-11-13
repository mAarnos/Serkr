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

/// Simplifies a clause if possible.
pub fn simplify(cl: &mut Clause) {
    delete_resolved(cl);
    delete_duplicates(cl);
}

/// Deletes all duplicated literals from a clause.
/// Time complexity is O(n^2) where n is the amount of literals, but usually the clauses are rather short.
// TODO: see how much time is spent here.
fn delete_duplicates(cl: &mut Clause) {
    let mut i = 0;
    while i < cl.size() {
        let mut j = i + 1;
        while j < cl.size() {
            if cl[i] == cl[j] {
                cl.swap_remove(j);
                continue;
            }
            j += 1;
        }
        i += 1;
    }
}

/// Deletes all resolved literals (s <> s) from a clause.
/// Time complexity is O(n) where n is the amount of literals.
fn delete_resolved(cl: &mut Clause) {
    let mut i = 0;
    while i < cl.size() {
        if !cl[i].is_positive() && cl[i].get_lhs() == cl[i].get_rhs() {
            cl.swap_remove(i);
            continue;
        }
        i += 1;
    }
}

#[cfg(test)]
mod test {
    use super::delete_resolved;
    use prover::term::Term;
    use prover::literal::Literal;
    use prover::clause::Clause;
    
    #[test]
    fn delete_resolved_1() {
        let x = Term::new(-1, false, Vec::new());
        let y = Term::new(-2, false, Vec::new());
        let z = Term::new(-3, false, Vec::new());
        let l1 = Literal::new(false, x.clone(), y.clone());
        let l2 = Literal::new(true, z.clone(), x.clone());
        let l3 = Literal::new(true, y.clone(), y.clone());
        let l4 = Literal::new(false, x, z);
        let mut cl = Clause::new(vec!(l1.clone(), l2.clone(), l3.clone(), l4.clone()));
        
        delete_resolved(&mut cl);
        assert!(cl.iter().any(|l| *l == l1));
        assert!(cl.iter().any(|l| *l == l2));
        assert!(!cl.iter().any(|l| *l == l3));
        assert!(cl.iter().any(|l| *l == l4));
    }
    
    #[test]
    fn delete_resolved_2() {
        let x = Term::new(-1, false, Vec::new());
        let y = Term::new(-2, false, Vec::new());
        let z = Term::new(-3, false, Vec::new());
        let l1 = Literal::new(false, x.clone(), y);
        let l2 = Literal::new(true, z, x);
        let mut cl = Clause::new(vec!(l1, l2));
        
        let cl_copy = cl.clone();     
        delete_resolved(&mut cl);
        assert_eq!(cl, cl_copy);
    }
    
    #[test]
    fn delete_resolved_3() {
        let x = Term::new(-1, false, Vec::new());
        let l = Literal::new(true, x.clone(), x);
        let mut cl = Clause::new(vec!(l.clone(), l.clone(), l));
          
        delete_resolved(&mut cl);
        assert_eq!(cl.size(), 0);
    }
} 

