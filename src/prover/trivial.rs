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

/// Checks if a clause is a syntactical tautology.
/// Running time is O(n^2) where n is the amount of literals in the clause.
/// O(n) is possible but that would probably be slower unless the clause is big.
pub fn trivial(cl: &Clause) -> bool {
    for (i, lit) in cl.iter().enumerate() {
        let negated_lit = lit.negate();
        for l in cl.iter().skip(i + 1) {
            if *l == negated_lit {
                return true;
            }
        }
    }  
    false
}

#[cfg(test)]
mod test {
    use super::trivial;
    use prover::clause::Clause;
    use prover::literal::Literal;
    
    #[test]
    fn trivial_1() {
        let p = Literal::new_from_id(1);
        let q = Literal::new_from_id(2);
        let cl = Clause::new_from_vec(vec!(p.negate(), q, p));
        assert!(trivial(&cl));
    }
    
    #[test]
    fn trivial_2() {
        let p = Literal::new_from_id(1);
        let q = Literal::new_from_id(2);
        let cl = Clause::new_from_vec(vec!(p, q));
        assert!(!trivial(&cl));
    }
    
    #[test]
    fn trivial_3() {
        let p = Literal::new_from_id(-1);
        let q = Literal::new_from_id(2);
        let r = Literal::new_from_id(3);
        let s = Literal::new_from_id(4);
        let cl = Clause::new_from_vec(vec!(q, s.clone(), r.negate(), p, s, r));
        assert!(trivial(&cl));
    }
} 

