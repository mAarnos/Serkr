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

/// Deletes all duplicated literals from a clause.
/// Time complexity is O(n^2) where n is the amount of literals, but usually the clauses are rather short.
// TODO: see how much time is spent here.
pub fn delete_duplicates(cl: &mut Clause) {
    let mut i = 0;
    while i < cl.size() {
        let mut j = i + 1;
        while j < cl.size() {
            if cl[i] == cl[j] {
                cl.swap_remove(j);
                continue;
            }
            j += 1;
        }
        i += 1;
    }
}

#[cfg(test)]
mod test {

} 

