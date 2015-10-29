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
use utils::formula::{Term, Formula};
use cnf::free_variables::occurs_in;
use prover::literal::Literal;

fn subst(t: Term, from: &Term, to: &Term) -> Term {
    match t {
        x @ Term::Variable(_) => if x == *from { to.clone() } else { x },
        Term::Function(s, subterms) => Term::Function(s, subterms.into_iter().map(|term| subst(term, from, to)).collect())
    }
}

fn unify(mut env: HashMap<Term, Term>, mut eqs: Vec<(Term, Term)>) -> Result<HashMap<Term, Term>, ()> {
    while let Some(eq) = eqs.pop() {
        if eq.0 == eq.1 {
            continue; // delete
        } 
        
        match eq {
            (Term::Function(s1, args1), Term::Function(s2, args2)) => 
                if s1 == s2 && args1.len() == args2.len() {
                    // decompose                  
                    for eq in args1.into_iter().zip(args2.into_iter()) {
                        eqs.push(eq);
                    }
                } else {
                    return Err(()); // conflict
                },
            (t @ Term::Function(_, _), x) => eqs.push((x, t)), // swap
            (x, t) => 
                if occurs_in(&t, &x) {
                    return Err(()); // check
                } else {
                    // eliminate
                    env.insert(x.clone(), t.clone());
                    eqs = eqs.into_iter().map(|eq| (subst(eq.0, &x, &t), subst(eq.1, &x, &t))).collect();
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

fn unify_literals(env: HashMap<Term, Term>, tmp: (Formula, Formula)) -> Result<HashMap<Term, Term>, ()> {
    match tmp {
        (Formula::Predicate(p1, args1), Formula::Predicate(p2, args2)) => Ok(try!(unify(env, vec!((Term::Function(p1, args1), Term::Function(p2, args2)))))),
        (Formula::Not(box p), Formula::Not(box q)) => unify_literals(env, (p, q)),
        _ => Err(())
    }
}

fn unify_literals2(env: HashMap<Term, Term>, p: Literal, q: Literal) -> Result<HashMap<Term, Term>, ()> {
    if p.get_id() == q.get_id() {
        let mut eqs = Vec::<(Term, Term)>::new();
        for eq in p.get_arguments().into_iter().zip(q.get_arguments().into_iter()) {
            eqs.push(eq);
        }
        Ok(try!(unify(env, eqs)))
    } else {
        Err(())
    }
}

pub fn negate(f: Formula) -> Formula {
    match f {
        Formula::Not(box p) => p,
        _ => Formula::Not(box f)
    }
}

pub fn mgu(p: Formula, q: Formula) -> Result<HashMap<Term, Term>, ()> {
    let env = HashMap::<Term, Term>::new();   
    Ok(solve(try!(unify_literals(env, (p, q)))))
}

pub fn mgu2(p: Literal, q: Literal) -> Result<HashMap<Term, Term>, ()> {
    let env = HashMap::<Term, Term>::new();   
    Ok(solve(try!(unify_literals2(env, p, q))))
}

#[cfg(test)]
mod test {
    use super::mgu;
    use utils::formula::Term;
    use parser::internal_parser::parse;
    
    #[test]
    fn mgu_1() {
        let f1 = parse("P(f(x, g(y)))").unwrap();
        let f2 = parse("P(f(f(z), w))").unwrap();
        let theta = mgu(f1, f2).unwrap();
        assert_eq!(theta.len(), 2);
        assert_eq!(*theta.get(&Term::Variable("w".to_owned())).unwrap(), Term::Function("g".to_owned(), vec!(Term::Variable("y".to_owned()))));
        assert_eq!(*theta.get(&Term::Variable("x".to_owned())).unwrap(), Term::Function("f".to_owned(), vec!(Term::Variable("z".to_owned()))));
    }
    
    #[test]
    fn mgu_2() {
        let f1 = parse("~P(f(x, y))").unwrap();
        let f2 = parse("~P(f(y, x))").unwrap();
        let theta = mgu(f1, f2).unwrap();
        // Other way round is okay too.
        assert_eq!(theta.len(), 1);
        assert_eq!(*theta.get(&Term::Variable("y".to_owned())).unwrap(), Term::Variable("x".to_owned()));
    }
    
    #[test]
    fn mgu_3() {
        let f1 = parse("P(f(x, g(y)))").unwrap();
        let f2 = parse("P(f(y, x))").unwrap();
        let theta = mgu(f1, f2);
        assert!(theta.is_err());
    }
    
    #[test]
    fn mgu_4() {
        // In HOL this would work.
        let f1 = parse("P(x)").unwrap();
        let f2 = parse("Q(y)").unwrap();
        let theta = mgu(f1, f2);
        assert!(theta.is_err());
    }
    
    #[test]
    fn mgu_5() {
        let f1 = parse("R(y)").unwrap();
        let f2 = parse("R(f(y))").unwrap();
        let theta = mgu(f1, f2);
        assert!(theta.is_err());
    }
}    
