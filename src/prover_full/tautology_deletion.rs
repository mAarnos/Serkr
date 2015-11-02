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

/// Checks if a clause is a syntactical tautology.
pub fn trivial(cl: &Clause) -> bool {
    td1(cl) || td2(cl)
}

/// Checks if a clause contains an equation of the form "s = s".
/// Time complexity is O(n) where n is the amount of equations.
fn td1(cl: &Clause) -> bool {
    cl.iter().any(|eqn| eqn.is_positive() && eqn.get_lhs() == eqn.get_rhs())
}

/// Checks if a clause contains an equation and its negation.
/// Time complexity is O(n^2) where n is the amount of equations, but usually the clauses are rather short.
// TODO: see how much time is spent here.
fn td2(cl: &Clause) -> bool {
    for (i, eqn1) in cl.iter().enumerate() {
        for eqn2 in cl.iter().skip(i + 1) {
            if eqn1.is_positive() != eqn2.is_positive() {
                if (eqn1.get_lhs() == eqn2.get_lhs() && eqn1.get_rhs() == eqn2.get_rhs()) ||
                   (eqn1.get_lhs() == eqn2.get_rhs() && eqn1.get_rhs() == eqn2.get_lhs()) {
                    return true;
                }
            }
        }
    }  
    false
}

#[cfg(test)]
mod test {
    // remember to have a test which tests x = y and y = x
} 

