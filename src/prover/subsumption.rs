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

fn term_match(mut env: HashMap<Term, Term>, mut eqs: Vec<(Term, Term)>) -> Result<HashMap<Term, Term>, ()> {
    while let Some((eq1, eq2)) = eqs.pop() {
        if eq1.is_function() && eq2.is_function() {
            if eq1.get_id() == eq2.get_id() && eq1.get_arity() == eq2.get_arity() {
                for eq in eq1.get_args().into_iter().zip(eq2.get_args().into_iter()) {
                    eqs.push(eq);
                }
            } else {
                return Err(());
            }
        } else {
            if let Some(eq3) = env.get(&eq1) {
                if *eq3 != eq2 {
                    return Err(()); 
                }
                continue;
            } 
            env.insert(eq1, eq2);
        }
    }
    
    Ok(env)
}

fn match_literals(env: HashMap<Term, Term>, p: Literal, q: Literal) -> Result<HashMap<Term, Term>, ()> {
    if p.get_id() == q.get_id() && p.get_arity() == q.get_arity() {
        let eqs = p.get_arguments().into_iter().zip(q.get_arguments().into_iter()).collect();
        Ok(try!(term_match(env, eqs)))
    } else {
        Err(())
    }
}

/// Checks if the clause cl1 subsumes the clause cl2.
pub fn subsumes_clause(cl1: &Clause, cl2: &Clause) -> bool {
    let mut env = HashMap::<Term, Term>::new();
    
    for l1 in cl1.iter() {
        let mut found_match = false;
        
        for l2 in cl2.iter().cloned() {
            if let Ok(theta) = match_literals(env.clone(), l1.clone(), l2) {
                env = theta;
                found_match = true;
                break;
            }
        }
        
        if !found_match {
            return false;
        }
    }
    
    true
}

#[cfg(test)]
mod test {
    
} 

