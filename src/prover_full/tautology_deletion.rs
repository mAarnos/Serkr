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

use prover_full::clause::Clause;
use prover_full::literal::terms_equal;

/// Checks if a clause is a syntactical tautology and as such can be eliminated completely.
pub fn trivial(cl: &Clause) -> bool {
    td1(cl) || td2(cl)
}

/// Checks if a clause contains a literal of the form "s = s".
/// Time complexity is O(n) where n is the amount of literals.
fn td1(cl: &Clause) -> bool {
    cl.iter().any(|l| l.is_positive() && l.get_lhs() == l.get_rhs())
}

/// Checks if a clause contains a literal and its negation.
/// Time complexity is O(n^2) where n is the amount of literals, but usually the clauses are rather short.
// TODO: see how much time is spent here.
fn td2(cl: &Clause) -> bool {
    for (i, l1) in cl.iter().enumerate() {
        for l2 in cl.iter().skip(i + 1) {
            if l1.is_positive() != l2.is_positive() && terms_equal(l1, l2) {
                return true;
            }
        }
    }  
    false
}

#[cfg(test)]
mod test {
    // remember to have a test which tests x = y and y = x
} 

