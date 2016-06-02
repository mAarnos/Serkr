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

use cnf::ast::{Term, Formula};
use cnf::renaming_info::RenamingInfo;
use cnf::variable_renaming::rename;
use cnf::free_variables::free_variables;

/// Eliminates existential quantifiers by replacing them with new skolem functions.
/// To do this we also rename all bound variables.
pub fn skolemize(f: Formula, renaming_info: &mut RenamingInfo) -> Formula {
    let renamed_f = rename(f, renaming_info);
    let skolemized_f = skolemize1(renamed_f, renaming_info);
    assert!(!contains_existential_quantifiers(&skolemized_f));
    skolemized_f
}

fn skolemize1(f: Formula, ri: &mut RenamingInfo) -> Formula {
    match f {
        Formula::And(l) => Formula::And(l.into_iter().map(|x| skolemize1(x, ri)).collect()),
        Formula::Or(l) => Formula::Or(l.into_iter().map(|x| skolemize1(x, ri)).collect()),
        Formula::Forall(_, p) => skolemize1(*p, ri),
        Formula::Exists(id, p) => skolemize_exists(id, *p, ri),
        _ => f,
    }
}

fn skolemize_exists(id: i64, f: Formula, ri: &mut RenamingInfo) -> Formula {
    let skolem_f_id = ri.create_new_skolem_function_id();
    let mut vars = free_variables(&f);
    vars.remove(&id);
    let sf = Term::Function(skolem_f_id, vars.into_iter().map(Term::Variable).collect());
    skolemize1(tsubst(f, id, &sf), ri)
}

fn tsubst(f: Formula, from: i64, to: &Term) -> Formula {
    match f {
        Formula::Predicate(id, terms) => {
            Formula::Predicate(id,
                               terms.into_iter()
                                    .map(|term| tsubst_variable(term, from, to))
                                    .collect())
        }
        Formula::Not(p) => Formula::Not(Box::new(tsubst(*p, from, to))),
        Formula::And(l) => Formula::And(l.into_iter().map(|x| tsubst(x, from, to)).collect()),
        Formula::Or(l) => Formula::Or(l.into_iter().map(|x| tsubst(x, from, to)).collect()),
        Formula::Forall(id, p) => Formula::Forall(id, Box::new(tsubst(*p, from, to))),
        Formula::Exists(id, p) => Formula::Exists(id, Box::new(tsubst(*p, from, to))),
        _ => f,
    }
}

fn tsubst_variable(t: Term, from: i64, to: &Term) -> Term {
    match t {
        Term::Variable(id) => {
            if from == id {
                to.clone()
            } else {
                Term::Variable(id)
            }
        }
        Term::Function(id, subterms) => {
            Term::Function(id,
                           subterms.into_iter()
                                   .map(|term| tsubst_variable(term, from, to))
                                   .collect())
        }
    }
}

/// Checks if a given formula contains existential quantifiers.
/// Obviously after skolemization there shouldn't be any.
fn contains_existential_quantifiers(f: &Formula) -> bool {
    match *f {
        Formula::And(ref l) | Formula::Or(ref l) => l.iter().any(contains_existential_quantifiers),
        Formula::Forall(_, ref p) => contains_existential_quantifiers(p),
        Formula::Exists(_, _) => true,
        _ => false,
    }
}

#[cfg(test)]
mod test {}
