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

use parser::formula::{Term, Formula};
use cnf::simplify::simplify;
use cnf::nnf::nnf;

/// Turns a formula into CNF.
pub fn cnf(f: Formula) -> Formula {
    let nnf_f = nnf(simplify(f));
    nnf_f
}

/// Renames a single variable in a term. We do not check for collisions in this function.
fn rename_variable_in_term(t: Term, from: &str, to: &str) -> Term {
    match t {
        Term::Variable(s) => if from == s { Term::Variable(to.to_string()) } else { Term::Variable(s) },
        Term::Function(s, subterms) => Term::Function(s, subterms.into_iter().map(|term| rename_variable_in_term(term, from, to)).collect())
    }
}

/// Renames a single variable in a formula. We do not check for collisions in this function.
fn rename_variable(f: Formula, from: &str, to: &str) -> Formula {
    match f {
        Formula::Predicate(s, terms) => Formula::Predicate(s, terms.into_iter().map(|term| rename_variable_in_term(term, from, to)).collect()),
        Formula::Not(box p) => Formula::Not(box rename_variable(p, from, to)),
        Formula::And(box p, box q) => Formula::And(box rename_variable(p, from, to), box rename_variable(q, from, to)),
        Formula::Or(box p, box q) => Formula::Or(box rename_variable(p, from, to), box rename_variable(q, from, to)),
        Formula::Forall(s, box p) => Formula::Forall(if s == from { to.to_string() } else { s }, box rename_variable(p, from, to)),
        Formula::Exists(s, box p) => Formula::Exists(if s == from { to.to_string() } else { s }, box rename_variable(p, from, to)),
        _ => f
    }
}

/// Renames variables so that different occurences of quantifiers bind different variables.
fn ren(f: Formula, n: &mut isize) -> Formula {
    match f {
        Formula::Not(box p) => Formula::Not(box ren(p, n)),
        Formula::And(box p, box q) => Formula::And(box ren(p, n), box ren(q, n)),
        Formula::Or(box p, box q) => Formula::Or(box ren(p, n), box ren(q, n)),
        Formula::Forall(s, box p) => { let new_var = format!("v_{}", *n); *n += 1; Formula::Forall(new_var.clone(), box ren(rename_variable(p, &s, &new_var), n)) },
        Formula::Exists(s, box p) => { let new_var = format!("v_{}", *n); *n += 1; Formula::Exists(new_var.clone(), box ren(rename_variable(p, &s, &new_var), n)) }
        _ => f
    }
}

/// Drops all universal quantifiers.
fn drop_universal_quantifiers(f: Formula) -> Formula {
    match f {
        Formula::Forall(_, box p) => drop_universal_quantifiers(p),
        _ => f,
    }
}

/// Distributes ORs inwards over ANDs.
fn distribute_ors_over_ands(f: Formula) -> Formula {
    match f {
        Formula::And(box p, box q) => Formula::And(box distribute_ors_over_ands(p), box distribute_ors_over_ands(q)),
        Formula::Or(box ref p, box Formula::And(box ref q, box ref r)) | 
        Formula::Or(box Formula::And(box ref q, box ref r), box ref p) => Formula::And(box distribute_ors_over_ands(Formula::Or(box p.clone(), box q.clone())),
                                                                                       box distribute_ors_over_ands(Formula::Or(box p.clone(), box r.clone()))),
        Formula::Or(box p, box q) => Formula::Or(box distribute_ors_over_ands(p), box distribute_ors_over_ands(q)),
        _ => f,
    }
}