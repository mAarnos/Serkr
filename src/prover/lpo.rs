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

/// Returns true if t is greater than s according to a LPO.
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

} 