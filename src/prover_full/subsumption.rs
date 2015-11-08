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

use prover_full::term::Term;
use prover_full::literal::Literal;
use prover_full::clause::Clause;

fn term_match(env: &mut Vec<(Term, Term)>, mut eqs: Vec<(Term, Term)>) -> bool {
    while let Some((eq1, eq2)) = eqs.pop() {
        if eq1.is_function() && eq2.is_function() {
            if eq1.get_id() == eq2.get_id() {
                for eq in eq1.get_args().into_iter().zip(eq2.get_args().into_iter()) {
                    eqs.push(eq);
                }
            } else {
                return false;
            }
        } else if eq1.is_variable() {
            let mut success = false;
            for x in env.iter() {
                if x.0 == eq1 {
                    if x.1 != eq2 {
                        return false;
                    }
                    success = true;
                    break;
                }
            }
            
            if !success {
                env.push((eq1, eq2));
            }
            
        } else {
            return false;
        }
    }
    
    true
}

fn match_literals(env: &mut Vec<(Term, Term)>, p: Literal, q: Literal) -> bool {
    if p.is_positive() == q.is_positive() {
        let eqs = vec!((p.get_lhs().clone(), q.get_lhs().clone()), 
                       (p.get_rhs().clone(), q.get_rhs().clone()));
        term_match(env, eqs)
    } else {
        false
    }
}

fn subsumes_clause0(env: &mut Vec<(Term, Term)>, exclusion: &mut Vec<bool>, cl1: &Clause, cl2: &Clause, n: usize) -> bool {
    if n >= cl1.size() {
        true 
    } else {
        let s = env.len();  
        let l1 = &cl1[n];

        for (i, l2) in cl2.iter().enumerate() {
            if !exclusion[i] {
                exclusion[i] = true;
                let result = match_literals(env, l1.clone(), l2.clone()) && subsumes_clause0(env, exclusion, cl1, cl2, n + 1);
                exclusion[i] = false;
                if result {
                    return true;
                }
                env.truncate(s);
            }
        }
        
        false
    }
}

/// Checks if the clause cl1 subsumes the clause cl2.
/// We use multiset subsumption instead of set subsumption to prevent some undesirable effects like a clause subsuming its factors.
pub fn subsumes_clause(cl1: &Clause, cl2: &Clause) -> bool {
    if cl1.size() <= cl2.size() {    
        let mut env = Vec::<(Term, Term)>::new();
        let mut exclusion = vec![false; cl2.size()];
        subsumes_clause0(&mut env, &mut exclusion, cl1, cl2, 0)
    } else {
        false
    }
}

#[cfg(test)]
mod test {
    
} 

