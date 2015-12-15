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

use std::collections::hash_map::HashMap;
use prover::term::Term;
        
/// Checks if s is greater than t according to the ordering.    
pub fn kbo_gt(only_unary_func: &Option<i64>, s: &Term, t: &Term) -> bool {
    if s.is_function() && t.is_function() {
        let s_weight = weight(only_unary_func, s);
        let t_weight = weight(only_unary_func, t);
        if s_weight > t_weight {
            variable_domination(s, t)
        } else if s_weight == t_weight {
            if precedence(only_unary_func, s, t) || (s.get_id() == t.get_id() && lexical_ordering(only_unary_func, s, t)) {
                variable_domination(s, t)
            } else {
                false
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
pub fn kbo_ge(only_unary_func: &Option<i64>, s: &Term, t: &Term) -> bool {
    s == t || kbo_gt(only_unary_func, s, t)
}

fn variable_domination(s: &Term, t: &Term) -> bool {
    let mut variable_counts = HashMap::new();
    variable_count(&mut variable_counts, s, 1);
    variable_count(&mut variable_counts, t, -1);
    variable_counts.values().all(|&count| count >= 0)
}

fn variable_count(counts: &mut HashMap<i64, i64>, t: &Term, weight: i64) {
    if t.is_variable() {
        let v = counts.entry(t.get_id()).or_insert(0);
        *v += weight;
    } else {
        for x in t.iter() {
            variable_count(counts, x, weight);
        }
    }
}

fn lexical_ordering(only_unary_func: &Option<i64>, s: &Term, t: &Term) -> bool {
    assert_eq!(s.get_id(), t.get_id());
    assert_eq!(s.get_arity(), t.get_arity());
            
    for i in 0..s.get_arity() {
        if kbo_gt(only_unary_func, &s[i], &t[i]) {
            return true;
        } else if s[i] != t[i] {
            return false;
        }
    }
            
    false
}

/// Returns true if s is "heavier" than t.
/// Heavier means that it either has a larger arity or in the case that the arities are equal a larger id. 
/// One small exception: if there is exactly one unary function in the problem, that function is greater than all other functions.
fn precedence(only_unary_func: &Option<i64>, s: &Term, t: &Term) -> bool {
    if Some(s.get_id()) == *only_unary_func {
        return true;
    } else if Some(t.get_id()) == *only_unary_func {
        return false;
    }
        
    if s.get_arity() == t.get_arity()  {
        s.get_id() > t.get_id()
    } else {
        s.get_arity() > t.get_arity()
    }
}

/// Gives a weight to all terms.
/// Variables have weight 1.
/// If there is exactly one unary function in the problem, it has weight 0. All other symbols have weight 1.
/// The weight function is extended to terms like so: weight(f(t1, ..., tn)) = weight(f) + weight(t1) + weight(...) + weight(tn).
fn weight(only_unary_func: &Option<i64>, s: &Term) -> usize {
    if s.is_variable() {
        1
    } else {
        let func_symbol_weight = if Some(s.get_id()) == *only_unary_func {
                                     0
                                 } else {
                                     1
                                 };
        s.iter().fold(func_symbol_weight, |acc, t| acc + weight(only_unary_func, t))
    }
}

#[cfg(test)]
mod test {

} 