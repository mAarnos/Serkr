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
use std::collections::BinaryHeap;
use prover::flatten_cnf::flatten_cnf;
use prover::clause::Clause;
use prover::literal_deletion::simplify;
use prover::tautology_deletion::trivial;
use prover::subsumption::subsumes_clause;
use prover::term_ordering::TermOrdering;
use prover::lpo::LPO;
use prover::kbo::KBO;
use prover::superposition::superposition;
use prover::equality_resolution::equality_resolution;
use prover::equality_factoring::equality_factoring;
use utils::formula::Formula;
use utils::stopwatch::Stopwatch;
use cnf::naive_cnf::cnf;
use parser::internal_parser::parse;

/// Rename a clause so that it contains no variables in common with any other clause we currently have.
fn rename_clause(cl: &mut Clause, var_cnt: &mut i64) {
    let mut var_map = HashMap::<i64, i64>::new();
    for l in cl.iter_mut() {
        l.rename_no_common(&mut var_map, var_cnt);
    }
}

/// The main proof search loop.
fn serkr_loop<T: TermOrdering + ?Sized>(term_ordering: &T, 
                               mut unused: BinaryHeap<Clause>, 
                               mut var_cnt: i64) -> Result<bool, &'static str> {
    let mut sw = Stopwatch::new();
    let mut ms_count = 1000;
    let mut iterations = 0;
    let mut used = Vec::<Clause>::new();
    
    println!("Initial clauses: {}", unused.len());
    
    sw.start();
    
    while let Some(mut chosen_clause) = unused.pop() {
        if sw.elapsed_ms() > ms_count {
            println!("{} seconds have elapsed, iterations = {}, used = {}, unused = {}", sw.elapsed_ms() / 1000, iterations, used.len(), unused.len());
            ms_count += 1000;
        }
        
        // If we derived a contradiction we are done.
        if chosen_clause.is_empty() {
            return Ok(true);
        }
        
        if !used.iter().any(|cl| subsumes_clause(cl, &chosen_clause)) {
            // println!("Chosen clause: {:?}", chosen_clause);
            rename_clause(&mut chosen_clause, &mut var_cnt);
            
            let mut inferred_clauses = Vec::new();
            for cl in &used {
                superposition(term_ordering, &chosen_clause, cl, &mut inferred_clauses);
                superposition(term_ordering, cl, &chosen_clause, &mut inferred_clauses);
            }
            equality_resolution(term_ordering, &chosen_clause, &mut inferred_clauses);
            equality_factoring(term_ordering, &chosen_clause, &mut inferred_clauses);
            
            // Simplify the generated clauses.
            for x in &mut inferred_clauses {
                simplify(x);
            }
            
            // Forward subsumption.
            // inferred_clauses = inferred_clauses.into_iter().filter(|cl| !unused.iter().any(|cl2| subsumes_clause(cl2, cl))).collect();
            
            // Finally add everything to the queue.
            for x in inferred_clauses.into_iter() {
                unused.push(x);
            }
            used.push(chosen_clause); // Not quite sure if this should be here or before.
        }
        
        iterations += 1;
    }
    
    Err("No proof found.")
}

fn preprocess_clauses(clauses: Vec<Clause>) -> BinaryHeap<Clause> {
    // First get rid of duplicates.
    let mut new_clauses = clauses.into_iter().filter(|cl| !trivial(cl)).collect::<Vec<_>>();
    
    // Then simplify the clauses as much as possible.
    for cl in &mut new_clauses {
        simplify(cl);
    }
    
    // Remove subsumes clauses.
    let mut newer_clauses = Vec::new();
    while let Some(cl) = new_clauses.pop() {
        // Backward subsumption.
        let mut i = 0;
        while i < newer_clauses.len() {
            if subsumes_clause(&cl, &newer_clauses[i]) {
                newer_clauses.swap_remove(i);
                continue;
            }
            i += 1;
        }
        
        // Forward subsumption.
        let mut j = 0;
        while j < new_clauses.len() {
            if subsumes_clause(&cl, &new_clauses[j]) {
                new_clauses.swap_remove(j);
                continue;
            }
            j += 1;
        }
        
        newer_clauses.push(cl);
    }
    
    newer_clauses.into_iter().collect()
}

