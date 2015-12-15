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

/// Checks if s is greater than t according to the ordering.   
pub fn lpo_gt(s: &Term, t: &Term) -> bool {
    if s.is_function() && t.is_function() {
        if s.iter().any(|arg| lpo_ge(arg, t)) {
            true
        } else if t.iter().all(|arg| lpo_gt(s, arg)) {
            if s.get_id() == t.get_id() && lexical_ordering(s, t) {
                true
            } else {
                precedence(s, t)
            }    
        } else {
            false
        }
    } else if s.is_function() && t.is_variable() {
        s.occurs_proper(t)
    } else {
        false
    }
}

/// Checks if s is greater than or equal to t according to the ordering.        
pub fn lpo_ge(s: &Term, t: &Term) -> bool {
    s == t || lpo_gt(s, t)
}

fn lexical_ordering(s: &Term, t: &Term) -> bool {
    assert_eq!(s.get_id(), t.get_id());
    assert_eq!(s.get_arity(), t.get_arity());
            
    for i in 0..s.get_arity() {
        if lpo_gt(&s[i], &t[i]) {
            return true;
        } else if s[i] != t[i] {
            return false;
        }
    }
            
    false
}

/// Returns true if s is "heavier" than t.
/// Heavier means that it either has a larger arity or in the case that the arities are equal a larger id. 
fn precedence(s: &Term, t: &Term) -> bool {
    if s.get_arity() == t.get_arity()  {
        s.get_id() > t.get_id()
    } else {
        s.get_arity() > t.get_arity()
    }
}

#[cfg(test)]
mod test {
    use super::{lpo_gt, lpo_ge};    
    use prover::term::Term;
    
    #[test]
    fn lpo_gt_1() {
        let x = Term::new(-1, false, Vec::new());
        let y = Term::new(-2, false, Vec::new());       
        assert!(!lpo_gt(&x, &y));
        assert!(!lpo_gt(&y, &x));
    }
    
    #[test]
    fn lpo_gt_2() {
        let x = Term::new(-1, false, Vec::new());
        let f_x = Term::new(1, false, vec!(x.clone()));       
        assert!(lpo_gt(&f_x, &x));
        assert!(!lpo_gt(&x, &f_x));
    }
    
    #[test]
    fn lpo_gt_3() {
        let x = Term::new(-1, false, Vec::new());
        let y = Term::new(-2, false, Vec::new());
        let f_y = Term::new(1, false, vec!(y));       
        assert!(!lpo_gt(&f_y, &x));
        assert!(!lpo_gt(&x, &f_y));
    }
    
    #[test]
    fn lpo_gt_4() {
        let x = Term::new(-1, false, Vec::new());
        let f_x = Term::new(1, false, vec!(x.clone()));       
        let f_f_x = Term::new(1, false, vec!(f_x.clone()));   
        assert!(lpo_gt(&f_f_x, &f_x));
        assert!(!lpo_gt(&f_x, &f_f_x));
        assert!(lpo_gt(&f_f_x, &x));
        assert!(!lpo_gt(&x, &f_f_x));
    }
    
    #[test]
    fn lpo_gt_5() {
        let x = Term::new(-1, false, Vec::new());
        let f_g_x = Term::new(1, false, vec!(Term::new(2, false, vec!(x.clone()))));         
        assert!(lpo_gt(&f_g_x, &x));
        assert!(!lpo_gt(&x, &f_g_x));
    }
    
    #[test]
    fn lpo_gt_6() {
        let x = Term::new(-1, false, Vec::new());
        let f_x_x = Term::new(1, false, vec!(x.clone(), x)); 
        let t = Term::new_truth();
        assert!(lpo_gt(&f_x_x, &t));
        assert!(!lpo_gt(&t, &f_x_x));
    }
    
    #[test]
    fn lpo_ge_1() {
        let x = Term::new(-1, false, Vec::new());
        let f_x = Term::new(1, false, vec!(x));         
        assert!(!lpo_gt(&f_x, &f_x));
        assert!(lpo_ge(&f_x, &f_x));
    }
    
    /*
    #[test]
    fn lpo_ge_lit_1() {
        let x = Term::new(-1, false, Vec::new());
        let f_g_x = Term::new(1, false, vec!(Term::new(2, false, vec!(x))));  
        let l = Literal::new(false, f_g_x, Term::new_truth());
        assert!(!lpo_gt_lit(&l, &l));
        assert!(lpo_ge_lit(&l, &l));
    }
    */
} 