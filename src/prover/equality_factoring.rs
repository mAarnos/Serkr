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

use prover::term::Term;
use prover::literal::Literal;
use prover::clause::Clause;
use prover::unification::mgu;
use prover::tautology_deletion::trivial;
use prover::term_ordering::TermOrdering;

/// Infers new clauses by equality factoring
/// Time complexity is O(n^2) where n is the amount of literals, but usually the clauses are rather short.
// TODO: see how much time is spent here.
pub fn equality_factoring<T: TermOrdering>(term_ordering: &T, cl: &Clause, factors: &mut Vec<Clause>) {
    for (i, l) in cl.iter().enumerate() {
        if l.is_negative() {
            continue;
        }
        
        for j in (i + 1)..cl.size() {
            if cl[j].is_negative() {
                continue;
            }
            
            // So we have found two equality literals. There are four ways to try to combine them.
            equality_factoring_create_new(term_ordering, cl, factors, l.get_lhs(), l.get_rhs(), cl[j].get_lhs(), cl[j].get_rhs(), i);
            equality_factoring_create_new(term_ordering, cl, factors, l.get_lhs(), l.get_rhs(), cl[j].get_rhs(), cl[j].get_lhs(), i);
            equality_factoring_create_new(term_ordering, cl, factors, l.get_rhs(), l.get_lhs(), cl[j].get_lhs(), cl[j].get_rhs(), i);
            equality_factoring_create_new(term_ordering, cl, factors, l.get_rhs(), l.get_lhs(), cl[j].get_rhs(), cl[j].get_lhs(), i);
        }
    }
}

fn equality_factoring_create_new<T: TermOrdering>(term_ordering: &T, 
                                                  cl: &Clause, 
                                                  factors: &mut Vec<Clause>, 
                                                  s: &Term, t: &Term, 
                                                  u: &Term, v: &Term, 
                                                  i: usize) {
    if let Some(theta) = mgu(s, u) {
        let mut l = Literal::new(false, s.clone(), t.clone());
        l.subst(&theta);
        
        if !term_ordering.gt(l.get_rhs(), l.get_lhs()) {        
            let mut new_cl = cl.clone();
            new_cl.swap_remove(i);
            new_cl.subst(&theta);
            
            if new_cl.iter().all(|lit| !term_ordering.gt_lit(lit, &l)) {
                let mut new_ineq_lit = Literal::new(true, t.clone(), v.clone());
                new_ineq_lit.subst(&theta);
                new_cl.add_literal(new_ineq_lit);
                
                if !trivial(&new_cl) {
                    factors.push(new_cl);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    
} 