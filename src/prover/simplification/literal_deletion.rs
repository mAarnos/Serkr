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
use prover::unification::full_unification::mgu;

/// Simplifies a clause with cheap (i.e. fast to run) rules if possible.
pub fn cheap_simplify(cl: &mut Clause) {
    destructive_equality_resolution(cl);
    delete_resolved(cl);
    delete_duplicates(cl);
}

/// Deletes all duplicated literals from a clause.
/// Time complexity is O(n^2) where n is the amount of literals.
pub fn delete_duplicates(cl: &mut Clause) {
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
pub fn delete_resolved(cl: &mut Clause) {
    let mut i = 0;
    while i < cl.size() {
        if cl[i].is_negative() && cl[i].get_lhs() == cl[i].get_rhs() {
            cl.swap_remove(i);
            continue;
        }
        i += 1;
    }
}

/// A special case of equality resolution which is applied only to variables.
/// It is rather easy to see that we can use this for simplifying instead of generation.
/// Time complexity is probably O(n^2) where n is the amount of literals.
// TODO: possibly use also with x = f(...) where x does not occur in f(...). Probably complete.
pub fn destructive_equality_resolution(cl: &mut Clause) {
    let mut i = 0;
    while i < cl.size() {
        if cl[i].is_negative() && cl[i].get_lhs().is_variable() && cl[i].get_rhs().is_variable() {
            if let Some(sigma) = mgu(cl[i].get_lhs(), cl[i].get_rhs()) {
                cl.swap_remove(i);
                cl.subst(&sigma);
                // Since we always remove one literal from the clause the recursion stops at some point.
                destructive_equality_resolution(cl);
                break;
            }
        }
        i += 1;
    }
}

#[cfg(test)]
mod test {
    use super::{delete_resolved, delete_duplicates};
    use prover::term::Term;
    use prover::literal::Literal;
    use prover::clause::Clause;
    
    #[test]
    fn delete_resolved_1() {
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let z = Term::new_variable(-3);
        let l1 = Literal::new(false, x.clone(), y.clone());
        let l2 = Literal::new(true, z.clone(), x.clone());
        let l3 = Literal::new(true, y.clone(), y.clone());
        let l4 = Literal::new(false, x, z);
        let mut cl = Clause::new(vec!(l1.clone(), l2.clone(), l3.clone(), l4.clone()));
        
        delete_resolved(&mut cl);
        assert_eq!(cl.size(), 3);
        assert!(cl.iter().any(|l| *l == l1));
        assert!(cl.iter().any(|l| *l == l2));
        assert!(cl.iter().any(|l| *l == l4));
    }
    
    #[test]
    fn delete_resolved_2() {
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let z = Term::new_variable(-3);
        let l1 = Literal::new(false, x.clone(), y);
        let l2 = Literal::new(true, z, x);
        let mut cl = Clause::new(vec!(l1, l2));
        
        let cl_copy = cl.clone();     
        delete_resolved(&mut cl);
        assert_eq!(cl, cl_copy);
    }
    
    #[test]
    fn delete_resolved_3() {
        let x = Term::new_variable(-1);
        let l = Literal::new(true, x.clone(), x);
        let mut cl = Clause::new(vec!(l.clone(), l.clone(), l));
          
        delete_resolved(&mut cl);
        assert!(cl.is_empty());
    }
    
    #[test]
    fn delete_duplicates_1() {
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let z = Term::new_variable(-3);
        let l1 = Literal::new(true, x.clone(), y.clone());
        let l2 = Literal::new(false, z.clone(), x.clone());
        let l3 = Literal::new(true, y, x);
        let mut cl = Clause::new(vec!(l1.clone(), l2.clone(), l3.clone()));

        delete_duplicates(&mut cl);
        
        assert_eq!(cl.size(), 2);
        assert!(cl.iter().any(|l| *l == l1));
        assert!(cl.iter().any(|l| *l == l2));
    }
    
    #[test]
    fn delete_duplicates_2() {
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let z = Term::new_variable(-3);
        let w = Term::new_variable(-4);
        let l1 = Literal::new(true, x.clone(), y.clone());
        let l2 = Literal::new(true, z.clone(), w.clone());
        let l3 = Literal::new(false, w.clone(), x.clone());
        let l4 = Literal::new(false, y, z);
        let mut cl = Clause::new(vec!(l1, l2, l3, l4));
        let cl_copy = cl.clone();
        
        delete_duplicates(&mut cl);
        assert!(cl == cl_copy);
    }
    
    #[test]
    fn delete_duplicates_3() {
        let x = Term::new_variable(-1);
        let l = Literal::new(false, x.clone(), x);
        let mut cl = Clause::new(vec!(l.clone(), l.clone(), l.clone()));
          
        delete_duplicates(&mut cl);
        assert_eq!(cl.size(), 1);
        assert!(cl[0] == l);
    }
} 

