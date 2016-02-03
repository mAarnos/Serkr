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
use cnf::renaming_info::RenamingInfo;
use cnf::rename::rename;
use cnf::free_variables::fv;

/// Eliminates existential quantifiers by replacing them with new skolem functions.
/// To do this we also rename all bound variables.
pub fn skolemize(f: Formula, renaming_info: &mut RenamingInfo) -> Formula {
    let renamed_f = rename(f, &mut renaming_info.var_cnt);
    let skolemized_f = skolemize1(renamed_f, &mut renaming_info.fun_cnt);
    assert!(!contains_existential_quantifiers(&skolemized_f));
    skolemized_f
}

fn skolemize1(f: Formula, n: &mut i64) -> Formula {
    match f {
        Formula::And(p, q) => Formula::And(Box::new(skolemize1(*p, n)), Box::new(skolemize1(*q, n))),
        Formula::Or(p, q) => Formula::Or(Box::new(skolemize1(*p, n)), Box::new(skolemize1(*q, n))),
        Formula::Forall(id, p) => Formula::Forall(id, Box::new(skolemize1(*p, n))),
        Formula::Exists(id, p) => skolemize_exists(id, *p, n),
        _ => f,
    }
}

fn skolemize_exists(id: i64, f: Formula, n: &mut i64) -> Formula {
    *n += 1;
    let xs = fv(&Formula::Exists(id, Box::new(f.clone()))); 
    let sf = Term::Function(*n, xs.into_iter().map(Term::Variable).collect()); 
    skolemize1(tsubst(f, id, sf), n)
}

fn tsubst(f: Formula, from: i64, to: Term) -> Formula {
    match f {
        Formula::Predicate(id, terms) => Formula::Predicate(id, terms.into_iter().map(|term| tsubst_variable(term, from, to.clone())).collect()),
        Formula::Not(p) => Formula::Not(Box::new(tsubst(*p, from, to))),
        Formula::And(p, q) => Formula::And(Box::new(tsubst(*p, from, to.clone())), Box::new(tsubst(*q, from, to))),
        Formula::Or(p, q) => Formula::Or(Box::new(tsubst(*p, from, to.clone())), Box::new(tsubst(*q, from, to))),
        Formula::Forall(id, p) => Formula::Forall(id, Box::new(tsubst(*p, from, to))),
        Formula::Exists(id, p) => Formula::Exists(id, Box::new(tsubst(*p, from, to))),
        _ => f
    }
}

fn tsubst_variable(t: Term, from: i64, to: Term) -> Term {
    match t {
        Term::Variable(id) => if from == id { to } else { Term::Variable(id) },
        Term::Function(id, subterms) => Term::Function(id, subterms.into_iter().map(|term| tsubst_variable(term, from, to.clone())).collect())
    }
}

/// Checks if a given formula contains existential quantifiers. After skolemization this should not happen at all.
fn contains_existential_quantifiers(f: &Formula) -> bool {
    match *f {
        Formula::And(ref p, ref q) | Formula::Or(ref p, ref q) => contains_existential_quantifiers(&p) || contains_existential_quantifiers(&q),
        Formula::Forall(_, ref p) => contains_existential_quantifiers(&p),
        Formula::Exists(_, _) => true,
        _ => false
    }
}

#[cfg(test)]
mod test {
    use super::skolemize1;
    use cnf::ast_transformer_internal::{internal_to_cnf_ast, internal_to_cnf_ast_general};

    #[test]
    fn skolemize1_1() {
        let (f, mut ri) = internal_to_cnf_ast("forall v0. (R(v0, v0) /\\ exists v1. (P(v1) \\/ forall v2. exists v3. (R(v2, v3) \\/ forall v4. Q(v4))))").unwrap(); 
        let (correct_f, _) = internal_to_cnf_ast_general("forall v0. (R(v0, v0) /\\ (P(sf0()) \\/ forall v2. (R(v2, sf1(v2)) \\/ forall v4. Q(v4))))", ri.clone()).unwrap(); 
        assert_eq!(skolemize1(f, &mut ri.fun_cnt), correct_f);
    }
}    
