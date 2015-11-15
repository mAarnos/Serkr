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

fn create_overlapped_term(t: &Term, new_part: &Term, trace: &[usize]) -> Term {
    let mut new_term = t.clone();
    create_overlapped_term0(&mut new_term, new_part, trace, 0);
    new_term
}

fn create_overlapped_term0(new_term: &mut Term, new_part: &Term, trace: &[usize], n: usize) {
    if n >= trace.len() {
        *new_term = new_part.clone();
    } else {
        assert_neq!(new_term.get_arity(), 0);
        create_overlapped_term0(&mut new_term[trace[n]], new_part, trace, n + 1)
    }
}

fn overlaps(l: &Term, r: &Term, term_top: &Term, t: &Term, trace: &mut Vec<usize>, rfn: &Fn(HashMap<Term, Term>, Term) -> Clause, paramodulants: &mut Vec<Clause>) {
    if t.is_function() {
        if let Some(theta) = mgu(l, t) {
            let new_term = create_overlapped_term(term_top, r, trace);
            let new_cl = rfn(theta, new_term);
            if !trivial(&new_cl) {
                paramodulants.push(new_cl);
            }

            for (i, x) in t.iter().enumerate() {
                if x.is_function() {
                    trace.push(i);
                    overlaps(l, r, term_top, x, trace, rfn, paramodulants);
                    trace.pop();
                }
            }
        }       
    }
}

fn overlaps_literal(eqn: &Literal, p: &Literal, rfn: &Fn(HashMap<Term, Term>, Literal) -> Clause, paramodulants: &mut Vec<Clause>) {
    let lhs_rfn = |theta, lhs| { rfn(theta, Literal::new(p.is_negative(), lhs, p.get_rhs().clone())) };
    let rhs_rfn = |theta, rhs| { rfn(theta, Literal::new(p.is_negative(), p.get_lhs().clone(), rhs)) };
    let mut trace = Vec::new();
    
    // s = t
    if lpo_gt(eqn.get_lhs(), eqn.get_rhs()) {         
        overlaps(eqn.get_lhs(), eqn.get_rhs(), p.get_lhs(), p.get_lhs(), &mut trace, &lhs_rfn, paramodulants);
        overlaps(eqn.get_lhs(), eqn.get_rhs(), p.get_rhs(), p.get_rhs(), &mut trace, &rhs_rfn, paramodulants);
    }
    // t = s
    if lpo_gt(eqn.get_rhs(), eqn.get_lhs()) {   
        overlaps(eqn.get_rhs(), eqn.get_lhs(), p.get_lhs(), p.get_lhs(), &mut trace, &lhs_rfn, paramodulants);
        overlaps(eqn.get_rhs(), eqn.get_lhs(), p.get_rhs(), p.get_rhs(), &mut trace, &rhs_rfn, paramodulants);
    }
}

/// Assumes that cl1 was renamed so that it can have no variables in common with anything else.
pub fn paramodulate_clauses(cl1: &Clause, cl2: &Clause, paramodulants: &mut Vec<Clause>) {
    for (i, l1) in cl1.iter().enumerate() {
        if l1.is_positive() {
            for (j, l2) in cl2.iter().enumerate() {
                let rfn = |theta, l| { 
                                        let mut ret = cl1.clone(); 
                                        ret.swap_remove(i);
                                        let mut d = cl2.clone();
                                        d.swap_remove(j);
                                        ret.add_literals(d); 
                                        ret.add_literal(l);
                                        ret.subst(&theta); 
                                        ret 
                                     };
                overlaps_literal(l1, l2, &rfn, paramodulants);
            }
        }
    }
}

#[cfg(test)]
mod test {
    
} 
