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

/// Converts a formula into an equivalent negation normal form.
pub fn nnf(f: Formula) -> Formula {
    move_nots_inward(eliminate_imp_and_eq(f))
}

/// Eliminates all implications and equivalences in a formula.
fn eliminate_imp_and_eq(f: Formula) -> Formula {
    match f {
        Formula::Predicate(_, _) => f,
        Formula::Not(box p) => Formula::Not(box eliminate_imp_and_eq(p)),
        Formula::And(box p, box q) => Formula::And(box eliminate_imp_and_eq(p), box eliminate_imp_and_eq(q)),
        Formula::Or(box p, box q) => Formula::Or(box eliminate_imp_and_eq(p), box eliminate_imp_and_eq(q)),
        Formula::Implies(box p, box q) => Formula::Or(box Formula::Not(box eliminate_imp_and_eq(p)), box eliminate_imp_and_eq(q)),
        Formula::Equivalent(box p, box q) => Formula::And(box Formula::Or(box eliminate_imp_and_eq(p.clone()), 
                                                                          box Formula::Not(box eliminate_imp_and_eq(q.clone()))), 
                                                          box Formula::Or(box Formula::Not(box eliminate_imp_and_eq(p)), 
                                                                          box eliminate_imp_and_eq(q))),
        Formula::Forall(s, box p) => Formula::Forall(s, box eliminate_imp_and_eq(p)),
        Formula::Exists(s, box p) => Formula::Exists(s, box eliminate_imp_and_eq(p)),
        _ => f,
    }
}

/// Moves all NOTs inward by repeatedly applying De Morgan's laws.
fn move_nots_inward(f: Formula) -> Formula {
    match f {
        Formula::Not(box Formula::Not(box p)) => move_nots_inward(p),
        Formula::Not(box Formula::And(box ref p, box ref q)) => Formula::Or(box move_nots_inward(Formula::Not(box p.clone())), box move_nots_inward(Formula::Not(box q.clone()))),
        Formula::Not(box Formula::Or(box ref p, box ref q)) => Formula::And(box move_nots_inward(Formula::Not(box p.clone())), box move_nots_inward(Formula::Not(box q.clone()))),
        Formula::Not(box Formula::Forall(ref s, box ref p)) => Formula::Exists(s.clone(), box move_nots_inward(Formula::Not(box p.clone()))),
        Formula::Not(box Formula::Exists(ref s, box ref p)) => Formula::Forall(s.clone(), box move_nots_inward(Formula::Not(box p.clone()))),
        Formula::Not(box p) => Formula::Not(box move_nots_inward(p)),
        Formula::And(box p, box q) => Formula::And(box move_nots_inward(p), box move_nots_inward(q)),
        Formula::Or(box p, box q) => Formula::Or(box move_nots_inward(p), box move_nots_inward(q)),
        Formula::Forall(s, box p) => Formula::Forall(s, box move_nots_inward(p)),
        Formula::Exists(s, box p) => Formula::Exists(s, box move_nots_inward(p)),
        _ => f
    }
}