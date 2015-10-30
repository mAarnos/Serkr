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
use prover::unification::mgu;
use prover::term::Term;
use prover::literal::Literal;
use prover::clause::Clause;
use prover::trivial::trivial;
use prover::factoring::factor;
use prover::flatten_cnf::flatten_cnf;
use prover::subsumption::subsumes_clause;
use prover::duplicate_deletion::delete_duplicates;
use parser::internal_parser::parse;
use cnf::naive_cnf::cnf;
use utils::formula::Formula;

fn rename(pfx: String, cl: &mut Clause) {
    let fvs: Set<String> = cl.iter().flat_map(|l| l.variables()).collect();
    let mut mapping = HashMap::<Term, Term>::new();
    for x in fvs.into_iter() {
        mapping.insert(Term::Variable(x.clone()), Term::Variable(pfx.clone() + &x)); 
    }
    for f in cl.iter_mut() {
        f.tsubst(&mapping);
    }
}

fn add_resolvents(cl1: &Clause, cl2: &Clause, p: Literal, used: &[Clause], unused: &mut Vec<Clause>) {
    let neg_p = p.negate();
    for x in cl2.iter().cloned() {
        let possible_theta = mgu(x.clone(), neg_p.clone());
        if let Ok(theta) = possible_theta {
            let mut cl1_done = Clause::new_from_vec(cl1.iter()
                                                       .cloned()
                                                       .filter(|l| *l != p)
                                                       .map(|mut l| { l.tsubst(&theta); l })
                                                       .collect());
            let cl2_done = Clause::new_from_vec(cl2.iter()
                                                   .cloned()
                                                   .filter(|l| *l != x)
                                                   .map(|mut l| { l.tsubst(&theta); l })
                                                   .collect());                                   
            cl1_done.add_literals(cl2_done);
            delete_duplicates(&mut cl1_done);
            if !trivial(&cl1_done) && !used.iter().any(|cl| subsumes_clause(cl, &cl1_done)) && !unused.iter().any(|cl| subsumes_clause(cl, &cl1_done)) {
                unused.push(cl1_done);
            }
        }
    }
}

fn resolve_clauses(mut cl1: Clause, mut cl2: Clause, used: &[Clause], unused: &mut Vec<Clause>) {
    // Positive resolution: one of the resolution clauses must be all-positive.
    if cl1.iter().all(|l| l.is_positive()) || cl2.iter().all(|l| l.is_positive()) {
        rename("x".to_owned(), &mut cl1);
        rename("y".to_owned(), &mut cl2);
        for p in cl1.iter().cloned() {
            add_resolvents(&cl1, &cl2, p, used, unused);
        }
    }
}

/// Picks and removes the best clause from the unused clauses according to heuristics.
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

fn resolution_loop(mut used: Vec<Clause>, mut unused: Vec<Clause>) -> Result<bool, &'static str> {
    while !unused.is_empty() {
        println!("Used: {} Unused: {}", used.len(), unused.len());
        let chosen_clause = pick_clause(&mut unused);
        // If we derived a contradiction we are done.
        if chosen_clause.is_empty() {
            return Ok(true);
        }
        used.push(chosen_clause.clone());
        
        for cl in &used {
            resolve_clauses(chosen_clause.clone(), cl.clone(), &used, &mut unused);
        }
        factor(chosen_clause, &mut unused);
    }
    Err("No proof found.")
}

/// Attempts to parse and prove the string passed in.
pub fn resolution(s: &str) -> Result<bool, &'static str> {
    let cnf_f = cnf(Formula::Not(box parse(s).unwrap()));
    if cnf_f == Formula::False {
        Ok(true)
    } else if cnf_f == Formula::True {
        Ok(false)
    } else {
        resolution_loop(Vec::new(), flatten_cnf(cnf_f).into_iter().filter(|cl| !trivial(cl)).collect())
    }
}

#[cfg(test)]
mod test {
    use super::resolution;
    
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
    
    #[test]
    fn pelletier_10() {
        let result = resolution("(((((Q ==> R) /\\ (R ==> (P /\\ Q))) /\\ (P ==> (Q \\/ R)))) ==> (P <=> Q))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_11() {
        let result = resolution("(P <=> P)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_12() {
        let result = resolution("(((P <=> Q) <=> R) <=> (P <=> (Q <=> R)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_13() {
        let result = resolution("((P \\/ (Q /\\ R)) <=> ((P \\/ Q) /\\ (P \\/ R)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_14() {
        let result = resolution("((P <=> Q) <=> ((~P \\/ Q) /\\ (~Q \\/ P)))");
        assert!(result.is_ok());
    }
    
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
    fn pelletier_17() {
        let result = resolution("(((P /\\ (Q ==> R)) ==> S) <=>
                                  (((~P \\/ Q) \\/ S) /\\
                                  (~P \\/ (~R \\/ S))))");
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
    
    #[test]
    fn pelletier_21() {
        let result = resolution("(((exists x. (P ==> F(x))) /\\ 
                                   (exists x. (F(x) ==> P))) 
                                    ==> (exists x. (P <=> F(x))))");
        assert!(result.is_ok());
    }

    #[test]
    fn pelletier_22() {
        let result = resolution("((forall x. (P <=> F(x))) ==> (P <=> forall x. F(x)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_23() {
        let result = resolution("((forall x. (P \\/ F(x))) <=> (P \\/ forall x. F(x)))");
        assert!(result.is_ok());
    }
    
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
    
    /*
    #[test]
    fn pelletier_40() {
        let result = resolution("((exists y. forall x. (F(x, y) <=> F(x, x))) ==>
                                  ~forall x. exists y. forall z. (F(x, y) <=> ~F(z, x)))");
        assert!(result.is_ok());
    }
    */
    
    #[test]
    fn pelletier_42() {
        let result = resolution("~exists y. forall x. (F(x, y) <=> ~exists z. (F(x, z) /\\ F(z, x)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn davis_putnam() {
        let result = resolution("exists x. exists y. forall z. ((F(x, y) ==> (F(y, z) /\\ F(z, z))) /\\ ((F(x, y) /\\ G(x, y)) ==> (G(x, z) /\\ G(z, z))))");
        assert!(result.is_ok());
    }
}    