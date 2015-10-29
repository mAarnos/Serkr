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
    let mut mapping = HashMap::<Term, Term>::new();
    for x in fvs.into_iter() {
        mapping.insert(Term::Variable(x.clone()), Term::Variable(pfx.clone() + &x)); 
    }
    for f in cl.iter_mut() {
        *f = tsubst(f.clone(), &mapping);
    }
}

fn add_resolvents(cl1: &Clause, cl2: &Clause, p: Formula, unused: &mut Vec<Clause>) {
    let neg_p = negate(p.clone());
    for x in cl2.iter().cloned() {
        let possible_theta = mgu(x.clone(), neg_p.clone());
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

fn resolve_clauses(mut cl1: Clause, mut cl2: Clause, unused: &mut Vec<Clause>) {
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
fn pick_clause(unused: &mut Vec<Clause>) -> Clause {
    // TODO: can be done better by using max.
    // TODO: can be done even better with a priority queue
    let mut best_clause_index = 0;
    let mut best_clause_size = unused[0].size();
    
    for i in 1..unused.len() {
        if unused[i].size() < best_clause_size {
            best_clause_index = i;
            best_clause_size = unused[i].size();
        }
    }
    
    unused.swap_remove(best_clause_index)
}

/// Factors a clause if possible. 
#[allow(needless_range_loop)]
fn factor(cl: Vec<Formula>, unused: &mut Vec<Vec<Formula>>) {
    for i in 0..cl.len() {
        for j in (i + 1)..cl.len() {
            if let Ok(theta) = mgu(cl[i].clone(), cl[j].clone()) {
                let mut new_cl = cl.clone();
                new_cl.swap_remove(j);
                for l in &mut new_cl {
                    *l = tsubst(l.clone(), &theta);
                }
                unused.push(new_cl);
            }
        }
    }
}

fn resolution_loop(mut used: Vec<Clause>, mut unused: Vec<Clause>) -> Result<bool, &'static str> {
    while !unused.is_empty() {
        let chosen_clause = pick_clause(&mut unused);
        // If we derived a contradiction we are done.
        if chosen_clause.is_empty() {
            return Ok(true);
        }
        used.push(chosen_clause.clone());
        
        for cl in &used {
            resolve_clauses(chosen_clause.clone(), cl.clone(), &mut unused);
        }
        factor(chosen_clause, &mut unused);
    }
    Err("No proof found.")
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
    
    #[test]
    fn pelletier_15() {
        let result = resolution("((P ==> Q) <=> (~P \\/ Q))");
        assert!(result.is_ok());
    }
    
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
   fn pelletier_30() {
        let result = resolution("(((forall x. ((F(x) \\/ G(x)) ==> ~H(x))) /\\ (forall x. ((G(x) ==> ~I(x)) ==> (F(x) /\\ H(x))))) ==> (forall x. I(x)))");
        assert!(result.is_ok());
    }

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
    
    #[test]
    fn pelletier_42() {
        let result = resolution("~exists y. forall x. (F(x, y) <=> ~exists z. (F(x, z) /\\ F(z, x)))");
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