/// If the problem contains one unary function, this function finds it.
fn single_unary_function(clauses: &BinaryHeap<Clause>) -> Option<i64> {
    let mut found_unary = None;
    
    for cl in clauses {
        for l in cl.iter() {
            if l.get_lhs().get_arity() == 1 {
                if found_unary.is_some() {
                    return None;
                } else {
                    found_unary = Some(l.get_lhs().get_id());
                }
            }
            
            if l.get_rhs().get_arity() == 1 {
                if found_unary.is_some() {
                    return None;
                } else {
                    found_unary = Some(l.get_rhs().get_id());
                }
            }
        }
    }
    
    found_unary
}

fn create_term_ordering(lpo_over_kbo: bool, clauses: &BinaryHeap<Clause>) -> Box<TermOrdering> {
    if lpo_over_kbo {
        Box::new(LPO::new())
    } else {
        if let Some(unary_func) = single_unary_function(&clauses) {
            Box::new(KBO::new(true, unary_func))
        } else {
            Box::new(KBO::new(false, 0))
        } 
    }
}

/// Attempts to prove the FOL formula passed in.
pub fn prove(s: &str) -> Result<bool, &'static str> {
    let cnf_f = cnf(Formula::Not(Box::new(parse(s).unwrap())));
    if cnf_f == Formula::False {
        Ok(true)
    } else if cnf_f == Formula::True {
        Ok(false)
    } else {
        let (flattened_cnf_f, renaming_info) = flatten_cnf(cnf_f);
        let preprocessed_problem = preprocess_clauses(flattened_cnf_f);                
        let term_ordering = create_term_ordering(true, &preprocessed_problem);
        serkr_loop(&*term_ordering, preprocessed_problem, renaming_info.var_cnt)
    }
}

#[cfg(test)]
mod test {
    use super::prove;
    
    // Contains some problems negated so that we can make sure we don't prove a theorem not actually provable.
    
