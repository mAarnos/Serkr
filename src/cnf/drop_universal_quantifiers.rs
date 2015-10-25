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

 use utils::formula::Formula;

/// Drops all universal quantifiers from the start of a formula. 
pub fn drop_universal_quantifiers(f: Formula) -> Formula {
    match f {
        Formula::Forall(_, box p) => drop_universal_quantifiers(p),
        _ => f,
    }
}

/// Checks if the formula contains universal quantifiers.
fn contains_universal_quantifiers(f: Formula) -> bool {
    match f {
        Formula::And(box p, box q) | 
        Formula::Or(box p, box q) => contains_universal_quantifiers(p) && 
                                     contains_universal_quantifiers(q),
        Formula::Forall(_, _) => true,
        _ => false,
        
    }
}

#[cfg(test)]
mod test {
    use super::drop_universal_quantifiers;
    use parser::internal_parser::parse;
    
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