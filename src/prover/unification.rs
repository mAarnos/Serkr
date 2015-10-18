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
use parser::formula::Term;
use cnf::free_variables::occurs_in;

fn subst(t: Term, from: &Term, to: &Term) -> Term {
    match t {
        x @ Term::Variable(_) => if x == *from { to.clone() } else { x },
        Term::Function(s, subterms) => Term::Function(s, subterms.into_iter().map(|term| subst(term, from, to)).collect())
    }
}

fn unify(mut eqs: Vec<(Term, Term)>) -> Result<HashMap<Term, Term>, ()> {
    let mut env = HashMap::<Term, Term>::new();

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
    
    for (_, v) in new_env.iter_mut() {
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

fn fullunify(eqs: Vec<(Term, Term)>) -> Result<HashMap<Term, Term>, ()> {
    Ok(solve(try!(unify(eqs))))
}

#[cfg(test)]
mod test {
    use super::fullunify;
    use parser::formula::{Term, Formula};
    use parser::internal_parser::parse;
    
    fn equals_to_eqs(f: Formula) -> Vec<(Term, Term)> {
        match f {
            Formula::Predicate(_, args) => vec!((args[0].clone(), args[1].clone())),
            _ => panic!("WTF?")
        }
    }

    #[test]
    fn fullunify_1() {
        let f = parse("Equal(f(x, g(y)), f(f(z), w))").unwrap();
        let eqs = equals_to_eqs(f);
        let mgu = fullunify(eqs).unwrap();
        assert_eq!(mgu.len(), 2);
        assert_eq!(*mgu.get(&Term::Variable("w".to_string())).unwrap(), Term::Function("g".to_string(), vec!(Term::Variable("y".to_string()))));
        assert_eq!(*mgu.get(&Term::Variable("x".to_string())).unwrap(), Term::Function("f".to_string(), vec!(Term::Variable("z".to_string()))));
    }
    
    #[test]
    fn fullunify_2() {
        let f = parse("Equal(f(x, y), f(y, x))").unwrap();
        let eqs = equals_to_eqs(f);
        let mgu = fullunify(eqs).unwrap();
        // Other way round is okay too.
        assert_eq!(mgu.len(), 1);
        assert_eq!(*mgu.get(&Term::Variable("y".to_string())).unwrap(), Term::Variable("x".to_string()));
    }
    
    #[test]
    fn fullunify_3() {
        let f = parse("Equal(f(x, g(y)), f(y, x))").unwrap();
        let eqs = equals_to_eqs(f);
        assert!(fullunify(eqs).is_err());
    }
}    
