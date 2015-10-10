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

use parser::formula::{Formula};

/// Drops all universal quantifiers from the start of a formula. 
pub fn drop_universal_quantifiers(f: Formula) -> Formula {
    match f {
        Formula::Forall(_, box p) => drop_universal_quantifiers(p),
        _ => f,
    }
}

fn all_universal_quantifiers_are_outside_formula(f: Formula, outside: bool) -> bool {
    match f {
        Formula::Predicate(_, _) => true,
        Formula::Not(box p) => all_universal_quantifiers_are_outside_formula(p, false),
        Formula::And(box p, box q) | Formula::Or(box p, box q) => all_universal_quantifiers_are_outside_formula(p, false) && all_universal_quantifiers_are_outside_formula(q, false),
        Formula::Forall(_, box p) => if !outside { false } else { all_universal_quantifiers_are_outside_formula(p, true) },
        _ => panic!("These should have been eliminated previously, something is SERIOUSLY wrong somewhere!"),
        
    }
}

#[cfg(test)]
mod test {
    use super::{drop_universal_quantifiers};
    use parser::parser::parse;
    
    #[test]
    fn drop_universal_quantifiers_1() {
        let f = parse("forall x. forall y. P(x, y)").unwrap();
        let correct_f = parse("P(x, y)").unwrap();
        assert_eq!(drop_universal_quantifiers(f), correct_f);
    }
    
    #[test]
    fn drop_universal_quantifiers_2() {
        let f = parse("P(x, f(y))").unwrap();
        assert_eq!(drop_universal_quantifiers(f.clone()), f);
    }
}    