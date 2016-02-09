/*
    Serkr - An automated theorem prover. Copyright (C) 2015-2016 Mikko Aarnos.

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
use prover::flatten_cnf::flatten_cnf;

use prover::data_structures::term::Term;
use prover::data_structures::clause::Clause;

use prover::proof_statistics::ProofStatistics;
use prover::proof_state::ProofState;

use prover::simplification::literal_deletion::cheap_simplify;
use prover::simplification::tautology_deletion::trivial;
use prover::simplification::subsumption::subsumes_clause;
use prover::simplification::equality_subsumption::equality_subsumes_clause;
use prover::simplification::simplify_reflect::simplify_reflect;
use prover::simplification::rewriting::rewrite_literals;

use prover::ordering::precedence::Precedence;
use prover::ordering::weight::Weight;
use prover::ordering::term_ordering::TermOrdering;

use prover::inference::equality_resolution::equality_resolution;
use prover::inference::equality_factoring::equality_factoring;
use prover::inference::superposition::superposition;

use cnf::ast::Formula;
use cnf::ast_transformer_internal::internal_to_cnf_ast;
use cnf::ast_transformer_tptp::tptp_to_cnf_ast;
use cnf::naive_cnf::cnf;
use utils::stopwatch::Stopwatch;

/// Contains the result of a proof attempt.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[allow(missing_docs)]
pub enum ProofAttemptResult {
    Refutation,
    Saturation,
    Timeout,
    GaveUp,
    Error,
}

/// Rename a clause so that it contains no variables in common with any other clause we currently have.
fn rename_clause(cl: &mut Clause, var_cnt: &mut i64) {
    let mut var_map = HashMap::<i64, i64>::new();
    for l in cl.iter_mut() {
        l.get_lhs_mut().rename_no_common(&mut var_map, var_cnt);
        l.get_rhs_mut().rename_no_common(&mut var_map, var_cnt);
    }
}

/// Checks if a given clause is subsumed by some clause in a given set of clauses.
fn forward_subsumed(cl: &Clause, clauses: &[Clause]) -> bool {
    clauses.iter().any(|cl2| equality_subsumes_clause(cl2, cl) || subsumes_clause(cl2, cl))
} 

/// Removes all clauses from a given set subsumed by a given clause.
/// Returns the amount of back subsumed clauses.
fn backward_subsumption(cl: &Clause, clauses: &mut Vec<Clause>) -> u64 {
    let mut i = 0;
    let mut bs_count = 0;
    
    while i < clauses.len() {
        if equality_subsumes_clause(&cl, &clauses[i]) || subsumes_clause(&cl, &clauses[i]) {
            clauses.swap_remove(i);
            bs_count += 1;
            continue;
        }
        i += 1;
    }
    
    bs_count
}

/// A more expensive version of cheap_simplify with more effective rules.
fn simplify(term_ordering: &TermOrdering, cl: &mut Clause, clauses: &[Clause]) {
    for cl2 in clauses {
        simplify_reflect(cl2, cl);
    }
    rewrite_literals(term_ordering, clauses, cl);
}

/// The main proof search loop.
/// Note that we use the DISCOUNT version of the given clause algorithm.
fn serkr_loop(mut proof_state: ProofState, mut var_cnt: i64, max_time_in_ms: u64) -> (ProofAttemptResult, ProofStatistics) {
    let mut sw = Stopwatch::new();
    let mut stats = ProofStatistics::new();
    let mut ms_count = 1000;
    let mut added_to_unused = 0;
    
    println!("Initial clauses: {}", proof_state.get_unused_size());
    
    sw.start();
    
    while let Some(mut chosen_clause) = proof_state.pick_best_clause() {
        stats.iterations += 1;
        stats.elapsed_ms = sw.elapsed_ms();
        
        // If we have spent too much time already just give up.
        if stats.elapsed_ms > max_time_in_ms {
            return (ProofAttemptResult::Timeout, stats);
        }
        
        if stats.elapsed_ms > ms_count {
            println!("info time {} iterations {} used {} unused {} sp {} ef {} er {} trivial {} fs {} bs {}", stats.elapsed_ms, 
                                                                                                              stats.iterations, 
                                                                                                              proof_state.get_used_size(), 
                                                                                                              proof_state.get_unused_size(), 
                                                                                                              stats.sp_count,
                                                                                                              stats.ef_count,
                                                                                                              stats.er_count,
                                                                                                              stats.trivial_count,
                                                                                                              stats.fs_count,
                                                                                                              stats.bs_count);
            ms_count += 1000;
        }
        
        // We start by simplifying the chosen clause as much as possible.
        simplify(proof_state.get_term_ordering(), &mut chosen_clause, proof_state.get_used());
        
        // If we derived a contradiction we are done.
        if chosen_clause.is_empty() {
            return (ProofAttemptResult::Refutation, stats);
        }
        
        // Check if the clause is redundant in some way. If it is no need to process it more.
        if !forward_subsumed(&chosen_clause, proof_state.get_used()) {
            stats.bs_count += backward_subsumption(&chosen_clause, proof_state.get_used_mut());
        
            rename_clause(&mut chosen_clause, &mut var_cnt);
            
            let mut inferred_clauses = Vec::new();
            stats.sp_count += superposition(proof_state.get_term_ordering(), &chosen_clause, proof_state.get_used(), &mut inferred_clauses);
            stats.er_count += equality_resolution(proof_state.get_term_ordering(), &chosen_clause, &mut inferred_clauses);
            stats.ef_count += equality_factoring(proof_state.get_term_ordering(), &chosen_clause, &mut inferred_clauses);

            for mut cl in inferred_clauses.into_iter() {
                // Simplification need to be done before triviality checking.
                // Consider the clause x <> y, y <> z, x = z which is clearly a tautology.
                // We cannot detect it as a tautology with a pure syntactical check unless we first simplify it with destructive equality resolution.
                cheap_simplify(&mut cl);
                if !trivial(&cl) {
                    // Give a unique ID to the clause.
                    cl.set_id(added_to_unused);
                    added_to_unused += 1;
                    proof_state.add_to_unused(cl);
                } else {
                    stats.trivial_count += 1;
                }
            }
            
            // Not quite sure if this should be here or before. Doesn't _seem_ incomplete due to this.
            proof_state.add_to_used(chosen_clause); 
        } else {
            stats.fs_count += 1;
        }
    }
    
    (ProofAttemptResult::Saturation, stats)
}

/// Try to simplify, delete tautologies and remove subsumed clauses from those clauses passed in.
/// Possibly very costly due to subsumption checks.
fn preprocess_clauses(mut clauses: Vec<Clause>) -> Vec<Clause> {
    // Simplify the clauses as much as possible.
    for cl in &mut clauses {
        cheap_simplify(cl);
    }
    
    // Then get rid of tautologies.
    let mut new_clauses = clauses.into_iter().filter(|cl| !trivial(cl)).collect::<Vec<_>>();
       
    // Remove all subsumed clauses.
    let mut newer_clauses = Vec::new();
    while let Some(cl) = new_clauses.pop() {
        if !forward_subsumed(&cl, &newer_clauses) {
            backward_subsumption(&cl, &mut newer_clauses);
            newer_clauses.push(cl);
        }
    }
    
    newer_clauses
}

/// If the problem contains one unary function, this function finds it.
fn single_unary_function(clauses: &[Clause]) -> Option<i64> {
    let mut found_unary = None;
    
    for cl in clauses {
        for l in cl.iter() {
            for t in l.iter() {
                if t.get_arity() == 1 {
                    assert!(t.is_function());
                    if found_unary.is_some() {
                        if found_unary != Some(t.get_id()) {
                            return None;
                        }    
                    } else {
                        found_unary = Some(t.get_id());
                    }
                }
            }
        }
    }
    
    found_unary
}

fn update_function_symbol_count(counts: &mut HashMap<i64, i64>, t: &Term) {
    if t.is_function() {
        // Get around lifetime stuff.
        {
            let v = counts.entry(t.get_id()).or_insert(0);
            *v += 1;
        }    
        for sub_t in t.iter() {
            update_function_symbol_count(counts, sub_t)
        }
    }
}

fn create_function_symbol_count(clauses: &[Clause]) -> HashMap<i64, i64> {
    let mut counts = HashMap::new();
    
    for cl in clauses {
        for l in cl.iter() {
            for t in l.iter() {
                update_function_symbol_count(&mut counts, t)
            }
        }
    }
    
    counts
}

fn create_term_ordering(lpo_over_kbo: bool, clauses: &[Clause]) -> TermOrdering {
    if lpo_over_kbo {
        TermOrdering::LPO(Precedence::default())
    } else {
        let counts = create_function_symbol_count(clauses);
        TermOrdering::KBO(Precedence::ArityFrequency(counts), Weight::SimpleWeight, single_unary_function(clauses)) 
    }
}

/// A more general version of prove with many parameters exposed.
/// First we can decide whether we want to use LPO or KBO.
/// Then there is the option for not negating the input clause if we are more interested in satisfiability.
pub fn prove_general(s: &str, use_lpo: bool, negate_input_formula: bool, max_time_in_s: u64, tptp_format: bool) -> (ProofAttemptResult, ProofStatistics) {
    let (parsed_formula, mut renaming_info) = if tptp_format {
                                                  tptp_to_cnf_ast(s)
                                              } else {
                                                  match internal_to_cnf_ast(s) {
                                                      Ok(res) => res,
                                                      Err(_) => { return (ProofAttemptResult::Error, ProofStatistics::new()); },
                                                  }
                                              };
    let cnf_f = if negate_input_formula { 
                    cnf(Formula::Not(Box::new(parsed_formula)), &mut renaming_info)
                } else {
                    cnf(parsed_formula, &mut renaming_info)
                };                    
    if cnf_f == Formula::False {
        (ProofAttemptResult::Refutation, ProofStatistics::new())
    } else if cnf_f == Formula::True {
        (ProofAttemptResult::Saturation, ProofStatistics::new())
    } else {
        let flattened_cnf_f = flatten_cnf(cnf_f);
        let preprocessed_problem = preprocess_clauses(flattened_cnf_f);                
        let term_ordering = create_term_ordering(use_lpo, &preprocessed_problem);
        let proof_state = ProofState::new(preprocessed_problem, term_ordering);
        serkr_loop(proof_state, renaming_info.var_cnt, max_time_in_s * 1000)
    }
}

/// Attempts to prove the stuff in the TPTP file at the location given.
pub fn prove_tptp(s: &str, use_lpo: bool, time_limit: u64) -> (ProofAttemptResult, ProofStatistics) {
    prove_general(s, use_lpo, false, time_limit, true)
}

/// Attempts to prove the formula represented as a string in the internal format.
pub fn prove(s: &str) -> (ProofAttemptResult, ProofStatistics) {
    prove_general(s, false, true, 300, false)
}

#[cfg(test)]
mod test {
    use super::{prove, prove_general, ProofAttemptResult};
    
    // Contains some problems negated so that we can make sure we don't prove a theorem not actually provable.
    
    #[test]
    fn pelletier_1() {
        let result = prove("(P ==> Q) <=> (~Q ==> ~P)").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_1_negated() {
        let result = prove_general("(P ==> Q) <=> (~Q ==> ~P)", true, false, 30, false).0;
        assert_eq!(result, ProofAttemptResult::Saturation);
    }
    
    #[test]
    fn pelletier_2() {
        let result = prove("~~P <=> P").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_3() {
        let result = prove("~(P ==> Q) ==> (Q ==> P)").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_4() {
        let result = prove("(~P ==> Q) <=> (~Q ==> P)").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_5() {
        let result = prove("((P \\/ Q) ==> (P \\/ R)) ==> (P \\/ (Q ==> R))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_6() {
        let result = prove("P \\/ ~P").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_7() {
        let result = prove("P \\/ ~~~P").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_8() {
        let result = prove("((P ==> Q) ==> P) ==> P").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_8_negated() {
        let result = prove_general("((P ==> Q) ==> P) ==> P", true, false, 30, false).0;
        assert_eq!(result, ProofAttemptResult::Saturation);
    }
    
    #[test]
    fn pelletier_9() {
        let result = prove("(P \\/ Q) /\\ (~P \\/ Q) /\\ (P \\/ ~Q) ==> ~(~P \\/ ~Q)").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_10() {
        let result = prove("(Q ==> R) /\\ (R ==> P /\\ Q) /\\ (P ==> Q \\/ R) ==> (P <=> Q)").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_11() {
        let result = prove("P <=> P").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_12() {
        let result = prove("((P <=> Q) <=> R) <=> (P <=> (Q <=> R))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_13() {
        let result = prove("(P \\/ Q /\\ R) <=> ((P \\/ Q) /\\ (P \\/ R))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_14() {
        let result = prove("(P <=> Q) <=> ((~P \\/ Q) /\\ (~Q \\/ P))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_15() {
        let result = prove("(P ==> Q) <=> (~Q \\/ Q)").0;
        assert_eq!(result, ProofAttemptResult::Saturation);
    }
    
    #[test]
    fn pelletier_15_errata() {
        let result = prove("(P ==> Q) <=> (~P \\/ Q)").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_16() {
        let result = prove("(P ==> Q) \\/ (Q ==> P)").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_17() {
        let result = prove("(P /\\ (Q ==> R) ==> S) <=> ((~P \\/ Q \\/ S) /\\ (~P \\/ ~R \\/ S))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_18() {
        let result = prove("exists y. forall x. (F(y) ==> F(x))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_18_negated() {
        let result = prove("~exists y. forall x. (F(y) ==> F(x))").0;
        assert_eq!(result, ProofAttemptResult::Saturation);
    }
    
    #[test]
    fn pelletier_19() {
        let result = prove("exists x. forall y. forall z. ((P(y) ==> Q(z)) ==> (P(x) ==> Q(x)))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_20() {
        let result = prove("forall x. forall y. exists z. forall w. (P(x) /\\ Q(y) ==> R(z) /\\ S(w))
                            ==> (exists x. exists y. (P(x) /\\ Q(y)) ==> exists z. R(z))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_21() {
        let result = prove("exists x. (P ==> F(x)) /\\ 
                            exists x. (F(x) ==> P) 
                            ==> exists x. (P <=> F(x))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }

    #[test]
    fn pelletier_22() {
        let result = prove("forall x. (P <=> F(x)) ==> (P <=> forall x. F(x))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_22_negated() {
        let result = prove_general("forall x. (P <=> F(x)) ==> (P <=> forall x. F(x))", true, false, 30, false).0;
        assert_eq!(result, ProofAttemptResult::Saturation);
    }
    
    #[test]
    fn pelletier_23() {
        let result = prove("forall x. (P \\/ F(x)) <=> P \\/ forall x. F(x)").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_24() {
        let result = prove("~exists x. (S(x) /\\ Q(x)) /\\
                            forall x. (P(x) ==> Q(x) \\/ R(x)) /\\
                            (~exists x. P(x) ==> exists x. Q(x)) /\\
                            forall x. (Q(x) \\/ R(x) ==> S(x))
                            ==> exists x. (P(x) /\\ R(x))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_25() {
        let result = prove("exists x. P(x) /\\ 
                            forall x. (F(x) ==> ~G(x) /\\ R(x)) /\\
                            forall x. (P(x) ==> G(x) /\\ F(x)) /\\
                           (forall x. (P(x) ==> Q(x)) \\/ exists x. (P(x) /\\ R(x)))
                            ==> exists x. (Q(x) /\\ P(x))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_26() {
        let result = prove("(exists x. P(x) <=> exists x. Q(x)) /\\
                            forall x. forall y. (P(x) /\\ Q(y) ==> (R(x) <=> S(y)))
                            ==> (forall x. (P(x) ==> R(x)) <=> forall x. (Q(x) ==> S(x)))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_27() {
        let result = prove("exists x. (F(x) /\\ ~G(x)) /\\ 
                            forall x. (F(x) ==> H(x)) /\\ 
                            forall x. (J(x) /\\ I(x) ==> F(x)) /\\ 
                            (exists x. (H(x) /\\ ~G(x)) ==> forall x. (I(x) ==> ~H(x)))
                            ==> forall x. (J(x) ==> ~I(x))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }

    #[test]
    fn pelletier_28_orig() {
        let result = prove("(forall x. P(x) ==> forall x. Q(x)) /\\
                            (forall x. (Q(x) \\/ R(x)) ==> exists x. (Q(x) /\\ S(x))) /\\
                            (exists x. S(x) ==> forall x. (F(x) ==> G(x))) 
                             ==> forall x. (P(x) /\\ F(x) ==> G(x))").0;
        assert_eq!(result, ProofAttemptResult::Saturation);
    }
    
    #[test]
    fn pelletier_28_errata() {
        let result = prove("forall x. (P(x) ==> forall x. Q(x)) /\\
                           (forall x. (Q(x) \\/ R(x)) ==> exists x. (Q(x) /\\ S(x))) /\\
                           (exists x. S(x) ==> forall x. (F(x) ==> G(x)))
                            ==> forall x. (P(x) /\\ F(x) ==> G(x))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_29() {
        let result = prove("(exists x. F(x) /\\ exists x. G(x))
                             ==> ((forall x. (F(x) ==> H(x)) /\\ forall x. (G(x) ==> J(x))) <=> forall x. forall y. (F(x) /\\ G(y) ==> H(x) /\\ J(y)))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_30() {
        let result = prove("forall x. (F(x) \\/ G(x) ==> ~H(x)) /\\ 
                            forall x. ((G(x) ==> ~I(x)) ==> F(x) /\\ H(x)) 
                            ==> forall x. I(x)").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_30_negated() {
        let result = prove_general("forall x. (F(x) \\/ G(x) ==> ~H(x)) /\\ 
                                    forall x. ((G(x) ==> ~I(x)) ==> F(x) /\\ H(x)) 
                                    ==> forall x. I(x)", true, false, 30, false).0;
        assert_eq!(result, ProofAttemptResult::Saturation);
    }
    
    #[test]
    fn pelletier_31() {
        let result = prove("~exists x. (F(x) /\\ (G(x) \\/ H(x))) /\\ 
                            exists x. (I(x) /\\ F(x)) /\\ 
                            forall x. (~H(x) ==> J(x))
                            ==> exists x. (I(x) /\\ J(x))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_32() {
        let result = prove("forall x. (F(x) /\\ (G(x) \\/ H(x)) ==> I(x)) /\\ 
                            forall x. (I(x) /\\ H(x) ==> J(x)) /\\ 
                            forall x. (K(x) ==> H(x))
                            ==> forall x. (F(x) /\\ K(x) ==> J(x))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_33() {
        let result = prove("forall x. (P(a) /\\ (P(x) ==> P(b)) ==> P(c)) <=>
                            forall x. ((~P(a) \\/ P(x) \\/ P(c)) /\\ (~P(a) \\/ ~P(b) \\/ P(c)))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    /*
    #[test]
    fn pelletier_34() {
        let result = prove("((exists x. forall y. P(x) <=> P(y)) <=>
                             (exists x. Q(x) <=> forall y. Q(y))) <=>
                            ((exists x. forall y. Q(x) <=> Q(y)) <=>
                             (exists x. P(x) <=> forall y. P(y)))");
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    */

    #[test]
    fn pelletier_35() {
        let result = prove("exists x. exists y. (P(x, y) ==> forall x. forall y. P(x, y))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_35_negated() {
        let result = prove("~exists x. exists y. (P(x, y) ==> forall x. forall y. P(x, y))").0;
         assert_eq!(result, ProofAttemptResult::Saturation);
    }
    
    #[test]
    fn pelletier_36() {
        let result = prove("forall x. exists y. F(x, y) /\\ 
                            forall x. exists y. G(x, y) /\\ 
                            forall x. forall y. (F(x, y) \\/ G(x, y) ==> forall z. (F(y, z) \\/ G(y, z) ==> H(x, z)))
                            ==> forall x. exists y. H(x, y)").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_37() {
        let result = prove("forall z. exists w. forall x. exists y. ((P(x, z) ==> P(y, w)) /\\ P(y, z) /\\ (P(y, w) ==> exists u. Q(u, w))) /\\
                            forall x. forall z. (~P(x, z) ==> exists y. Q(y, z)) /\\
                            (exists x. exists y. Q(x, y) ==> forall x. R(x, x))
                            ==> forall x. exists y. R(x, y)").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }

    /*
    #[test]
    fn pelletier_38() {
        let result = prove("forall x. (P(a()) /\\ (P(x) ==> exists y. (P(y) /\\ R(x, y))) ==> exists z. exists w. (P(z) /\\ R(x, w) /\\ R(w, z))) <=>
                            forall x. ((~P(a()) \\/ P(x) \\/ exists z. exists w. (P(z) /\\ R(x, w) /\\ R(w, z))) /\\
                            (~P(a()) \\/ ~exists y. (P(y) /\\ R(x, y)) \\/ exists z. exists w. (P(z) /\\ R(x, w) /\\ R(w, z))))");
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    */
    
    #[test]
    fn pelletier_39() {
        let result = prove("~exists x. forall y. (F(y, x) <=> ~F(y, y))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_40() {
        let result = prove("exists y. forall x. (F(x, y) <=> F(x, x)) 
                            ==> ~forall x. exists y. forall z. (F(x, y) <=> ~F(z, x))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_40_errata() {
        let result = prove("exists y. forall x. (F(x, y) <=> F(x, x)) 
                            ==> ~forall x. exists y. forall z. (F(z, y) <=> ~F(z, x))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_41() {
        let result = prove("forall z. exists y. forall x. (F(x, y) <=> F(x, z) /\\ ~F(x, x)) 
                            ==> ~exists z. forall x. F(x, z)").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_42() {
        let result = prove("~exists y. forall x. (F(x, y) <=> ~exists z. (F(x, z) /\\ F(z, x)))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_42_negated() {
        let result = prove("exists y. forall x. (F(x, y) <=> ~exists z. (F(x, z) /\\ F(z, x)))").0;
        assert_eq!(result, ProofAttemptResult::Saturation);
    }

    #[test]
    fn pelletier_43() {
        let result = prove("forall x. forall y. (Q(x, y) <=> forall z. (F(z, x) <=> F(z, y)))
                            ==> forall x. forall y. (Q(x, y) <=> Q(y, x))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }

    #[test]
    fn pelletier_44() {
        let result = prove("forall x. (F(x) ==> exists y. (G(y) /\\ H(x, y)) /\\ exists y. (G(y) /\\ ~H(x, y))) /\\ 
                            exists x. (J(x) /\\ forall y. (G(y) ==> H(x, y)))
                            ==> exists x. (J(x) /\\ ~F(x))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }   

    #[test]
    fn pelletier_45() {
        let result = prove("forall x. (F(x) /\\ forall y. (G(y) /\\ H(x, y) ==> J(x, y)) ==> forall y. (G(y) /\\ H(x, y) ==> K(y))) /\\ 
                            ~exists y. (L(y) /\\ K(y)) /\\ 
                            exists x. (F(x) /\\ forall y. (H(x, y) ==> L(y)) /\\ forall y. (G(y) /\\ H(x, y) ==> J(x, y)))
                            ==> exists x. (F(x) /\\ ~exists y. (G(y) /\\ H(x, y)))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_46() {
        let result = prove("forall x. (F(x) /\\ forall y. (F(y) /\\ H(y, x) ==> G(y)) ==> G(x)) /\\
                           (exists x. (F(x) /\\ ~G(x)) ==> exists x. (F(x) /\\ ~G(x) /\\ forall y. (F(y) /\\ ~G(y) ==> J(x, y)))) /\\
                            forall x. forall y. (F(x) /\\ F(y) /\\ H(x, y) ==> ~J(y, x))
                            ==> forall x. (F(x) ==> G(x))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    /*
    #[test]
    fn pelletier_47() {
        let result = prove("forall x. (P1(x) ==> P0(x)) /\\ exists x. P1(x) /\\ 
                            forall x. (P2(x) ==> P0(x)) /\\ exists x. P2(x) /\\ 
                            forall x. (P3(x) ==> P0(x)) /\\ exists x. P3(x) /\\
                            forall x. (P4(x) ==> P0(x)) /\\ exists x. P4(x) /\\
                            forall x. (P5(x) ==> P0(x)) /\\ exists x. P5(x) /\\ 
                            exists x. Q1(x) /\\ forall x. (Q1(x) ==> Q0(x)) /\\
                            forall x. (P0(x) ==> (forall y. (Q0(y) ==> R(x, y)) \\/ forall y. (P0(y) /\\ S(y, x) /\\ exists z. (Q0(z) /\\ R(y, z)) ==> R(x, y)))) /\\
                            forall x. forall y. (P3(y) /\\ (P5(x) \\/ P4(x)) ==> S(x, y)) /\\ 
                            forall x. forall y. (P3(x) /\\ P2(y) ==> S(x, y)) /\\ 
                            forall x. forall y. (P2(x) /\\ P1(y) ==> S(x, y)) /\\
                            forall x. forall y. (P1(x) /\\ (P2(y) \\/ Q1(y)) ==> ~R(x, y)) /\\ 
                            forall x. forall y. (P3(x) /\\ P4(y) ==> R(x, y)) /\\ 
                            forall x. forall y. (P3(x) /\\ P5(y) ==> ~R(x, y)) /\\
                            forall x. (P4(x) \\/ P5(x) ==> exists y. (Q0(y) /\\ R(x, y)))
                            ==> exists x. exists y. (P0(x) /\\ P0(y) /\\ exists z. (Q1(z) /\\ R(y, z) /\\ R(x, y)))");
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    */
    
    #[test]
    fn pelletier_48() {
        let result = prove("(a() = b() \\/ c() = d()) /\\ 
                            (a() = c() \\/ b() = d()) 
                            ==> a() = d() \\/ b() = c()").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_49() {
        let result = prove("exists x. exists y. forall z. (z = x \\/ z = y) /\\ 
                            P(a()) /\\ P(b()) /\\ a() <> b()
                            ==> forall x. P(x)").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_50() {
        let result = prove("forall x. (F(a(), x) \\/ forall y. F(x, y)) ==> exists x. forall y. F(x, y)").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_50_negated() {
        let result = prove_general("forall x. (F(a(), x) \\/ forall y. F(x, y)) ==> exists x. forall y. F(x, y)", true, false, 30, false).0;
        assert_eq!(result, ProofAttemptResult::Saturation);
    }
    
    #[test]
    fn pelletier_51() {
        let result = prove("exists z. exists w. forall x. forall y. (F(x, y) <=> x = z /\\ y = w)
                            ==> exists z. forall x. (exists w. forall y. (F(x, y) <=> y = w) <=> x = z)").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_52() {
        let result = prove("exists z. exists w. forall x. forall y. (F(x, y) <=> x = z /\\ y = w)
                            ==> exists w. forall y. (exists z. forall x. (F(x, y) <=> x = z) <=> y = w)").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
  
    /*
    #[test]
    fn pelletier_53() {
        let result = prove("exists x. exists y. (x <> y /\\ forall z. (z = x \\/ z = y)) <=>
                           (exists z. forall x. (exists w. forall y. (F(x, y) <=> y = w) <=> x = z)
                            <=> exists w. forall y. (exists z. forall x. (F(x, y) <=> x = z) <=> y = w))");
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    */

    #[test]
    fn pelletier_54() {
        let result = prove("forall y. exists z. forall x. (F(x, z) <=> x = y) 
                            ==> ~exists w. forall x. (F(x, w) <=> forall u. (F(x, u) ==> exists y. (F(y, u) /\\ ~exists z. (F(z, u) /\\ F(z, y)))))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_55() {
        let result = prove("exists x. (L(x) /\\ K(x, a())) /\\ 
                            L(a()) /\\ L(b()) /\\ L(c()) /\\ 
                            forall x. (L(x) ==> x = a() \\/ x = b() \\/ x = c()) /\\ 
                            forall x. forall y. (K(x, y) ==> H(x, y)) /\\ 
                            forall x. forall y. (K(x, y) ==> ~R(x, y)) /\\ 
                            forall x. (H(a(), x) ==> ~H(c(), x)) /\\ 
                            forall x. (x <> b() ==> H(a(), x)) /\\ 
                            forall x. (~R(x, a()) ==> H(b(), x)) /\\ 
                            forall x. (H(a(), x) ==> H(b(), x)) /\\ 
                            forall x. exists y. ~H(x, y) /\\ 
                            a() <> b()
                            ==> K(a(), a())").0; 
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_56() {
        let result = prove("forall x. (exists y. (F(y) /\\ x = f(y)) ==> F(x)) <=> forall x. (F(x) ==> F(f(x)))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_56_negated() {
        let result = prove_general("forall x. (exists y. (F(y) /\\ x = f(y)) ==> F(x)) <=> forall x. (F(x) ==> F(f(x)))", true, false, 30, false).0;
        assert_eq!(result, ProofAttemptResult::Saturation);
    }
    
    #[test]
    fn pelletier_57() {
        let result = prove("F(f(a(), b()), f(b(), c())) /\\ 
                            F(f(b(), c()), f(a(), c())) /\\
                            forall x. forall y. forall z. (F(x, y) /\\ F(y, z) ==> F(x, z)) 
                            ==> F(f(a(), b()), f(a(), c()))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_58() {
        let result = prove("forall x. forall y. f(x) = g(y) ==> forall x. forall y. f(f(x)) = f(g(y))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_59() {
        let result = prove("forall x. (F(x) <=> ~F(f(x))) ==> exists x. (F(x) /\\ ~F(f(x)))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_60() {
        let result = prove("forall x. (F(x, f(x)) <=> exists y. (forall z. (F(z, y) ==> F(z, f(x))) /\\ F(x, y)))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_61() {
        let result = prove("forall x. forall y. forall z. f(x, f(y, z)) = f(f(x, y), z)
                            ==> forall x. forall y. forall z. forall w. f(x, f(y, f(z, w))) = f(f(f(x, y), z), w)").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_62_errata_errata() {
        let result = prove("forall x. (F(a()) /\\ (F(x) ==> F(f(x))) ==> F(f(f(x)))) <=>
                            forall x. ((~F(a()) \\/ F(x) \\/ F(f(f(x)))) /\\ (~F(a()) \\/ ~F(f(x)) \\/ F(f(f(x)))))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_63() {
        let result = prove("forall x. forall y. forall z. f(f(x, y), z) = f(x, f(y, z)) /\\ 
                            forall x. f(a(), x) = x /\\ 
                            forall x. exists y. f(y, x) = a()
                            ==> forall x. forall y. forall z. (f(x, y) = f(z, y) ==> x = z)").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_64() {
        let result = prove("forall x. forall y. forall z. f(f(x, y), z) = f(x, f(y, z)) /\\ 
                            forall x. f(a(), x) = x /\\ 
                            forall x. exists y. f(y, x) = a()
                            ==> forall x. forall y. (f(y, x) = a() ==> f(x, y) = a())").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_65() {
        let result = prove("forall x. forall y. forall z. f(f(x, y), z) = f(x, f(y, z)) /\\ 
                            forall x. f(a(), x) = x
                            ==> (forall x. f(x, x) = a() ==> forall x. forall y. f(x, y) = f(y, x))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn pelletier_72() {
        let result = prove_general("(P1 \\/ P2 \\/ P3) /\\
                                    (P4 \\/ P5 \\/ P6) /\\
                                    (P7 \\/ P8 \\/ P9) /\\
                                    (P10 \\/ P11 \\/ P12) /\\
                                    (~P1 \\/ ~P4) /\\
                                    (~P1 \\/ ~P7) /\\
                                    (~P1 \\/ ~P10) /\\
                                    (~P4 \\/ ~P7) /\\
                                    (~P4 \\/ ~P10) /\\
                                    (~P7 \\/ ~P10) /\\
                                    (~P2 \\/ ~P5) /\\
                                    (~P2 \\/ ~P8) /\\
                                    (~P2 \\/ ~P11) /\\
                                    (~P5 \\/ ~P8) /\\
                                    (~P5 \\/ ~P11) /\\
                                    (~P8 \\/ ~P11) /\\
                                    (~P3 \\/ ~P6) /\\
                                    (~P3 \\/ ~P9) /\\
                                    (~P3 \\/ ~P12) /\\
                                    (~P6 \\/ ~P9) /\\
                                    (~P6 \\/ ~P12) /\\
                                    (~P9 \\/ ~P12)", true, false, 30, false).0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    /*
    #[test]
    fn pelletier_73() {
        let result = prove_general("exists x. exists y. exists z. exists w. (O(x) /\\ O(y) /\\ O(z) /\\ O(w) /\\ x <> y /\\ x <> z /\\ x <> w /\\ y <> z /\\ y <> w /\\ z <> w) /\\
                                    exists x. exists y. exists z. (H(x) /\\ H(y) /\\ H(z) /\\ x <> y /\\ x <> z /\\ y <> z /\\ forall w. (H(w) ==> w = x \\/ w = y \\/ w = z)) /\\
                                    forall x. (O(x) ==> exists y. (H(y) /\\ I(x, y))) /\\
                                    forall x. (H(x) ==> forall y. forall z. (O(y) /\\ O(z) /\\ I(y, x) /\\ I(z, x) ==> y = z))", false, false, 30);
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    */

    #[test]
    fn los() {
        let result = prove("forall x. forall y. forall z. (P(x, y) /\\ P(y, z) ==> P(x, z)) /\\
                            forall x. forall y. forall z. (Q(x, y) /\\ Q(y, z) ==> Q(x, z)) /\\
                            forall x. forall y. (Q(x, y) ==> Q(y, x)) /\\
                            forall x. forall y. (P(x, y) \\/ Q(x, y))
                            ==> forall x. forall y. P(x, y) \\/ forall x. forall y. Q(x, y)").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
       
    #[test]
    fn davis_putnam() {
        let result = prove("exists x. exists y. forall z. ((F(x, y) ==> F(y, z) /\\ F(z, z)) /\\ (F(x, y) /\\ G(x, y) ==> G(x, z) /\\ G(z, z)))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn djikstra() {
        let result = prove("forall x. f(f(x)) = f(x) /\\ forall x. exists y. f(y) = x 
                            ==> forall x. f(x) = x").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn djikstra_negated() {
        let result = prove_general("forall x. f(f(x)) = f(x) /\\ forall x. exists y. f(y) = x 
                                    ==> forall x. f(x) = x", true, false, 30, false).0;
        assert_eq!(result, ProofAttemptResult::Saturation);
    }
    
    #[test]
    fn ewd() {
        let result = prove("forall x. (F(x) ==> G(x)) /\\ 
                            exists x. F(x) /\\
                            forall x. forall y. (G(x) /\\ G(y) ==> x = y)
                            ==> forall y. (G(y) ==> F(y))").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    /*
    #[test]
    fn wishnu() {
        let result = prove("exists x. (x = f(g(x)) /\\ forall x1. (x1 = f(g(x1)) ==> x = x1)) <=>
                            exists y. (y = g(f(y)) /\\ forall y1. (y1 = g(f(y1)) ==> y = y1))");
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    */
    
    #[test]
    fn group_x_times_x_equals_1_abelian() {
        let result = prove("forall x. forall y. forall z. mult(x, mult(y, z)) = mult(mult(x, y), z) /\\
                            forall x. mult(e(), x) = x /\\
                            forall x. mult(x, x) = e()
                            ==> forall x. forall y. mult(x, y) = mult(y, x)").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn group_left_inverse_means_right_inverse() {
        let result = prove("forall x. forall y. forall z. mult(x, mult(y, z)) = mult(mult(x, y), z) /\\
                            forall x. mult(e(), x) = x /\\
                            forall x. mult(i(x), x) = e()
                             ==> forall x. mult(x, i(x)) = e()").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn set_union_is_commutative() {
        let result = prove("forall a. forall b. forall x. (F(x, union(a, b)) <=> F(x, a) \\/ F(x, b)) /\\
                            forall a. forall b. (forall x. (F(x, a) <=> F(x, b)) ==> a = b)
                            ==> forall a. forall b. union(a, b) = union(b, a)").0;
        assert_eq!(result, ProofAttemptResult::Refutation);
    }
    
    #[test]
    fn lists_1() {
        let result = prove_general("forall x. suc(x) <> zero() /\\
                                    forall x. forall y. (suc(x) = suc(y) ==> x = y)", true, false, 300, false).0;
        assert_eq!(result, ProofAttemptResult::Saturation);
    }
} 
