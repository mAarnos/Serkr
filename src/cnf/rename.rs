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

use cnf::ast::{Term, Formula};

/// Renames variables so that different occurences of quantifiers bind different variables.
pub fn rename(f: Formula, n: &mut i64) -> Formula {
    match f {
        Formula::Not(p) => Formula::Not(Box::new(rename(*p, n))),
        Formula::And(p, q) => Formula::And(Box::new(rename(*p, n)), Box::new(rename(*q, n))),
        Formula::Or(p, q) => Formula::Or(Box::new(rename(*p, n)), Box::new(rename(*q, n))),
        Formula::Forall(id, p) => rename_quantifier(id, *p, n, true),
        Formula::Exists(id, p) => rename_quantifier(id, *p, n, false),
        _ => f
    }
}

fn rename_quantifier(id: i64, p: Formula, n: &mut i64, universal_quantifier: bool) -> Formula {
    *n -= 1;
    let new_id = *n;
    let renamed_p = rename(rename_variable(p, id, *n), n);
    
    if universal_quantifier {
        Formula::Forall(new_id, Box::new(renamed_p))
    } else {
        Formula::Exists(new_id, Box::new(renamed_p))
    }
}

/// Renames all occurrences of a single variable in a formula to another variable.
fn rename_variable(f: Formula, from: i64, to: i64) -> Formula {
    match f {
        Formula::Predicate(id, terms) => Formula::Predicate(id, terms.into_iter().map(|t| rename_variable_in_term(t, from, to)).collect()),
        Formula::Not(p) => Formula::Not(Box::new(rename_variable(*p, from, to))),
        Formula::And(p, q) => Formula::And(Box::new(rename_variable(*p, from, to)), Box::new(rename_variable(*q, from, to))),
        Formula::Or(p, q) => Formula::Or(Box::new(rename_variable(*p, from, to)), Box::new(rename_variable(*q, from, to))),
        Formula::Forall(id, p) => Formula::Forall(if id == from { to } else { id }, Box::new(rename_variable(*p, from, to))),
        Formula::Exists(id, p) => Formula::Exists(if id == from { to } else { id }, Box::new(rename_variable(*p, from, to))),
        _ => f
    }
}

/// Renames all occurrences of a single variable in a term to another variable.
fn rename_variable_in_term(t: Term, from: i64, to: i64) -> Term {
    match t {
        Term::Variable(id) => if from == id { Term::Variable(to) } else { Term::Variable(id) },
        Term::Function(id, args) => Term::Function(id, args.into_iter().map(|t2| rename_variable_in_term(t2, from, to)).collect())
    }
}

#[cfg(test)]
mod test {
    use super::{rename, rename_variable};
    use cnf::ast_transformer_internal::internal_to_cnf_ast;
    
    // The tests here are very fragile. Never figured out a better way for doing them though.
    
    #[test]
    fn rename_1() {
        let (f, mut ri) = internal_to_cnf_ast("forall x. exists x. P(x)").unwrap(); 
        let (almost_correct_f, _) = internal_to_cnf_ast("forall v0. exists v1. P(v1)").unwrap(); 
        let correct_f = rename_variable(rename_variable(almost_correct_f, -2, -3), -1, -2);
        
        assert_eq!(rename(f, &mut ri.var_cnt), correct_f);
    }
    
    #[test]
    fn rename_2() {
        let (f, mut ri) = internal_to_cnf_ast("forall x. forall y. (P(x, y) /\\ Q(y, z))").unwrap(); 
        let (almost_correct_f, _) = internal_to_cnf_ast("forall v0. forall v1. (P(v0, v1) /\\ Q(v1, z))").unwrap(); 
        let correct_f = rename_variable(rename_variable(almost_correct_f, -1, -4), -2, -5);
        
        assert_eq!(rename(f, &mut ri.var_cnt), correct_f);
    }
    
    #[test]
    fn rename_3() {
        let (f, mut ri) = internal_to_cnf_ast("forall x. (R(x, x) /\\ exists y. (P(y) \\/ forall x. exists y. (R(x, y) \\/ forall z. Q(z))))").unwrap(); 
        let (almost_correct_f, _) = internal_to_cnf_ast("forall v0. (R(v0, v0) /\\ exists v1. (P(v1) \\/ forall v2. exists v3. (R(v2, v3) \\/ forall v4. Q(v4))))").unwrap(); 
        let correct_f = rename_variable(rename_variable(rename_variable(rename_variable(rename_variable(almost_correct_f, -4, -7), -5, -8), -1, -4), -2, -5), -3, -6);
        
        assert_eq!(rename(f, &mut ri.var_cnt), correct_f);
    }
}    
