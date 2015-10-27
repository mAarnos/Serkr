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

use utils::set::Set;
use utils::formula::{Term, Formula};
use cnf::naive_cnf::cnf;
use cnf::free_variables::fv;
use prover::unification::{negate, mgu};
use std::collections::HashMap;
use parser::internal_parser::parse;

fn rename_variables(f: Formula, sfn: &HashMap<String, String>) -> Formula {
    match f {
        Formula::Predicate(s, terms) => Formula::Predicate(s, terms.into_iter().map(|t| rename_variables_in_term(t, sfn)).collect()),
        Formula::Not(box p) => Formula::Not(box rename_variables(p, sfn)),
        _ => f
    }
}

fn rename_variables_in_term(t: Term, sfn: &HashMap<String, String>) -> Term {
    match t {
        Term::Variable(s) => if let Some(s2) = sfn.get(&s) { Term::Variable(s2.clone()) } else { Term::Variable(s) },
        Term::Function(s, args) => Term::Function(s, args.into_iter().map(|t2| rename_variables_in_term(t2, sfn)).collect())
    }
}

fn tsubst(f: Formula, sfn: &HashMap<Term, Term>) -> Formula {
    match f {
        Formula::Predicate(s, args) => Formula::Predicate(s, args.into_iter().map(|arg| tsubst_variable(arg, sfn)).collect()),
        Formula::Not(box p) => Formula::Not(box tsubst(p, sfn)),
        _ => f
    }
}

fn tsubst_variable(t: Term, sfn: &HashMap<Term, Term>) -> Term {
    if let Some(t2) = sfn.get(&t) {
        t2.clone()
    } else {
        match t {
            Term::Variable(_) => t,
            Term::Function(s, args) => Term::Function(s, args.into_iter().map(|arg| tsubst_variable(arg, sfn)).collect())
        }
    }
}

fn rename(pfx: String, cl: &mut Vec<Formula>) {
    let fvs: Set<String> = cl.iter().flat_map(|f| fv(f.clone())).collect();
    let mut mapping = HashMap::<String, String>::new();
    for x in fvs.into_iter() {
        mapping.insert(x.clone(), pfx.clone() + &x); 
    }
    for f in cl.iter_mut() {
        *f = rename_variables(f.clone(), &mapping);
    }
}

fn add_resolvents(cl1: &[Formula], cl2: &[Formula], p: Formula, unused: &mut Vec<Vec<Formula>>) {
    let neg_p = negate(p.clone());
    for x in cl2.iter().cloned() {
        let possible_theta = mgu(vec!(x.clone(), neg_p.clone()));
        if let Ok(theta) = possible_theta {
            let mut cl1_done: Vec<Formula> = cl1.iter()
                                                .cloned()
                                                .filter(|l| *l != p)
                                                .map(|l| tsubst(l, &theta))
                                                .collect();
            let mut cl2_done: Vec<Formula> = cl2.iter()
                                                .cloned()
                                                .filter(|l| *l != x)
                                                .map(|l| tsubst(l, &theta))
                                                .collect();
            cl1_done.append(&mut cl2_done);
            if !trivial(&cl1_done) {
                unused.push(cl1_done);
            }
        }
    }
}

fn positive(f: &Formula) -> bool {
    match *f {
        Formula::Not(_) => false,
        _ => true
    }
}

fn resolve_clauses(mut cl1: Vec<Formula>, mut cl2: Vec<Formula>, unused: &mut Vec<Vec<Formula>>) {
    // Positive resolution: one of the resolution clauses must be all-positive.
    if cl1.iter().all(positive) || cl2.iter().all(positive) {
        rename("x".to_owned(), &mut cl1);
        rename("y".to_owned(), &mut cl2);
        for p in cl1.iter().cloned() {
            add_resolvents(&cl1, &cl2, p, unused);
        }
    }
}

