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
use prover_full::term::Term;

fn unify(mut eqs: Vec<(Term, Term)>) -> Result<HashMap<Term, Term>, ()> {
    let mut env = HashMap::<Term, Term>::new(); 
    
    while let Some((eq1, eq2)) = eqs.pop() {
        if eq1 == eq2 {
            continue; // delete
        } 
        
        if eq1.is_function() && eq2.is_function() {
            if eq1.get_id() == eq2.get_id() {
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

/// Tries to find the most general unifier of two terms.
pub fn mgu(p: Term, q: Term) -> Result<HashMap<Term, Term>, ()> {
    Ok(solve(try!(unify(vec!((p, q))))))
}

#[cfg(test)]
mod test {
    /*
    use super::mgu;
    use prover_full::term::Term;
    use parser::internal_parser::parse;
    use prover_full::flatten_cnf::flatten_cnf;
    
    // TODO: some of the tests here rely on the behaviour of flatten_cnf. Figure out a way to deal with that.
    
    #[test]
    fn mgu_1() {
        let f = flatten_cnf(parse("(P(f(x, g(y))) /\\ P(f(f(z), w)))").unwrap()).0;
        let f1 = f[0][0].clone();
        let f2 = f[1][0].clone();
        let theta = mgu(f1, f2).unwrap();
        assert_eq!(theta.len(), 2);
        assert_eq!(*theta.get(&Term::new(-4, Vec::new())).unwrap(), Term::new(3, vec!(Term::new(-2, Vec::new()))));
        assert_eq!(*theta.get(&Term::new(-1, Vec::new())).unwrap(), Term::new(4, vec!(Term::new(-3, Vec::new()))));
    }

    #[test]
    fn mgu_2() {
        let f = flatten_cnf(parse("(~P(f(x, y)) /\\ ~P(f(y, x)))").unwrap()).0;
        let f1 = f[0][0].clone();
        let f2 = f[1][0].clone();
        let theta = mgu(f1, f2).unwrap();
        // Other way round is okay too.
        assert_eq!(theta.len(), 1);
        assert_eq!(*theta.get(&Term::new(-2, Vec::new())).unwrap(), Term::new(-1, Vec::new()));
    }
    
    #[test]
    fn mgu_3() {
        let f = flatten_cnf(parse("((P(f(x, g(y)))) /\\ P(f(y, x)))").unwrap()).0;
        let f1 = f[0][0].clone();
        let f2 = f[1][0].clone();
        let theta = mgu(f1, f2);
        assert!(theta.is_err());
    }
    
    #[test]
    fn mgu_4() {
        // In HOL this would work.
        let f = flatten_cnf(parse("(P(x) /\\ Q(y))").unwrap()).0;
        let f1 = f[0][0].clone();
        let f2 = f[1][0].clone();
        let theta = mgu(f1, f2);
        assert!(theta.is_err());
    }
    
    #[test]
    fn mgu_5() {
        let f = flatten_cnf(parse("(R(y) /\\ R(f(y)))").unwrap()).0;
        let f1 = f[0][0].clone();
        let f2 = f[1][0].clone();
        let theta = mgu(f1, f2);
        assert!(theta.is_err());
    }
    
    #[test]
    fn mgu_6() {
        let f = flatten_cnf(parse("(F(y, y) /\\ F(f(x), x))").unwrap()).0;
        let f1 = f[0][0].clone();
        let f2 = f[1][0].clone();
        let theta = mgu(f1, f2);
        assert!(theta.is_err());
    }
    
    #[test]
    fn mgu_7() {
        let f = flatten_cnf(parse("(P(x) /\\ P(x, x))").unwrap()).0;
        let f1 = f[0][0].clone();
        let f2 = f[1][0].clone();
        let theta = mgu(f1, f2);
        assert!(theta.is_err());
    }
    */
}    
