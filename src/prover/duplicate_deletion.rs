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

use prover::clause::Clause;

/// Deletes all duplicated literals from a clause.
/// Running time is O(n^2) where n is the amount of literal in a clause.
/// O(n) is possible but that would probably be slower unless clauses are big.
pub fn delete_duplicates(cl: &mut Clause) {
    let mut i = 0;
    while i < cl.size() {
        let mut j = i + 1;
        while j < cl.size() {
            if cl.at(i) == cl.at(j) {
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
    use super::delete_duplicates;
    use prover::clause::Clause;
    use prover::literal::Literal;
    
    #[test]
    fn delete_duplicates_1() {
        let p = Literal::new_from_id(-1);
        let q = Literal::new_from_id(2);
        let r = Literal::new_from_id(3);
        let s = Literal::new_from_id(-4);
        let mut cl = Clause::new_from_vec(vec!(p.clone(), q, r.clone(), s, p, r));
        delete_duplicates(&mut cl);
        assert_eq!(cl.size(), 4);
    }
    
    #[test]
    fn delete_duplicates_2() {
        let p = Literal::new_from_id(1);
        let q = Literal::new_from_id(-2);
        let r = Literal::new_from_id(3);
        let s = Literal::new_from_id(4);
        let mut cl = Clause::new_from_vec(vec!(p, q, r, s));
        let correct_cl = cl.clone();
        delete_duplicates(&mut cl);
        assert_eq!(cl, correct_cl);
    }
    
    #[test]
    fn delete_duplicates_3() {
        let p = Literal::new_from_id(-1);
        let mut cl = Clause::new_from_vec(vec!(p.clone(), p.clone(), p));
        delete_duplicates(&mut cl);
        assert_eq!(cl.size(), 1);
    }
} 