/// Picks and removes the "best" clause from the unused clauses according to heuristics.
/// Currently just picks the shortest one.
fn pick_clause(unused: &mut Vec<Vec<Formula>>) -> Vec<Formula> {
    // TODO: can be done better by using max.
    let mut best_clause_index = 0;
    let mut best_clause_size = unused[0].len();
    
    for i in 1..unused.len() {
        if unused[i].len() < best_clause_size {
            best_clause_index = i;
            best_clause_size = unused[i].len();
        }
    }
    
    unused.swap_remove(best_clause_index)
}

/// Checks if a clause is trivial, i.e. it is a syntactical tautology.
fn trivial(cl: &[Formula]) -> bool {
    for (i, lit) in cl.iter().enumerate() {
        let counterpart = negate(lit.clone());
        for j in (i + 1)..cl.len() {
            if cl[j] == counterpart {
                return true;
            }
        }
    }  
    false
}

/// Factors a clause if possible. Returns true if something was factored.
fn factor(cl: &mut Vec<Formula>) -> bool {
    let mut factored_something = false;
    let mut i = 0;
    
    while i < cl.len() {
        let mut j = i + 1;
        
        while j < cl.len() {
            if let Ok(theta) = mgu(vec!(cl[i].clone(), cl[j].clone())) {
                cl.swap_remove(j);
                for l in cl.iter_mut() {
                    *l = tsubst(l.clone(), &theta);
                }
                factored_something = true;
                continue;
            }
            j += 1;
        }
        
        i += 1;
    }
    
    factored_something
}

fn resolution_loop(mut used: Vec<Vec<Formula>>, mut unused: Vec<Vec<Formula>>) -> Result<bool, &'static str> {
    while !unused.is_empty() {
        let mut chosen_clause = pick_clause(&mut unused);
        factor(&mut chosen_clause);
        assert!(!factor(&mut chosen_clause));
        used.push(chosen_clause.clone());
        
        for cl in &used {
            resolve_clauses(chosen_clause.clone(), cl.clone(), &mut unused);
        }
        
        // TODO: ridiculously inefficient, replace.
        if unused.iter().any(|cl| cl.is_empty()) {
            return Ok(true);
        }
    }
    Err("No proof found.")
}

fn collect_or(f: Formula) -> Vec<Formula> {
    match f {
        Formula::Predicate(_, _) => vec!(f),
        Formula::Not(box Formula::Predicate(_, _)) => vec!(f),
        Formula::Or(box p, box q) => { let mut left = collect_or(p); left.append(&mut collect_or(q)); left }
        _ => panic!("Encountered something which should already be gone")
    }
}

fn collect(f: Formula) -> Vec<Vec<Formula>> {
    match f {
        Formula::Predicate(_, _) => vec!(vec!(f)),
        Formula::Not(box Formula::Predicate(_, _)) => vec!(vec!(f)),
        Formula::Or(_, _) => vec!(collect_or(f)),
        Formula::And(box p, box q) => { let mut left = collect(p); left.append(&mut collect(q)); left }
        _ => panic!("Encountered something which should already be gone")
    }
}

pub fn resolution(s: &str) -> Result<bool, &'static str> {
    let cnf_f = cnf(Formula::Not(box parse(s).unwrap()));
    if cnf_f == Formula::False {
        Ok(true)
    } else if cnf_f == Formula::True {
        Err("False.")
    } else {
        resolution_loop(Vec::new(), collect(cnf_f).into_iter().filter(|cl| !trivial(cl)).collect())
    }
}

#[cfg(test)]
mod test {
    use super::{resolution, trivial};
    use prover::unification::negate;
    use utils::formula::Formula;
    
    #[test]
    fn unprovable() {
        let result = resolution("(P ==> (Q ==> R))");
        assert!(result.is_err());
    }
    
