// Serkr - An automated theorem prover. Copyright (C) 2015-2016 Mikko Aarnos.
//
// Serkr is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Serkr is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Serkr. If not, see <http://www.gnu.org/licenses/>.
//

use cnf::ast::Formula;

/// Drops all universal quantifiers from the start of a formula.
pub fn drop_universal_quantifiers(f: Formula) -> Formula {
    let new_f = drop_universal_quantifiers0(f);
    assert!(!contains_universal_quantifiers(&new_f));
    new_f
}

/// The real meat of the above function.
fn drop_universal_quantifiers0(f: Formula) -> Formula {
    match f {
        Formula::Forall(_, p) => drop_universal_quantifiers0(*p),
        _ => f,
    }
}

/// Checks if the formula contains universal quantifiers.
fn contains_universal_quantifiers(f: &Formula) -> bool {
    match *f {
        Formula::And(ref l) |
        Formula::Or(ref l) => l.into_iter().any(contains_universal_quantifiers),
        Formula::Forall(_, _) => true,
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use super::drop_universal_quantifiers;
    use cnf::ast::{Term, Formula};

    #[test]
    fn drop_universal_quantifiers_1() {
        let x = Term::Variable(-1);
        let y = Term::Variable(-2);
        let p = Formula::Predicate(1, vec![x, y]);
        let f = Formula::Forall(-1, Box::new(Formula::Forall(-2, Box::new(p.clone()))));
        assert_eq!(drop_universal_quantifiers(f), p);
        assert_eq!(drop_universal_quantifiers(p.clone()), p);
    }
}
