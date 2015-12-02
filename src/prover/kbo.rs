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
use prover::term_ordering::TermOrdering;

/// An implementation of the Knuth-Bendix ordering.
pub struct KBO {
    one_unary_function: bool,
    unary_function_id: i64
}

impl KBO {   
    /// Creates a new KBO term ordering.
    #[allow(dead_code)]
    pub fn new(problem_has_one_unary_function: bool, unary_function_id: i64) -> KBO {
        KBO { one_unary_function: problem_has_one_unary_function, unary_function_id: unary_function_id }
    }

    fn lexical_ordering(&self, s: &Term, t: &Term) -> bool {
        assert_eq!(s.get_id(), t.get_id());
        assert_eq!(s.get_arity(), t.get_arity());
            
        for i in 0..s.get_arity() {
            if self.gt(&s[i], &t[i]) {
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
    fn precedence(&self, s: &Term, t: &Term) -> bool {
        if self.one_unary_function {
            if self.unary_function_id == s.get_id() {
                return true;
            } else if self.unary_function_id == t.get_id() {
                return false;
            }
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
    fn weight(&self, s: &Term) -> usize {
        if s.is_variable() {
            1
        } else {
            let func_symbol_weight = if self.one_unary_function && self.unary_function_id == s.get_id() {
                                         0
                                     } else {
                                         1
                                     };
            s.iter().fold(func_symbol_weight, |acc, t| acc + t.symbol_count())
        }
    }
}

impl TermOrdering for KBO {
    fn gt(&self, s: &Term, t: &Term) -> bool {
        if s.is_function() && t.is_function() {
            let s_weight = self.weight(s);
            let t_weight = self.weight(t);
            if s_weight > t_weight {
                unimplemented!();
            } else if s_weight == t_weight {
                if self.precedence(s, t) || (s.get_id() == t.get_id() && self.lexical_ordering(s, t)) {
                    unimplemented!();
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
    
    fn ge(&self, s: &Term, t: &Term) -> bool {
        s == t || self.gt(s, t)
    }
}

#[cfg(test)]
mod test {

} 