    #[test]
    fn pelletier_1() {
        let result = prove("(P ==> Q) <=> (~Q ==> ~P)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_1_negated() {
        let result = prove("~((P ==> Q) <=> (~Q ==> ~P))");
        assert!(result.is_err());
    }
    
    #[test]
    fn pelletier_2() {
        let result = prove("~~P <=> P");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_3() {
        let result = prove("~(P ==> Q) ==> (Q ==> P)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_4() {
        let result = prove("(~P ==> Q) <=> (~Q ==> P)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_5() {
        let result = prove("((P \\/ Q) ==> (P \\/ R)) ==> (P \\/ (Q ==> R))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_6() {
        let result = prove("P \\/ ~P");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_7() {
        let result = prove("P \\/ ~~~P");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_8() {
        let result = prove("((P ==> Q) ==> P) ==> P");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_8_negated() {
        let result = prove("~(((P ==> Q) ==> P) ==> P)");
        assert!(result.is_err());
    }
    
    #[test]
    fn pelletier_9() {
        let result = prove("(P \\/ Q) /\\ (~P \\/ Q) /\\ (P \\/ ~Q) ==> ~(~P \\/ ~Q)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_10() {
        let result = prove("(Q ==> R) /\\ (R ==> P /\\ Q) /\\ (P ==> Q \\/ R) ==> (P <=> Q)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_11() {
        let result = prove("P <=> P");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_12() {
        let result = prove("((P <=> Q) <=> R) <=> (P <=> (Q <=> R))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_13() {
        let result = prove("(P \\/ Q /\\ R) <=> ((P \\/ Q) /\\ (P \\/ R))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_14() {
        let result = prove("(P <=> Q) <=> ((~P \\/ Q) /\\ (~Q \\/ P))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_15() {
        let result = prove("(P ==> Q) <=> (~Q \\/ Q)");
        assert!(result.is_err());
    }
    
    #[test]
    fn pelletier_15_errata() {
        let result = prove("(P ==> Q) <=> (~P \\/ Q)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_16() {
        let result = prove("(P ==> Q) \\/ (Q ==> P)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_17() {
        let result = prove("(P /\\ (Q ==> R) ==> S) <=> ((~P \\/ Q \\/ S) /\\ (~P \\/ ~R \\/ S))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_18() {
        let result = prove("exists y. forall x. (F(y) ==> F(x))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_18_negated() {
        let result = prove("~exists y. forall x. (F(y) ==> F(x))");
        assert!(result.is_err());
    }
    
    #[test]
    fn pelletier_19() {
        let result = prove("exists x. forall y. forall z. ((P(y) ==> Q(z)) ==> (P(x) ==> Q(x)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_20() {
        let result = prove("forall x. forall y. exists z. forall w. (P(x) /\\ Q(y) ==> R(z) /\\ S(w))
                            ==> (exists x. exists y. (P(x) /\\ Q(y)) ==> exists z. R(z))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_21() {
        let result = prove("exists x. (P ==> F(x)) /\\ 
                            exists x. (F(x) ==> P) 
                            ==> exists x. (P <=> F(x))");
        assert!(result.is_ok());
    }

    #[test]
    fn pelletier_22() {
        let result = prove("forall x. (P <=> F(x)) ==> (P <=> forall x. F(x))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_23() {
        let result = prove("forall x. (P \\/ F(x)) <=> P \\/ forall x. F(x)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_24() {
        let result = prove("~exists x. (S(x) /\\ Q(x)) /\\
                            forall x. (P(x) ==> Q(x) \\/ R(x)) /\\
                            (~exists x. P(x) ==> exists x. Q(x)) /\\
                            forall x. (Q(x) \\/ R(x) ==> S(x))
                            ==> exists x. (P(x) /\\ R(x))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_25() {
        let result = prove("exists x. P(x) /\\ 
                            forall x. (F(x) ==> ~G(x) /\\ R(x)) /\\
                            forall x. (P(x) ==> G(x) /\\ F(x)) /\\
                           (forall x. (P(x) ==> Q(x)) \\/ exists x. (P(x) /\\ R(x)))
                            ==> exists x. (Q(x) /\\ P(x))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_26() {
        let result = prove("(exists x. P(x) <=> exists x. Q(x)) /\\
                            forall x. forall y. (P(x) /\\ Q(y) ==> (R(x) <=> S(y)))
                            ==> (forall x. (P(x) ==> R(x)) <=> forall x. (Q(x) ==> S(x)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_27() {
        let result = prove("exists x. (F(x) /\\ ~G(x)) /\\ 
                            forall x. (F(x) ==> H(x)) /\\ 
                            forall x. (J(x) /\\ I(x) ==> F(x)) /\\ 
                            (exists x. (H(x) /\\ ~G(x)) ==> forall x. (I(x) ==> ~H(x)))
                            ==> forall x. (J(x) ==> ~I(x))");
        assert!(result.is_ok());
    }

    #[test]
    fn pelletier_28_orig() {
        let result = prove("(forall x. P(x) ==> forall x. Q(x)) /\\
                            (forall x. (Q(x) \\/ R(x)) ==> exists x. (Q(x) /\\ S(x))) /\\
                            (exists x. S(x) ==> forall x. (F(x) ==> G(x))) 
                             ==> forall x. (P(x) /\\ F(x) ==> G(x))");
        assert!(result.is_err());
    }
    
    #[test]
    fn pelletier_28_errata() {
        let result = prove("forall x. (P(x) ==> forall x. Q(x)) /\\
                           (forall x. (Q(x) \\/ R(x)) ==> exists x. (Q(x) /\\ S(x))) /\\
                           (exists x. S(x) ==> forall x. (F(x) ==> G(x)))
                            ==> forall x. (P(x) /\\ F(x) ==> G(x))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_29() {
        let result = prove("(exists x. F(x) /\\ exists x. G(x))
                             ==> ((forall x. (F(x) ==> H(x)) /\\ forall x. (G(x) ==> J(x))) <=> forall x. forall y. (F(x) /\\ G(y) ==> H(x) /\\ J(y)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_30() {
        let result = prove("forall x. (F(x) \\/ G(x) ==> ~H(x)) /\\ 
                            forall x. ((G(x) ==> ~I(x)) ==> F(x) /\\ H(x)) 
                            ==> forall x. I(x)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_30_negated() {
        let result = prove("~(forall x. (F(x) \\/ G(x) ==> ~H(x)) /\\ forall x. ((G(x) ==> ~I(x)) ==> F(x) /\\ H(x)) ==> forall x. I(x))");
        assert!(result.is_err());
    }
    
    #[test]
    fn pelletier_31() {
        let result = prove("~exists x. (F(x) /\\ (G(x) \\/ H(x))) /\\ 
                            exists x. (I(x) /\\ F(x)) /\\ 
                            forall x. (~H(x) ==> J(x))
                            ==> exists x. (I(x) /\\ J(x))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_32() {
        let result = prove("forall x. ((F(x) /\\ (G(x) \\/ H(x))) ==> I(x)) /\\ 
                            forall x. (I(x) /\\ (H(x) ==> J(x))) /\\ 
                            forall x. (K(x) ==> H(x))
                            ==> forall x. (F(x) /\\ K(x) ==> J(x))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_33() {
        let result = prove("forall x. (P(a) /\\ (P(x) ==> P(b)) ==> P(c)) <=>
                            forall x. ((~P(a) \\/ P(x) \\/ P(c)) /\\ (~P(a) \\/ ~P(b) \\/ P(c)))");
        assert!(result.is_ok());
    }
    
    /*
    #[test]
    fn pelletier_34() {
        let result = prove("((exists x. forall y. P(x) <=> P(y)) <=>
                             (exists x. Q(x) <=> forall y. Q(y))) <=>
                            ((exists x. forall y. Q(x) <=> Q(y)) <=>
                             (exists x. P(x) <=> forall y. P(y)))");
        assert!(result.is_ok());
    }
    */

    #[test]
    fn pelletier_35() {
        let result = prove("exists x. exists y. (P(x, y) ==> forall x. forall y. P(x, y))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_35_negated() {
        let result = prove("~exists x. exists y. (P(x, y) ==> forall x. forall y. P(x, y))");
        assert!(result.is_err());
    }
    
    #[test]
    fn pelletier_36() {
        let result = prove("forall x. exists y. F(x, y) /\\ 
                            forall x. exists y. G(x, y) /\\ 
                            forall x. forall y. (F(x, y) \\/ G(x, y) ==> forall z. (F(y, z) \\/ G(y, z) ==> H(x, z)))
                            ==> forall x. exists y. H(x, y)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_37() {
        let result = prove("forall z. exists w. forall x. exists y. ((P(x, z) ==> P(y, w)) /\\ P(y, z) /\\ (P(y, w) ==> exists u. Q(u, w))) /\\
                            forall x. forall z. (~P(x, z) ==> exists y. Q(y, z)) /\\
                            (exists x. exists y. Q(x, y) ==> forall x. R(x, x))
                            ==> forall x. exists y. R(x, y)");
        assert!(result.is_ok());
    }

    /*
    #[test]
    fn pelletier_38() {
        let result = prove("forall x. (P(a()) /\\ (P(x) ==> exists y. (P(y) /\\ R(x, y))) ==> exists z. exists w. (P(z) /\\ R(x, w) /\\ R(w, z))) <=>
                            forall x. ((~P(a()) \\/ P(x) \\/ exists z. exists w. (P(z) /\\ R(x, w) /\\ R(w, z))) /\\
                            (~P(a()) \\/ ~exists y. (P(y) /\\ R(x, y)) \\/ exists z. exists w. (P(z) /\\ R(x, w) /\\ R(w, z))))");
        assert!(result.is_ok());
    }
    */
    
    #[test]
    fn pelletier_39() {
        let result = prove("~exists x. forall y. (F(y, x) <=> ~F(y, y))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_40() {
        let result = prove("exists y. forall x. (F(x, y) <=> F(x, x)) 
                            ==> ~forall x. exists y. forall z. (F(x, y) <=> ~F(z, x))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_40_errata() {
        let result = prove("exists y. forall x. (F(x, y) <=> F(x, x)) 
                            ==> ~forall x. exists y. forall z. (F(z, y) <=> ~F(z, x))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_41() {
        let result = prove("forall z. exists y. forall x. (F(x, y) <=> F(x, z) /\\ ~F(x, x)) 
                            ==> ~exists z. forall x. F(x, z)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_42() {
        let result = prove("~exists y. forall x. (F(x, y) <=> ~exists z. (F(x, z) /\\ F(z, x)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_42_negated() {
        let result = prove("exists y. forall x. (F(x, y) <=> ~exists z. (F(x, z) /\\ F(z, x)))");
        assert!(result.is_err());
    }

    #[test]
    fn pelletier_43() {
        let result = prove("forall x. forall y. (Q(x, y) <=> forall z. (F(z, x) <=> F(z, y)))
                            ==> forall x. forall y. (Q(x, y) <=> Q(y, x))");
        assert!(result.is_ok());
    }

    #[test]
    fn pelletier_44() {
        let result = prove("forall x. (F(x) ==> exists y. (G(y) /\\ H(x, y)) /\\ exists y. (G(y) /\\ ~H(x, y))) /\\ 
                            exists x. (J(x) /\\ forall y. (G(y) ==> H(x, y)))
                            ==> exists x. (J(x) /\\ ~F(x))");
        assert!(result.is_ok());
    }   

    #[test]
    fn pelletier_45() {
        let result = prove("forall x. (F(x) /\\ forall y. (G(y) /\\ H(x, y) ==> J(x, y)) ==> forall y. (G(y) /\\ H(x, y) ==> K(y))) /\\ 
                            ~exists y. (L(y) /\\ K(y)) /\\ 
                            exists x. (F(x) /\\ forall y. (H(x, y) ==> L(y)) /\\ forall y. (G(y) /\\ H(x, y) ==> J(x, y)))
                            ==> exists x. (F(x) /\\ ~exists y. (G(y) /\\ H(x, y)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_46() {
        let result = prove("forall x. (F(x) /\\ forall y. (F(y) /\\ H(y, x) ==> G(y)) ==> G(x)) /\\
                           (exists x. (F(x) /\\ ~G(x)) ==> exists x. (F(x) /\\ ~G(x) /\\ forall y. (F(y) /\\ ~G(y) ==> J(x, y)))) /\\
                            forall x. forall y. (F(x) /\\ F(y) /\\ H(x, y) ==> ~J(y, x))
                            ==> forall x. (F(x) ==> G(x))");
        assert!(result.is_ok());
    }
    
    /*
    #[test]
    fn pelletier_47() {
        let result = prove("(((((((((((((((forall x. ((P1(x) ==> P0(x)) /\\ exists x. P1(x))) /\\ 
                                          (forall x. ((P2(x) ==> P0(x)) /\\ exists x. P2(x)))) /\\ 
                                          (forall x. ((P3(x) ==> P0(x)) /\\ exists x. P3(x)))) /\\
                                          (forall x. ((P4(x) ==> P0(x)) /\\ exists x. P4(x)))) /\\
                                          (forall x. ((P5(x) ==> P0(x)) /\\ exists x. P5(x)))) /\\ 
                                          (exists x. Q1(x) /\\ forall x. (Q1(x) ==> Q0(x)))) /\\
                                          (forall x. (P0(x) ==> ((forall y. Q0(y) ==> R(x, y)) \\/ forall y. (((P0(y) /\\ S(y, x)) /\\ exists z. (Q0(z) /\\ R(y, z))) ==> R(x, y)))))) /\\
                                          (forall x. forall y. ((P3(y) /\\ (P5(x) \\/ P4(x))) ==> S(x, y)))) /\\ 
                                          (forall x. forall y. ((P3(x) /\\ P2(y)) ==> S(x, y)))) /\\ 
                                          (forall x. forall y. ((P2(x) /\\ P1(y)) ==> S(x, y)))) /\\
                                          (forall x. forall y. ((P1(x) /\\ (P2(y) \\/ Q1(y))) ==> ~R(x, y)))) /\\ 
                                          (forall x. forall y. ((P3(x) /\\ P4(y)) ==> R(x, y)))) /\\ 
                                          (forall x. forall y. ((P3(x) /\\ P5(y)) ==> ~R(x, y)))) /\\
                                          (forall x. ((P4(x) \\/ P5(x)) ==> exists y. (Q0(y) /\\ R(x, y)))))
                                           ==> exists x. exists y. ((P0(x) /\\ P0(y)) /\\ exists z. (Q1(z) /\\ (R(y, z) /\\ R(x, y)))))");
        assert!(result.is_ok());
    }
    */
    
    #[test]
    fn pelletier_48() {
        let result = prove("(a() = b() \\/ c() = d()) /\\ 
                            (a() = c() \\/ b() = d()) 
                            ==> a() = d() \\/ b() = c()");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_49() {
        let result = prove("((exists x. exists y. forall z. (z = x \\/ z = y) /\\ ((P(a()) /\\ P(b)) /\\ a() <> b()))
                              ==> forall x. P(x))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_50() {
        let result = prove("forall x. (F(a(), x) \\/ forall y. F(x, y)) ==> exists x. forall y. F(x, y)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_50_negated() {
        let result = prove("~(forall x. (F(a(), x) \\/ forall y. F(x, y)) ==> ~exists x. forall y. F(x, y))");
        assert!(result.is_err());
    }
    
    #[test]
    fn pelletier_51() {
        let result = prove("exists z. exists w. forall x. forall y. (F(x, y) <=> x = z /\\ y = w)
                            ==> exists z. forall x. ((exists w. forall y. (F(x, y) <=> y = w)) <=> x = z)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_52() {
        let result = prove("exists z. exists w. forall x. forall y. (F(x, y) <=> x = z /\\ y = w)
                             ==> exists w. forall y. ((exists z. forall x. (F(x, y) <=> x = z)) <=> y = w)");
        assert!(result.is_ok());
    }
  
    /*
    #[test]
    fn pelletier_53() {
        let result = prove("((exists x. exists y. (x <> y /\\ forall z. (z = x \\/ z = y))) ==>
                           (((exists z. forall x. ((exists w. forall y. (F(x, y) <=> y = w)) <=> x = z)))
                            <=> (exists w. forall y. ((exists z. forall x. (F(x, y) <=> x = z)) <=> y = w))))");
        assert!(result.is_ok());
    }
    */
    
    #[test]
    fn pelletier_54() {
        let result = prove("forall y. exists z. forall x. (F(x, z) <=> x = y) 
                            ==> ~exists w. forall x. (F(x, w) <=> forall u. (F(x, u) ==> exists y. (F(y, u) /\\ ~exists z. (F(z, u) /\\ F(z, y)))))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_55() {
        let result = prove("exists x. (L(x) /\\ K(x, a())) /\\ 
                            L(a()) /\\ L(b()) /\\ L(c()) /\\ 
                            forall x. (L(x) ==> (x = a() \\/ (x = b() \\/ x = c()))) /\\ 
                            forall x. forall y. (K(x, y) ==> H(x, y)) /\\ 
                            forall x. forall y. (K(x, y) ==> ~R(x, y)) /\\ 
                            forall x. (H(a(), x) ==> ~H(c(), x)) /\\ 
                            forall x. (x <> b() ==> H(a(), x)) /\\ 
                            forall x. (~R(x, a()) ==> H(b(), x)) /\\ 
                            forall x. (H(a(), x) ==> H(b(), x)) /\\ 
                            forall x. exists y. ~H(x, y) /\\ 
                            a() <> b()
                            ==> K(a(), a())"); 
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_56() {
        let result = prove("forall x. (exists y. (F(y) /\\ x = f(y)) ==> F(x)) <=> forall x. (F(x) ==> F(f(x)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_57() {
        let result = prove("F(f(a(), b()), f(b(), c())) /\\ F(f(b(), c()), f(a(), c())) /\\
                            forall x. forall y. forall z. (F(x, y) /\\ F(y, z) ==> F(x, z)) 
                            ==> F(f(a(), b()), f(a(), c()))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_58() {
        let result = prove("forall x. forall y. f(x) = g(y) ==> forall x. forall y. f(f(x)) = f(g(y))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_59() {
        let result = prove("forall x. (F(x) <=> ~F(f(x))) ==> exists x. (F(x) /\\ ~F(f(x)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_60() {
        let result = prove("forall x. (F(x, f(x)) <=> exists y. (forall z. (F(z, y) ==> F(z, f(x))) /\\ F(x, y)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_61() {
        let result = prove("forall x. forall y. forall z. f(x, f(y, z)) = f(f(x, y), z)
                            ==> forall x. forall y. forall z. forall w. f(x, f(y, f(z, w))) = f(f(f(x, y), z), w)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_62_errata_errata() {
        let result = prove("forall x. (F(a()) /\\ (F(x) ==> F(f(x))) ==> F(f(f(x)))) <=>
                            forall x. ((~F(a()) \\/ F(x) \\/ F(f(f(x)))) /\\ (~F(a()) \\/ ~F(f(x)) \\/ F(f(f(x)))))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_63() {
        let result = prove("forall x. forall y. forall z. f(f(x, y), z) = f(x, f(y, z)) /\\ 
                            forall x. f(a(), x) = x /\\ 
                            forall x. exists y. f(y, x) = a()
                            ==> forall x. forall y. forall z. (f(x, y) = f(z, y) ==> x = z)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_64() {
        let result = prove("forall x. forall y. forall z. f(f(x, y), z) = f(x, f(y, z)) /\\ 
                            forall x. f(a(), x) = x /\\ 
                            forall x. exists y. f(y, x) = a()
                            ==> forall x. forall y. (f(y, x) = a() ==> f(x, y) = a())");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_65() {
        let result = prove("forall x. forall y. forall z. f(f(x, y), z) = f(x, f(y, z)) /\\ 
                            forall x. f(a(), x) = x
                            ==> (forall x. f(x, x) = a() ==> forall x. forall y. f(x, y) = f(y, x))");
        assert!(result.is_ok());
    }

    #[test]
    fn los() {
        let result = prove("forall x. forall y. forall z. (P(x, y) /\\ P(y, z) ==> P(x, z)) /\\
                            forall x. forall y. forall z. (Q(x, y) /\\ Q(y, z) ==> Q(x, z)) /\\
                            forall x. forall y. (Q(x, y) ==> Q(y, x)) /\\
                            forall x. forall y. (P(x, y) \\/ Q(x, y))
                            ==> forall x. forall y. P(x, y) \\/ forall x. forall y. Q(x, y)");
        assert!(result.is_ok());
    }
       
    #[test]
    fn davis_putnam() {
        let result = prove("exists x. exists y. forall z. ((F(x, y) ==> F(y, z) /\\ F(z, z)) /\\ (F(x, y) /\\ G(x, y) ==> G(x, z) /\\ G(z, z)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn djikstra() {
        let result = prove("forall x. f(f(x)) = f(x) /\\ forall x. exists y. f(y) = x 
                            ==> forall x. f(x) = x");
        assert!(result.is_ok());
    }
    
    #[test]
    fn djikstra_negated() {
        let result = prove("~(forall x. f(f(x)) = f(x) /\\ forall x. exists y. f(y) = x 
                              ==> forall x. f(x) = x)");
        assert!(result.is_err());
    }
    
    #[test]
    fn ewd() {
        let result = prove("forall x. (F(x) ==> G(x)) /\\ 
                            exists x. F(x) /\\
                            forall x. forall y. (G(x) /\\ G(y) ==> x = y)
                            ==> forall y. (G(y) ==> F(y))");
        assert!(result.is_ok());
    }
    
    /*
    #[test]
    fn wishnu() {
        let result = prove("exists x. (x = f(g(x)) /\\ forall x1. (x1 = f(g(x1)) ==> x = x1)) <=>
                            exists y. (y = g(f(y)) /\\ forall y1. (y1 = g(f(y1)) ==> y = y1))");
        assert!(result.is_ok());
    }
    */
    
    #[test]
    fn group_x_times_x_equals_1_abelian() {
        let result = prove("forall x. forall y. forall z. mult(x, mult(y, z)) = mult(mult(x, y), z) /\\
                            forall x. mult(i(), x) = x /\\
                            forall x. mult(i(x), x) = i()
                            ==> forall x. mult(x, i()) = x");
        assert!(result.is_ok());
    }
} 
