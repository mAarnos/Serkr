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

 use cnf::ast::Formula;

/// Drops all universal quantifiers from the start of a formula. 
pub fn drop_universal_quantifiers(f: Formula) -> Formula {
    let new_f = drop_universal_quantifiers0(f);
    assert!(!contains_universal_quantifiers(&new_f));
    new_f
}

/// Drops all universal quantifiers from the start of a formula. 
pub fn drop_universal_quantifiers0(f: Formula) -> Formula {
    match f {
        Formula::Forall(_, p) => drop_universal_quantifiers0(*p),
        _ => f,
    }
}

/// Checks if the formula contains universal quantifiers.
fn contains_universal_quantifiers(f: &Formula) -> bool {
    match *f {
        Formula::And(ref p, ref q) | 
        Formula::Or(ref p, ref q) => contains_universal_quantifiers(p) && 
                                     contains_universal_quantifiers(q),
        Formula::Forall(_, _) => true,
        _ => false,
        
    }
}

#[cfg(test)]
mod test {
    use super::drop_universal_quantifiers;
    use cnf::ast_transformer_internal::internal_to_cnf_ast;
    
    #[test]
    fn drop_universal_quantifiers_1() {
        let (f, _) = internal_to_cnf_ast("forall x. forall y. P(x, y)").unwrap();
        let (correct_f, _) = internal_to_cnf_ast("P(x, y)").unwrap();
        assert_eq!(drop_universal_quantifiers(f), correct_f);
    }
    
    #[test]
    fn drop_universal_quantifiers_2() {
        let (f, _) = internal_to_cnf_ast("P(x, f(y))").unwrap();
        assert_eq!(drop_universal_quantifiers(f.clone()), f);
    }
}    