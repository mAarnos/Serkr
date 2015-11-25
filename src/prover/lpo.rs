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

use std::cmp::min;
use prover::term::Term;
use prover::literal::{Literal, terms_equal};

/// Returns true if literal l1 is greater than literal l2 according to a multiset extension of LPO.
pub fn lpo_gt_lit(l1: &Literal, l2: &Literal) -> bool {
    // An term equal to another term can never be greater than it.
    if terms_equal(l1, l2) && l1.is_negative() == l2.is_negative() {
        return false;
    }

    let l1_l2_diff = multiset_difference(l1, l2);
    let l2_l1_diff = multiset_difference(l2, l1);
    
    // l1 > l2 iff for each t2 of l2 with n(t2, l2) > n(t2, l1), there is a t of l1 so that t > t2 and n(t, l1) > n(t, l2).
    for (i, _) in l2_l1_diff.iter().filter(|&x| *x != 0).enumerate() {
        let mut dominating_term_found = false;
            
        for (j, _) in l1_l2_diff.iter().filter(|&x| *x != 0).enumerate() {
            let l1_term = if j == 0 { l1.get_lhs() } else { l1.get_rhs() };
            let l2_term = if i == 0 { l2.get_lhs() } else { l2.get_rhs() };
            if lpo_gt(l1_term, l2_term) {
                dominating_term_found = true;
                break;
            }
        }
            
        if !dominating_term_found {
            return false;
        }
    }
    
    true
}

/// Returns true if literal l1 is greater than or equal to literal l2 according to a multiset extension of LPO.
pub fn lpo_ge_lit(l1: &Literal, l2: &Literal) -> bool {
    l1 == l2 || lpo_gt_lit(l1, l2)
}

/// Calculates the multiset difference of two literals.
/// We map s = t to { s, t } and s <> t to { s, s, t, t }
fn multiset_difference(l: &Literal, r: &Literal) -> [usize; 2] {
    let mut l_count = [if l.is_negative() { 2 } else { 1 }, if l.is_negative() { 2 } else { 1 }];
    let mut r_count = [if r.is_negative() { 2 } else { 1 }, if r.is_negative() { 2 } else { 1 }];
    
    // Special handling of the case when lhs == rhs
    if l.get_lhs() == l.get_rhs() {
        l_count[0] += l_count[1];
        l_count[1] = 0;
    }
    
    // Same here.
    if r.get_lhs() == r.get_rhs() {
        r_count[0] += r_count[1];
        r_count[1] = 0;
    }
    
    if r.get_lhs() == l.get_lhs() {
        let min_diff = min(l_count[0], r_count[0]);
        l_count[0] -= min_diff;
        r_count[0] -= min_diff;
    }
    
    if r.get_lhs() == l.get_rhs() {
        let min_diff = min(l_count[1], r_count[0]);
        l_count[1] -= min_diff;
        r_count[0] -= min_diff;
    }
    
    if r.get_rhs() == l.get_lhs() {
        let min_diff = min(l_count[0], r_count[1]);
        l_count[0] -= min_diff;
        r_count[1] -= min_diff;
    }
    
    if r.get_rhs() == l.get_rhs() {
        let min_diff = min(l_count[1], r_count[1]);
        l_count[1] -= min_diff;
        r_count[1] -= min_diff;
    }
    
    l_count
}

/// Returns true if term t is greater than term s according to a LPO.
pub fn lpo_gt(s: &Term, t: &Term) -> bool {
    if s.is_function() && t.is_function() {
        if s.iter().any(|arg| lpo_ge(arg, t)) {
            true
        } else if t.iter().all(|arg| lpo_gt(s, arg)) {
            if s.get_id() == t.get_id() && lexical_ordering(s, t) {
                true
            } else {
                weight(s, t)
            }    
        } else {
            false
        }
    } else if t.is_variable() {
        s != t && s.occurs_proper(t)
    } else {
        false
    }
}

/// Returns true if t is greater than or equal to s according to a LPO.
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
fn weight(s: &Term, t: &Term) -> bool {
    if s.get_arity() == t.get_arity()  {
        s.get_id() > t.get_id()
    } else {
        s.get_arity() > t.get_arity()
    }
}

#[cfg(test)]
mod test {
    use super::{lpo_gt, lpo_ge, lpo_gt_lit, lpo_ge_lit};    
    use prover::term::Term;
    use prover::literal::Literal;
    
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
    
    #[test]
    fn lpo_ge_lit_1() {
        let x = Term::new(-1, false, Vec::new());
        let f_g_x = Term::new(1, false, vec!(Term::new(2, false, vec!(x))));  
        let l = Literal::new(false, f_g_x, Term::new_truth());
        assert!(!lpo_gt_lit(&l, &l));
        assert!(lpo_ge_lit(&l, &l));
    }
} 