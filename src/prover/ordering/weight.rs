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

use prover::data_structures::term::Term;

/// Defines a weighting function on terms.
/// Different enums represent different types of weightings.
#[derive(Debug)]
pub enum Weight {
    SimpleWeight,
}

impl Weight {
    /// Calculates the weight for t.
    pub fn weight(&self, only_unary_func: &Option<i64>, t: &Term) -> usize {        
        match *self {
            Weight::SimpleWeight => simple_weight(only_unary_func, t),
        }
    }
}

/// Variables have weight 1.
/// If there is exactly one unary function symbol in the problem, it has weight 0. All other function symbols have weight 1.
/// The weight function is extended to terms like so: weight(f(t1, ..., tn)) = weight(f) + weight(t1) + weight(...) + weight(tn).
fn simple_weight(only_unary_func: &Option<i64>, t: &Term) -> usize {
    if t.is_variable() {
        1
    } else {
        let func_symbol_weight = if Some(t.get_id()) == *only_unary_func {
                                     0
                                 } else {
                                     1
                                 };
        t.iter().fold(func_symbol_weight, |acc, s| acc + simple_weight(only_unary_func, s))
    }
}

#[cfg(test)]
mod test {

} 