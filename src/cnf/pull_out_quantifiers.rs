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

/// Move all universal quantifiers outwards.
pub fn pull_out_quantifiers(f: Formula) -> Formula {
    match f {
        Formula::And(l) => pull_out_quantifiers_and_or(l, true),
        Formula::Or(l) => pull_out_quantifiers_and_or(l, false),
        Formula::Forall(s, p) => Formula::Forall(s, Box::new(pull_out_quantifiers(*p))),
        _ => f,
    }
}

fn pull_out_quantifiers_and_or(l: Vec<Formula>, and_formula: bool) -> Formula {
    let pulled_out_l = l.into_iter().map(pull_out_quantifiers).collect::<Vec<_>>();
    let (quantifiers, mut normal): (Vec<_>, Vec<_>) = pulled_out_l.into_iter().partition(|x| match x { &Formula::Forall(_, _) => true, _ => false });
    
    let mut quantifier_list = Vec::new();
    for x in quantifiers.into_iter() {
        handle_quantifier(x, &mut normal, &mut quantifier_list);
    }
    
    let p = if and_formula {
                Formula::And(normal)
            } else {
                Formula::Or(normal)
            };
    quantifier_list.into_iter().fold(p, |acc, x| Formula::Forall(x, Box::new(acc)))        
}

fn handle_quantifier(f: Formula, l: &mut Vec<Formula>, ql: &mut Vec<i64>) {
    match f {
        Formula::Forall(id, p) => { ql.push(id); handle_quantifier(*p, l, ql); }
        _ => { l.push(f); }
    }
}

#[cfg(test)]
mod test {
    /*
    use super::pull_out_quantifiers;
    use cnf::ast_transformer_internal::{internal_to_cnf_ast, internal_to_cnf_ast_general};
    
    #[test]
    fn pull_out_quantifiers_1() {
        let (f, ri) = internal_to_cnf_ast("forall v0. (R(v0, v0) /\\ (P(sf0()) \\/ forall v2. (R(v2, sf1(v2)) \\/ forall v4. Q(v4))))").unwrap();
        let (correct_f, _) = internal_to_cnf_ast_general("forall v0. forall v2. forall v4. ((Q(v4) \\/ R(v2, sf1(v2)) \\/ P(sf0())) /\\ R(v0, v0))", ri).unwrap();
        assert_eq!(pull_out_quantifiers(f), correct_f);
    }
    */
}    
