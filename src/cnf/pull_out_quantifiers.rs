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

/// Move all universal quantifiers outwards.
pub fn pull_out_quantifiers(f: Formula) -> Formula {
    match f {
        Formula::And(box p, box q) => pull_out_quantifiers_and_or(p, q, true),
        Formula::Or(box p, box q) => pull_out_quantifiers_and_or(p, q, false),
        Formula::Forall(s, box p) => Formula::Forall(s, box pull_out_quantifiers(p)),
        _ => f,
    }
}

fn pull_out_quantifiers_and_or(f1: Formula, f2: Formula, and_formula: bool) -> Formula {
    let pulled_out_f1 = pull_out_quantifiers(f1);
    let pulled_out_f2 = pull_out_quantifiers(f2);
    
    match (pulled_out_f1, pulled_out_f2) {
        (Formula::Forall(s, box p), q) | 
        (q, Formula::Forall(s, box p)) => Formula::Forall(s, box pull_out_quantifiers(and_or_formula(p, q, and_formula))),
        (p, q) => and_or_formula(p, q, and_formula),
    }
}

fn and_or_formula(f1: Formula, f2: Formula, and_formula: bool) -> Formula {
    if and_formula {
        Formula::And(box f1, box f2)
    } else {
        Formula::Or(box f1, box f2)
    }
}

#[cfg(test)]
mod test {
    use super::pull_out_quantifiers;
    use parser::internal_parser::parse;
    
    #[test]
    fn pull_out_quantifiers_1() {
        let f = parse("forall v0. (R(v0, v0) /\\ (P(sf0()) \\/ forall v2. (R(v2, sf1(v2)) \\/ forall v4. Q(v4))))").unwrap();
        let correct_f = parse("forall v0. forall v2. forall v4. (((Q(v4) \\/ R(v2, sf1(v2))) \\/ P(sf0())) /\\ R(v0, v0))").unwrap();
        assert_eq!(pull_out_quantifiers(f), correct_f);
    }
}    