    #[test]
    fn pelletier_1() {
        let result = resolution("((P ==> Q) <=> (~Q ==> ~P))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_2() {
        let result = resolution("(~~P <=> P)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_3() {
        let result = resolution("(~(P ==> Q) ==> (Q ==> P))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_4() {
        let result = resolution("((~P ==> Q) <=> (~Q ==> P))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_5() {
        let result = resolution("(((P \\/ Q) ==> (P \\/ R)) ==> (P \\/ (Q ==> R)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_6() {
        let result = resolution("(P \\/ ~P)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_7() {
        let result = resolution("(P \\/ ~~~P)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_8() {
        let result = resolution("(((P ==> Q) ==> P) ==> P)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_9() {
        let result = resolution("((((P \\/ Q) /\\ (~P \\/ Q)) /\\ (P \\/ ~Q)) ==> ~(~P \\/ ~Q))");
        assert!(result.is_ok());
    }
    
    /*
    #[test]
    fn pelletier_10() {
        let result = resolution("(((((Q ==> R) /\\ (R ==> (P /\\ Q))) /\\ (P ==> (Q \\/ R)))) ==> (P <=> Q))");
        assert!(result.is_ok());
    }
    */
    #[test]
    fn pelletier_11() {
        let result = resolution("(P <=> P)");
        assert!(result.is_ok());
    }
    
    /*
    #[test]
    fn pelletier_12() {
        let result = resolution("(((P <=> Q) <=> R) <=> (P <=> (Q <=> R)))");
        assert!(result.is_ok());
    }
    */
    
    /*
    #[test]
    fn pelletier_13() {
        let result = resolution("((P \\/ (Q /\\ R)) <=> ((P \\/ Q) /\\ (P \\/ R)))");
        assert!(result.is_ok());
    }
    */
    
    /*
    #[test]
    fn pelletier_14() {
        let result = resolution("((P <=> Q) <=> ((~P \\/ Q) /\\ (~Q \\/ P)))");
        assert!(result.is_ok());
    }
    */
    
    /*
    #[test]
    fn pelletier_15() {
        let result = resolution("((P ==> Q) <=> (~P \\/ Q))");
        assert!(result.is_ok());
    }
    */
    
    #[test]
    fn pelletier_16() {
        let result = resolution("((P ==> Q) \\/ (Q ==> P))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_18() {
        let result = resolution("exists y. forall x. (F(y) ==> F(x))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_19() {
        let result = resolution("exists x. forall y. forall z. ((P(y) ==> Q(z)) ==> (P(x) ==> Q(x)))");
        assert!(result.is_ok());
    }
    
    /*
    #[test]
    fn pelletier_22() {
        let result = resolution("((forall x. (P <=> F(x))) ==> (P <=> forall x. F(x)))");
        assert!(result.is_ok());
    }
    */

    #[test]
    fn pelletier_35() {
        let result = resolution("exists x. exists y. (P(x, y) ==> forall x. forall y. P(x, y))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_39() {
        let result = resolution("~exists x. forall y. (F(y, x) <=> ~F(y, y))");
        assert!(result.is_ok());
    }
    
    /*
    #[test]
    fn davis_putnam() {
        let result = resolution("exists x. exists y. forall z. ((F(x, y) ==> (F(y, z) /\\ F(z, z))) /\\ ((F(x, y) /\\ G(x, y)) ==> (G(x, z) /\\ G(z, z))))");
        assert!(result.is_ok());
    }
    */
    
    #[test]
    fn trivial_1() {
        let p = Formula::Predicate("P".to_owned(), Vec::new());
        let q = Formula::Predicate("Q".to_owned(), Vec::new());
        let cls = vec!(p.clone(), q, negate(p));
        assert!(trivial(&cls));
    }
    
    #[test]
    fn trivial_2() {
        let p = Formula::Predicate("P".to_owned(), Vec::new());
        let q = Formula::Predicate("Q".to_owned(), Vec::new());
        let cls = vec!(p, q);
        assert!(!trivial(&cls));
    }
    
    #[test]
    fn trivial_3() {
        let p = Formula::Predicate("P".to_owned(), Vec::new());
        let q = Formula::Predicate("Q".to_owned(), Vec::new());
        let neg_p = Formula::Not(box Formula::Predicate("P".to_owned(), Vec::new()));
        let cls = vec!(neg_p, p, q);
        assert!(trivial(&cls));
    }
}    