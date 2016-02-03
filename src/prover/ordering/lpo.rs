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
use prover::ordering::precedence::Precedence;

/// Checks if s is greater than t according to the ordering.   
pub fn lpo_gt(precedence: &Precedence, s: &Term, t: &Term) -> bool {
    if s.is_function() && t.is_function() {
        if s.iter().any(|arg| lpo_ge(precedence, arg, t)) {
            true
        } else if t.iter().all(|arg| lpo_gt(precedence, s, arg)) {
            if s.get_id() == t.get_id() && lexical_ordering(precedence, s, t) {
                true
            } else {
                precedence.gt(s, t)
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
pub fn lpo_ge(precedence: &Precedence, s: &Term, t: &Term) -> bool {
    s == t || lpo_gt(precedence, s, t)
}

fn lexical_ordering(precedence: &Precedence, s: &Term, t: &Term) -> bool {
    assert_eq!(s.get_id(), t.get_id());
    assert_eq!(s.get_arity(), t.get_arity());
            
    for i in 0..s.get_arity() {
        if lpo_gt(precedence, &s[i], &t[i]) {
            return true;
        } else if s[i] != t[i] {
            return false;
        }
    }
            
    false
}

#[cfg(test)]
mod test {
    use super::{lpo_gt, lpo_ge};    
    use prover::term::Term;
    use prover::ordering::precedence::Precedence;
    
    #[test]
    fn lpo_gt_1() {
        let precedence = Precedence::default();
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);       
        assert!(!lpo_gt(&precedence, &x, &y));
        assert!(!lpo_gt(&precedence, &y, &x));
    }
    
    #[test]
    fn lpo_gt_2() {
        let precedence = Precedence::default();
        let x = Term::new_variable(-1);
        let f_x = Term::new_function(1, vec!(x.clone()));       
        assert!(lpo_gt(&precedence, &f_x, &x));
        assert!(!lpo_gt(&precedence, &x, &f_x));
    }
    
    #[test]
    fn lpo_gt_3() {
        let precedence = Precedence::default();
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let f_y = Term::new_function(1, vec!(y));       
        assert!(!lpo_gt(&precedence, &f_y, &x));
        assert!(!lpo_gt(&precedence, &x, &f_y));
    }
    
    #[test]
    fn lpo_gt_4() {
        let precedence = Precedence::default();
        let x = Term::new_variable(-1);
        let f_x = Term::new_function(1, vec!(x.clone()));       
        let f_f_x = Term::new_function(1, vec!(f_x.clone()));   
        assert!(lpo_gt(&precedence, &f_f_x, &f_x));
        assert!(!lpo_gt(&precedence, &f_x, &f_f_x));
        assert!(lpo_gt(&precedence, &f_f_x, &x));
        assert!(!lpo_gt(&precedence, &x, &f_f_x));
    }
    
    #[test]
    fn lpo_gt_5() {
        let precedence = Precedence::default();
        let x = Term::new_variable(-1);
        let f_g_x = Term::new_function(1, vec!(Term::new_function(2, vec!(x.clone()))));         
        assert!(lpo_gt(&precedence, &f_g_x, &x));
        assert!(!lpo_gt(&precedence, &x, &f_g_x));
    }
    
    #[test]
    fn lpo_gt_6() {
        let precedence = Precedence::default();
        let x = Term::new_variable(-1);
        let f_x_x = Term::new_function(1, vec!(x.clone(), x)); 
        let t = Term::new_truth();
        assert!(lpo_gt(&precedence, &f_x_x, &t));
        assert!(!lpo_gt(&precedence, &t, &f_x_x));
    }
    
    #[test]
    fn lpo_ge_1() {
        let precedence = Precedence::default();
        let x = Term::new_variable(-1);
        let f_x = Term::new_function(1, vec!(x));         
        assert!(!lpo_gt(&precedence, &f_x, &f_x));
        assert!(lpo_ge(&precedence, &f_x, &f_x));
    }
    
    /*
    #[test]
    fn lpo_ge_lit_1() {
        let x = Term::new_variable(-1);
        let f_g_x = Term::new(1, false, vec!(Term::new(2, false, vec!(x))));  
        let l = Literal::new(false, f_g_x, Term::new_truth());
        assert!(!lpo_gt_lit(&l, &l));
        assert!(lpo_ge_lit(&l, &l));
    }
    */
} 