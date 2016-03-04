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
use cnf::ast_transformer_tptp::tptp_to_cnf_ast;
use cnf::naive_cnf::cnf;
use utils::stopwatch::Stopwatch;

/// Contains the result of a proof attempt.
#[derive(Eq, PartialEq, Clone, Debug)]
#[allow(missing_docs)]
pub enum ProofAttemptResult {
    Theorem,
    CounterSatisfiable,
    Unsatisfiable,
    Satisfiable,
    Timeout,
    Error(String),
}

impl ProofAttemptResult {
    /// If the formula to prove contains conjectures, we should report Theorem instead of Unsatisfiable.
    fn new_refutation(contains_conjectures: bool) -> ProofAttemptResult {
        if contains_conjectures {
            ProofAttemptResult::Theorem
        } else {
            ProofAttemptResult::Unsatisfiable
        }
    }
    
    /// If the formula to prove contains conjectures, we should report CounterSatisfiable instead of Satisfiable.
    fn new_saturation(contains_conjectures: bool) -> ProofAttemptResult {
        if contains_conjectures {
            ProofAttemptResult::CounterSatisfiable
        } else {
            ProofAttemptResult::Satisfiable
        }
    }
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
fn serkr_loop(mut proof_state: ProofState, mut var_cnt: i64, max_time_in_ms: u64, contains_conjectures: bool) -> (ProofAttemptResult, ProofStatistics) {
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
            return (ProofAttemptResult::new_refutation(contains_conjectures), stats);
        }
        
        // Check if the clause is redundant in some way. If it is no need to process it more.
        if !forward_subsumed(&chosen_clause, proof_state.get_used()) {
            stats.bs_count += backward_subsumption(&chosen_clause, proof_state.get_used_mut());
            proof_state.add_to_used(chosen_clause.clone()); 
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
        } else {
            stats.fs_count += 1;
        }
    }
    
    (ProofAttemptResult::new_saturation(contains_conjectures), stats)
}

/// Try to simplify, delete tautologies and remove subsumed clauses from those clauses passed in.
/// Possibly very costly due to subsumption checks.
#[allow(dead_code)]
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

/// Attempts to prove the stuff in the TPTP file at the location given.
/// First we can decide whether we want to use LPO or KBO.
/// Then there is the option of limiting the time of the proof search.
pub fn prove(s: &str, use_lpo: bool, max_time_in_s: u64) -> (ProofAttemptResult, ProofStatistics) {
    let (parsed_formula, mut renaming_info, contains_conjectures) = match tptp_to_cnf_ast(s) {
                                                                        Ok(res) => res,
                                                                        Err(s) => { return (ProofAttemptResult::Error(s), ProofStatistics::new()); },
                                                                    };
                                                                    
    let cnf_f = if contains_conjectures { 
                    cnf(Formula::Not(Box::new(parsed_formula)), &mut renaming_info)
                } else {
                    cnf(parsed_formula, &mut renaming_info)
                };
                
    if cnf_f == Formula::False {
        (ProofAttemptResult::new_refutation(contains_conjectures), ProofStatistics::new())
    } else if cnf_f == Formula::True {
        (ProofAttemptResult::new_saturation(contains_conjectures), ProofStatistics::new())
    } else {
        let flattened_cnf_f = flatten_cnf(cnf_f);
        // Large problems get stuck in preprocessing, so it is not used currently.
        // let preprocessed_problem = preprocess_clauses(flattened_cnf_f);                
        let term_ordering = create_term_ordering(use_lpo, &flattened_cnf_f);
        let proof_state = ProofState::new(flattened_cnf_f, term_ordering);
        serkr_loop(proof_state, renaming_info.var_cnt, max_time_in_s * 1000, contains_conjectures)
    }
}

#[cfg(test)]
mod test {
    use super::{prove, ProofAttemptResult};
    
    // Contains some problems negated so that we can make sure we don't prove a theorem not actually provable.
    
