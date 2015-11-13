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

use std::collections::HashMap;
use prover::term::Term;
use prover::literal::Literal;
use prover::clause::Clause;
use prover::tautology_deletion::trivial;
use prover::unification::mgu;
use prover::lpo::lpo_gt;

fn overlaps(l: &Term, r: &Term, t: &Term, rfn: &Fn(HashMap<Term, Term>, Term) -> Clause, paramodulants: &mut Vec<Clause>) {
    if t.is_function() {
        if let Some(theta) = mgu(l, t) {
            let new_cl = rfn(theta, r.clone());
            if !trivial(&new_cl) {
                paramodulants.push(new_cl);
            }

            let args = t.get_args();
            for (i, x) in args.iter().enumerate() {
                if x.is_function() {
                    let new_rfn = |theta, mut h: Term| { let mut args2 = args.clone(); 
                                                         h.subst_general(&theta); 
                                                         args2[i] = h;
                                                         rfn(theta, Term::new(t.get_id(), t.get_sort_predicate(), args2)) };
                    overlaps(l, r, x, &new_rfn, paramodulants);
                }
            }
        }       
    }
}

fn overlaps_literal(eqn: &Literal, p: &Literal, rfn: &Fn(HashMap<Term, Term>, Literal) -> Clause, paramodulants: &mut Vec<Clause>) {
    let lhs_rfn = |theta, lhs| { rfn(theta, Literal::new(!p.is_positive(), lhs, p.get_rhs().clone())) };
    let rhs_rfn = |theta, rhs| { rfn(theta, Literal::new(!p.is_positive(), p.get_lhs().clone(), rhs)) };
    // s = t
    if lpo_gt(eqn.get_lhs(), eqn.get_rhs()) {    
        overlaps(eqn.get_lhs(), eqn.get_rhs(), p.get_lhs(), &lhs_rfn, paramodulants);
        overlaps(eqn.get_lhs(), eqn.get_rhs(), p.get_rhs(), &rhs_rfn, paramodulants);
    }
    // t = s
    if lpo_gt(eqn.get_rhs(), eqn.get_lhs()) {   
        overlaps(eqn.get_rhs(), eqn.get_lhs(), p.get_lhs(), &lhs_rfn, paramodulants);
        overlaps(eqn.get_rhs(), eqn.get_lhs(), p.get_rhs(), &rhs_rfn, paramodulants);
    }
}

/// Assumes that cl1 was renamed so that it can have no variables in common with anything else.
pub fn paramodulate_clauses(cl1: &Clause, cl2: &Clause, paramodulants: &mut Vec<Clause>) {
    for (i, l1) in cl1.iter().enumerate() {
        if l1.is_positive() {
            let mut c = cl1.clone();
            c.swap_remove(i);
            for (j, l2) in cl2.iter().enumerate() {
                let mut d = cl2.clone();
                d.swap_remove(j);
                let rfn = |theta, l| { let mut ret = c.clone(); 
                                       ret.add_literals(d.clone()); 
                                       ret.add_literal(l);
                                       ret.subst(&theta); 
                                       ret };
                overlaps_literal(l1, l2, &rfn, paramodulants);
            }
        }
    }
}

#[cfg(test)]
mod test {
    
} 
