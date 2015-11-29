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
use prover::term_ordering::TermOrdering;
use prover::tautology_deletion::trivial;

fn create_overlapped_term(u: &Term, t: &Term, trace: &[usize]) -> Term {
    let mut new_term = u.clone();
    create_overlapped_term0(&mut new_term, t, trace, 0);
    new_term
}

fn create_overlapped_term0(u_p: &mut Term, t: &Term, trace: &[usize], n: usize) {
    if n >= trace.len() {
        *u_p = t.clone();
    } else {
        assert_neq!(u_p.get_arity(), 0);
        create_overlapped_term0(&mut u_p[trace[n]], t, trace, n + 1)
    }
}

fn overlaps<T: TermOrdering>(term_ordering: &T, 
                             s: &Term, t: &Term, 
                             u: &Term, v: &Term, u_v_negated: bool,
                             u_p: &Term,
                             cl1: &Clause, cl1_i: usize, 
                             cl2: &Clause,  cl2_i: usize, 
                             trace: &mut Vec<usize>,
                             generated: &mut Vec<Clause>) {
    if !u_p.is_variable() {
        if let Some(theta) = mgu(u_p, s) {            
            let mut new_s_t = Literal::new(false, s.clone(), t.clone());
            new_s_t.subst(&theta);      
            
            if !term_ordering.gt(new_s_t.get_rhs(), new_s_t.get_lhs()) {
                let mut new_u_v = Literal::new(u_v_negated, u.clone(), v.clone());
                new_u_v.subst(&theta); 
                
                if !term_ordering.gt(new_u_v.get_rhs(), new_u_v.get_lhs()) {
                    let mut new_c = cl1.clone();
                    new_c.swap_remove(cl1_i);
                    new_c.subst(&theta);
                    
                    if new_c.iter().all(|lit| !term_ordering.ge_lit(lit, &new_s_t)) {                                                      
                        let mut new_d = cl2.clone();
                        new_d.swap_remove(cl2_i);
                        new_d.subst(&theta);
                        
                        let maximality_condition_fulfilled = if u_v_negated { 
                                                                new_d.iter().all(|lit| !term_ordering.gt_lit(lit, &new_u_v)) 
                                                             } else { 
                                                                new_d.iter().all(|lit| !term_ordering.ge_lit(lit, &new_u_v)) 
                                                             };
                        if maximality_condition_fulfilled {
                            let new_u = create_overlapped_term(u, t, trace);
                            let mut new_lit = Literal::new(u_v_negated, new_u, v.clone());
                            new_lit.subst(&theta);
                            new_c.add_literals(new_d);
                            new_c.add_literal(new_lit);
                            
                            if !trivial(&new_c) {
                                generated.push(new_c);
                            }
                        }
                    }
                }
            }            
        }
        
        for (i, x) in u_p.iter().enumerate() {
            trace.push(i);
            overlaps(term_ordering, s, t, u, v, u_v_negated, x, cl1, cl1_i, cl2, cl2_i, trace, generated);
            trace.pop();
        }
    }
}

fn overlaps_literal<T: TermOrdering>(term_ordering: &T, 
                                     cl1: &Clause, cl1_i: usize, 
                                     cl2: &Clause, cl2_i: usize, 
                                     generated: &mut Vec<Clause>) {
    let mut trace = Vec::new();
    let l_lhs = cl1[cl1_i].get_lhs();
    let l_rhs = cl1[cl1_i].get_rhs();
    let r_negated = cl2[cl2_i].is_negative();
    let r_lhs = cl2[cl2_i].get_lhs();
    let r_rhs = cl2[cl2_i].get_rhs();
    
    // Four different ways to arrange two equations
    overlaps(term_ordering, l_lhs, l_rhs, r_lhs, r_rhs, r_negated, r_lhs, cl1, cl1_i, cl2, cl2_i, &mut trace, generated);
    overlaps(term_ordering, l_rhs, l_lhs, r_lhs, r_rhs, r_negated, r_lhs, cl1, cl1_i, cl2, cl2_i, &mut trace, generated);
    overlaps(term_ordering, l_lhs, l_rhs, r_rhs, r_lhs, r_negated, r_rhs, cl1, cl1_i, cl2, cl2_i, &mut trace, generated);
    overlaps(term_ordering, l_rhs, l_lhs, r_rhs, r_lhs, r_negated, r_rhs, cl1, cl1_i, cl2, cl2_i, &mut trace, generated);
}

/// Infers new clauses by positive and negative superposition.
/// Time complexity is who the fuck knows.
/// Assumes that cl1 was renamed so that it can have no variables in common with anything else.
pub fn superposition<T: TermOrdering>(term_ordering: &T, 
                                      cl1: &Clause, 
                                      cl2: &Clause, 
                                      generated: &mut Vec<Clause>) {
    for (i, l1) in cl1.iter().enumerate() {
        if l1.is_positive() {
            for j in 0..cl2.size() {
                overlaps_literal(term_ordering, cl1, i, cl2, j, generated);
            }
        }
    }
}

#[cfg(test)]
mod test {

} 