    #[test]
    fn pelletier_1() {
        let (result, _) = prove("test_problems\\p1.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_1_negated() {
        let (result, _) = prove("test_problems\\p1n.p", false, 300);
        assert_eq!(result, ProofAttemptResult::CounterSatisfiable);
    }
    
    #[test]
    fn pelletier_2() {
        let (result, _) = prove("test_problems\\p2.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_3() {
        let (result, _) = prove("test_problems\\p3.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_4() {
        let (result, _) = prove("test_problems\\p4.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_5() {
        let (result, _) = prove("test_problems\\p5.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_6() {
        let (result, _) = prove("test_problems\\p6.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_7() {
        let (result, _) = prove("test_problems\\p7.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_8() {
        let (result, _) = prove("test_problems\\p8.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_8_negated() {
        let (result, _) = prove("test_problems\\p8n.p", false, 300);
        assert_eq!(result, ProofAttemptResult::CounterSatisfiable);
    }
    
    #[test]
    fn pelletier_9() {
        let (result, _) = prove("test_problems\\p9.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_10() {
        let (result, _) = prove("test_problems\\p10.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_11() {
        let (result, _) = prove("test_problems\\p11.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_12() {
        let (result, _) = prove("test_problems\\p12.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_13() {
        let (result, _) = prove("test_problems\\p13.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_14() {
        let (result, _) = prove("test_problems\\p14.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_15_original() {
        let (result, _) = prove("test_problems\\p15o.p", false, 300);
        assert_eq!(result, ProofAttemptResult::CounterSatisfiable);
    }
    
    #[test]
    fn pelletier_15_errata() {
        let (result, _) = prove("test_problems\\p15e.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_16() {
        let (result, _) = prove("test_problems\\p16.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_17() {
        let (result, _) = prove("test_problems\\p17.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_18() {
        let (result, _) = prove("test_problems\\p18.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_18_negated() {
        let (result, _) = prove("test_problems\\p18n.p", false, 300);
        assert_eq!(result, ProofAttemptResult::CounterSatisfiable);
    }
    
    #[test]
    fn pelletier_19() {
        let (result, _) = prove("test_problems\\p19.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_20() {
        let (result, _) = prove("test_problems\\p20.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_21() {
        let (result, _) = prove("test_problems\\p21.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_22() {
        let (result, _) = prove("test_problems\\p22.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_22_negated() {
        let (result, _) = prove("test_problems\\p22n.p", false, 300);
        assert_eq!(result, ProofAttemptResult::CounterSatisfiable);
    }
    
    #[test]
    fn pelletier_23() {
        let (result, _) = prove("test_problems\\p23.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_24() {
        let (result, _) = prove("test_problems\\p24.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_25() {
        let (result, _) = prove("test_problems\\p25.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_26() {
        let (result, _) = prove("test_problems\\p26.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_27() {
        let (result, _) = prove("test_problems\\p27.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_28_original() {
        let (result, _) = prove("test_problems\\p28o.p", false, 300);
        assert_eq!(result, ProofAttemptResult::CounterSatisfiable);
    }
    
    #[test]
    fn pelletier_28_errata() {
        let (result, _) = prove("test_problems\\p28e.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_29() {
        let (result, _) = prove("test_problems\\p29.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_30() {
        let (result, _) = prove("test_problems\\p30.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_30_negated() {
        let (result, _) = prove("test_problems\\p30n.p", false, 300);
        assert_eq!(result, ProofAttemptResult::CounterSatisfiable);
    }
    
    #[test]
    fn pelletier_31() {
        let (result, _) = prove("test_problems\\p31.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_32() {
        let (result, _) = prove("test_problems\\p32.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_33() {
        let (result, _) = prove("test_problems\\p33.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_35() {
        let (result, _) = prove("test_problems\\p35.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_35_negated() {
        let (result, _) = prove("test_problems\\p35n.p", false, 300);
        assert_eq!(result, ProofAttemptResult::CounterSatisfiable);
    }
    
    #[test]
    fn pelletier_36() {
        let (result, _) = prove("test_problems\\p36.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_37() {
        let (result, _) = prove("test_problems\\p37.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_39() {
        let (result, _) = prove("test_problems\\p39.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_40_original() {
        let (result, _) = prove("test_problems\\p40o.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_40_errata() {
        let (result, _) = prove("test_problems\\p40e.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_41() {
        let (result, _) = prove("test_problems\\p41.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_42() {
        let (result, _) = prove("test_problems\\p42.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_43() {
        let (result, _) = prove("test_problems\\p43.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_44() {
        let (result, _) = prove("test_problems\\p44.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_45() {
        let (result, _) = prove("test_problems\\p45.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_46() {
        let (result, _) = prove("test_problems\\p46.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_48() {
        let (result, _) = prove("test_problems\\p48.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_49() {
        let (result, _) = prove("test_problems\\p49.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_50() {
        let (result, _) = prove("test_problems\\p50.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_50_negated() {
        let (result, _) = prove("test_problems\\p50n.p", false, 300);
        assert_eq!(result, ProofAttemptResult::CounterSatisfiable);
    }
    
    #[test]
    fn pelletier_51() {
        let (result, _) = prove("test_problems\\p51.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_52() {
        let (result, _) = prove("test_problems\\p52.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_55() {
        let (result, _) = prove("test_problems\\p55.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_56() {
        let (result, _) = prove("test_problems\\p56.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_56_negated() {
        let (result, _) = prove("test_problems\\p56n.p", false, 300);
        assert_eq!(result, ProofAttemptResult::CounterSatisfiable);
    }
    
    #[test]
    fn pelletier_57() {
        let (result, _) = prove("test_problems\\p57.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_58() {
        let (result, _) = prove("test_problems\\p58.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_59() {
        let (result, _) = prove("test_problems\\p59.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_60() {
        let (result, _) = prove("test_problems\\p60.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_61() {
        let (result, _) = prove("test_problems\\p61.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_62_errata_errata() {
        let (result, _) = prove("test_problems\\p62ee.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_63() {
        let (result, _) = prove("test_problems\\p63.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_64() {
        let (result, _) = prove("test_problems\\p64.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_65() {
        let (result, _) = prove("test_problems\\p65.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn pelletier_72() {
        let (result, _) = prove("test_problems\\p72.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Unsatisfiable);
    }
    
    #[test]
    fn davis_putnam() {
        let (result, _) = prove("test_problems\\davis_putnam.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn los() {
        let (result, _) = prove("test_problems\\los.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn ewd() {
        let (result, _) = prove("test_problems\\ewd.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn djikstra() {
        let (result, _) = prove("test_problems\\djikstra.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn djikstra_negated() {
        let (result, _) = prove("test_problems\\djikstra_negated.p", false, 300);
        assert_eq!(result, ProofAttemptResult::CounterSatisfiable);
    }
    
    #[test]
    fn group_left_inverse_means_right_inverse() {
        let (result, _) = prove("test_problems\\group_left_inverse_means_right_inverse.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    #[test]
    fn set_union_is_commutative() {
        let (result, _) = prove("test_problems\\set_union_is_commutative.p", false, 300);
        assert_eq!(result, ProofAttemptResult::Theorem);
    }
    
    // Still haven't translated these as they either take too long to prove or cannot be proven at all currently.
    
    /*
    #[test]
    fn pelletier_34() {
        let result = prove("((exists x. forall y. P(x) <=> P(y)) <=>
                             (exists x. Q(x) <=> forall y. Q(y))) <=>
                            ((exists x. forall y. Q(x) <=> Q(y)) <=>
                             (exists x. P(x) <=> forall y. P(y)))");
        assert_eq!(result, ProofAttemptResult::Unsatisfiable);
    }

    #[test]
    fn pelletier_38() {
        let result = prove("forall x. (P(a()) /\\ (P(x) ==> exists y. (P(y) /\\ R(x, y))) ==> exists z. exists w. (P(z) /\\ R(x, w) /\\ R(w, z))) <=>
                            forall x. ((~P(a()) \\/ P(x) \\/ exists z. exists w. (P(z) /\\ R(x, w) /\\ R(w, z))) /\\
                            (~P(a()) \\/ ~exists y. (P(y) /\\ R(x, y)) \\/ exists z. exists w. (P(z) /\\ R(x, w) /\\ R(w, z))))");
        assert_eq!(result, ProofAttemptResult::Unsatisfiable);
    }

    // Exhibits interesting pathological behaviour
    #[test]
    fn pelletier_42_negated() {
        let result = prove("exists y. forall x. (F(x, y) <=> ~exists z. (F(x, z) /\\ F(z, x)))").0;
        assert_eq!(result, ProofAttemptResult::Satisfiable);
    }

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
        assert_eq!(result, ProofAttemptResult::Unsatisfiable);
    }

    #[test]
    fn pelletier_53() {
        let result = prove("exists x. exists y. (x <> y /\\ forall z. (z = x \\/ z = y)) <=>
                           (exists z. forall x. (exists w. forall y. (F(x, y) <=> y = w) <=> x = z)
                            <=> exists w. forall y. (exists z. forall x. (F(x, y) <=> x = z) <=> y = w))");
        assert_eq!(result, ProofAttemptResult::Unsatisfiable);
    }

    #[test]
    fn pelletier_54() {
        let result = prove("forall y. exists z. forall x. (F(x, z) <=> x = y) 
                            ==> ~exists w. forall x. (F(x, w) <=> forall u. (F(x, u) ==> exists y. (F(y, u) /\\ ~exists z. (F(z, u) /\\ F(z, y)))))").0;
        assert_eq!(result, ProofAttemptResult::Unsatisfiable);
    }

    #[test]
    fn pelletier_73() {
        let result = prove_general("exists x. exists y. exists z. exists w. (O(x) /\\ O(y) /\\ O(z) /\\ O(w) /\\ x <> y /\\ x <> z /\\ x <> w /\\ y <> z /\\ y <> w /\\ z <> w) /\\
                                    exists x. exists y. exists z. (H(x) /\\ H(y) /\\ H(z) /\\ x <> y /\\ x <> z /\\ y <> z /\\ forall w. (H(w) ==> w = x \\/ w = y \\/ w = z)) /\\
                                    forall x. (O(x) ==> exists y. (H(y) /\\ I(x, y))) /\\
                                    forall x. (H(x) ==> forall y. forall z. (O(y) /\\ O(z) /\\ I(y, x) /\\ I(z, x) ==> y = z))", false, false, 30);
        assert_eq!(result, ProofAttemptResult::Unsatisfiable);
    }

    #[test]
    fn wishnu() {
        let result = prove("exists x. (x = f(g(x)) /\\ forall x1. (x1 = f(g(x1)) ==> x = x1)) <=>
                            exists y. (y = g(f(y)) /\\ forall y1. (y1 = g(f(y1)) ==> y = y1))");
        assert_eq!(result, ProofAttemptResult::Unsatisfiable);
    }
    */
}
