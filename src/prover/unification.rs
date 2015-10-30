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
use prover::literal::Literal;
use prover::term::Term;

fn unify(mut eqs: Vec<(Term, Term)>) -> Result<HashMap<Term, Term>, ()> {
    let mut env = HashMap::<Term, Term>::new(); 
    
    while let Some((eq1, eq2)) = eqs.pop() {
        if eq1 == eq2 {
            continue; // delete
        } 
        
        if eq1.is_function() && eq2.is_function() {
            if eq1.get_id() == eq2.get_id() && eq1.get_arity() == eq2.get_arity() {
                // decompose  
                for eq in eq1.get_args().into_iter().zip(eq2.get_args().into_iter()) {
                    eqs.push(eq);
                }
            } else {
                return Err(()); // conflict
            }
        } else if eq1.is_function() {
            // swap
            eqs.push((eq2, eq1));
        } else {
            if eq2.occurs(eq1.get_id()) {
                return Err(()); // check
            } else {
                // eliminate
                env.insert(eq1.clone(), eq2.clone());
                for eq in &mut eqs {
                    eq.0.subst(eq1.get_id(), &eq2);
                    eq.1.subst(eq1.get_id(), &eq2);
                }
            }
        }
    }
    
    Ok(env)
}

fn solve(env: HashMap<Term, Term>) -> HashMap<Term, Term> {
    let mut new_env = env.clone();
    
    for (_, v) in &mut new_env {
        if let Some(new_v) = env.get(v) {
            *v = new_v.clone();
        }
    }
    
    if new_env == env {
        new_env
    } else {
        solve(new_env)
    }
}

pub fn mgu(p: Literal, q: Literal) -> Result<HashMap<Term, Term>, ()> {
    if p.get_id() == q.get_id() && p.get_arity() == q.get_arity() {
        let eqs = p.get_arguments().into_iter().zip(q.get_arguments().into_iter()).collect();
        Ok(solve(try!(unify(eqs))))
    } else {
        Err(())
    }
}

#[cfg(test)]
mod test {
    use super::mgu;
    use prover::term::Term;
    use parser::internal_parser::parse;
    use prover::flatten_cnf::flatten_cnf;
    
    #[test]
    fn mgu_1() {
        let f = flatten_cnf(parse("(P(f(x, g(y))) /\\ P(f(f(z), w)))").unwrap());
        let f1 = f[0].at(0);
        let f2 = f[1].at(0);
        let theta = mgu(f1, f2).unwrap();
        assert_eq!(theta.len(), 2);
        assert_eq!(*theta.get(&Term::new(-4, Vec::new())).unwrap(), Term::new(2, vec!(Term::new(-2, Vec::new()))));
        assert_eq!(*theta.get(&Term::new(-1, Vec::new())).unwrap(), Term::new(1, vec!(Term::new(-3, Vec::new()))));
    }

    #[test]
    fn mgu_2() {
        let f = flatten_cnf(parse("(~P(f(x, y)) /\\ ~P(f(y, x)))").unwrap());
        let f1 = f[0].at(0);
        let f2 = f[1].at(0);
        let theta = mgu(f1, f2).unwrap();
        // Other way round is okay too.
        assert_eq!(theta.len(), 1);
        assert_eq!(*theta.get(&Term::new(-2, Vec::new())).unwrap(), Term::new(-1, Vec::new()));
    }
    
    #[test]
    fn mgu_3() {
        let f = flatten_cnf(parse("((P(f(x, g(y)))) /\\ P(f(y, x)))").unwrap());
        let f1 = f[0].at(0);
        let f2 = f[1].at(0);
        let theta = mgu(f1, f2);
        assert!(theta.is_err());
    }
    
    #[test]
    fn mgu_4() {
        // In HOL this would work.
        let f = flatten_cnf(parse("(P(x) /\\ Q(y))").unwrap());
        let f1 = f[0].at(0);
        let f2 = f[1].at(0);
        let theta = mgu(f1, f2);
        assert!(theta.is_err());
    }
    
    #[test]
    fn mgu_5() {
        let f = flatten_cnf(parse("(R(y) /\\ R(f(y)))").unwrap());
        let f1 = f[0].at(0);
        let f2 = f[1].at(0);
        let theta = mgu(f1, f2);
        assert!(theta.is_err());
    }
    
    #[test]
    fn mgu_6() {
        let f = flatten_cnf(parse("(F(y, y) /\\ F(f(x), x))").unwrap());
        let f1 = f[0].at(0);
        let f2 = f[1].at(0);
        let theta = mgu(f1, f2);
        assert!(theta.is_err());
    }
    
    #[test]
    fn mgu_7() {
        let f = flatten_cnf(parse("(P(x) /\\ P(x, x))").unwrap());
        let f1 = f[0].at(0);
        let f2 = f[1].at(0);
        let theta = mgu(f1, f2);
        assert!(theta.is_err());
    }
}    
