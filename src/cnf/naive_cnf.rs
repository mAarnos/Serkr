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
use cnf::nnf::nnf;
use cnf::standard_skolemization::skolemize;
use cnf::pull_out_quantifiers::pull_out_quantifiers;
use cnf::drop_universal_quantifiers::drop_universal_quantifiers;
use cnf::distribute_ors_over_ands::distribute_ors_over_ands;

/// Turns a formula into CNF.
pub fn cnf(f: Formula) -> Formula {
    let nnf_f = nnf(f);
    let skolemized_f = skolemize(nnf_f);
    let quantifier_free_f = drop_universal_quantifiers(pull_out_quantifiers(skolemized_f));
    let cnf_f = distribute_ors_over_ands(quantifier_free_f);
    cnf_f
}

#[cfg(test)]
mod test {
    use super::cnf;
    use parser::parser::parse;
    
    #[test]
    fn cnf_1() {
        let f = parse("forall x. ((forall y. (Animal(y) ==> Loves(x, y))) ==> (exists y. Loves(y, x)))").unwrap();
        let correct_f = parse("((Loves(sf1(v0), v0) \\/ Animal(sf0(v0))) /\\ (Loves(sf1(v0), v0) \\/ ~Loves(v0, sf0(v0))))").unwrap();
        assert_eq!(cnf(f), correct_f);
    }
}    
