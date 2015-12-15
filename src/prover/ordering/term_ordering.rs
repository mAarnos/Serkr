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
use prover::literal::Literal;
use prover::ordering::lpo::{lpo_gt, lpo_ge};
use prover::ordering::kbo::{kbo_gt, kbo_ge};

/// A generic term ordering. Currently e have the option of using either LPO or KBO.
pub enum TermOrdering {
    LPO,
    KBO(Option<i64>)
}

impl TermOrdering {
    /// Returns true if term s is greater than term t according to the term ordering.
    pub fn gt(&self, s: &Term, t: &Term) -> bool {
        match *self {
            TermOrdering::LPO => lpo_gt(s, t),
            TermOrdering::KBO(only_unary_func) => kbo_gt(&only_unary_func, s, t)
        }
    }
    
    /// Returns true if s is greater than or equal to t according to the term ordering.
    #[allow(dead_code)]
    pub fn ge(&self, s: &Term, t: &Term) -> bool {
        match *self {
            TermOrdering::LPO => lpo_ge(s, t),
            TermOrdering::KBO(only_unary_func) => kbo_ge(&only_unary_func, s, t)
        }
    }
    
    /// Returns true if literal l1 is greater than literal l2 according to a multiset extension of the term ordering.
    pub fn gt_lit(&self, l1: &Literal, l2: &Literal) -> bool {
        // A term equal to another term can never be greater than it.
        if l1 == l2 {
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
                if self.gt(l1_term, l2_term) {
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
    
    /// Returns true if literal l1 is greater than or equal to literal l2 according to a multiset extension of the term ordering.
    pub fn ge_lit(&self, l1: &Literal, l2: &Literal) -> bool {
        l1 == l2 || self.gt_lit(l1, l2)
    }
}

/// Calculates the multiset difference of two literals.
/// We map s = t to { s, t } and s <> t to { s, s, t, t }
// TODO: improve efficiency
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
