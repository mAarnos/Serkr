// Serkr - An automated theorem prover. Copyright (C) 2015-2016 Mikko Aarnos.
//
// Serkr is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Serkr is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Serkr. If not, see <http://www.gnu.org/licenses/>.
//

use std::collections::HashMap;
use prover::flatten_cnf::flatten_cnf;

use prover::data_structures::clause::Clause;

use prover::proof_result::ProofResult;
use prover::proof_statistics::ProofStatistics;
use prover::proof_state::ProofState;

use prover::simplification::literal_deletion::cheap_simplify;
use prover::simplification::tautology_deletion::trivial;
use prover::simplification::subsumption::subsumed;
use prover::simplification::equality_subsumption::forward_equality_subsumed;
use prover::simplification::rewriting::rewrite_literals;
use prover::simplification::simplify_reflect::simplify_reflect;

use prover::inference::equality_resolution::equality_resolution;
use prover::inference::equality_factoring::equality_factoring;
use prover::inference::superposition::superposition;

use prover::problem_analysis::determine_term_ordering::create_term_ordering;

use cnf::ast::Formula;
use cnf::ast_transformer::tptp_to_cnf_ast;
use cnf::standard_cnf::cnf;
use utils::stopwatch::Stopwatch;

/// Rename a clause so that it contains no variables in common with any other clause.
fn rename_clause(cl: &mut Clause, var_cnt: &mut i64) {
    let mut var_map = HashMap::<i64, i64>::new();
    for l in cl.iter_mut() {
        l.get_lhs_mut().rename_no_common(&mut var_map, var_cnt);
        l.get_rhs_mut().rename_no_common(&mut var_map, var_cnt);
    }
}

/// Checks if a given clause is subsumed by the set of used clauses.
fn forward_subsumed(proof_state: &ProofState, cl: &Clause) -> bool {
    forward_equality_subsumed(proof_state.get_term_index(), cl) ||
    proof_state.get_used()
               .iter()
               .any(|cl2| subsumed(cl2, cl))
}

/// A more expensive version of cheap_simplify with more effective rules.
fn simplify(proof_state: &ProofState, cl: &mut Clause) {
    simplify_reflect(proof_state.get_term_index(), cl);
    rewrite_literals(proof_state.get_term_ordering(), 
                     proof_state.get_term_index(), 
                     cl);
}

/// The main proof search loop.
/// Note that we use the DISCOUNT version of the given clause algorithm.
fn serkr_loop(mut proof_state: ProofState,
              mut var_cnt: i64,
              max_time_in_ms: u64,
              contains_conjectures: bool)
              -> (ProofResult, ProofStatistics) {
    assert_eq!(proof_state.get_used_size(), 0);

    let mut sw = Stopwatch::new();
    let mut stats = ProofStatistics::new();
    stats.initial_clauses = proof_state.get_unused_size();

    sw.start();

    while let Some(mut chosen_clause) = proof_state.pick_best_clause() {
        stats.iterations += 1;
        stats.elapsed_ms = sw.elapsed_ms();

        // If we have spent too much time already just give up.
        if stats.elapsed_ms > max_time_in_ms {
            return (ProofResult::Timeout, stats);
        }

        // We start by simplifying the chosen clause as much as possible.
        simplify(&proof_state, &mut chosen_clause);
                 
        // If we derived a contradiction we are done.
        if chosen_clause.is_empty() {
            return (ProofResult::new_refutation(contains_conjectures), stats);
        }

        // Check if the clause is redundant in some way. If it is no need to process it more.
        if trivial(&chosen_clause) {
            stats.trivial_count += 1;
        } else if forward_subsumed(&proof_state, &chosen_clause) {
            stats.fs_count += 1;
        } else {
            // stats.bs_count += backward_subsumption(&mut proof_state, &chosen_clause);
            proof_state.add_to_used(chosen_clause.clone());
            rename_clause(&mut chosen_clause, &mut var_cnt);

            let mut inferred_clauses = Vec::new();
            stats.sp_count += superposition(proof_state.get_term_ordering(),
                                            &chosen_clause,
                                            proof_state.get_used(),
                                            &mut inferred_clauses);
            stats.er_count += equality_resolution(proof_state.get_term_ordering(),
                                                  &chosen_clause,
                                                  &mut inferred_clauses);
            stats.ef_count += equality_factoring(proof_state.get_term_ordering(),
                                                 &chosen_clause,
                                                 &mut inferred_clauses);

            for mut cl in inferred_clauses.into_iter() {
                // Simplification need to be done before triviality checking.
                // Consider the clause x <> y, y <> z, x = z which is clearly a tautology.
                // We cannot detect it as a tautology with a pure syntactical check,
                // unless we first simplify it with destructive equality resolution.
                cheap_simplify(&mut cl);
                if trivial(&cl) {
                    stats.trivial_inference_count += 1;
                } else {
                    proof_state.add_to_unused(cl);
                }
            }
        }
    }

    (ProofResult::new_saturation(contains_conjectures), stats)
}

/// Attempts to prove the stuff in the TPTP file at the location given.
/// First we can decide whether we want to use LPO or KBO.
/// Then there is the option of limiting the time of the proof search.
pub fn prove(s: &str,
             use_lpo: bool,
             max_time_in_s: u64,
             renaming_limit: u64)
             -> (ProofResult, ProofStatistics) {
    // First we obviously need to parse the file.
    let (mut axioms, conjectures, mut renaming_info) = match tptp_to_cnf_ast(s) {
        Ok(res) => res,
        Err(s) => {
            return (ProofResult::Error(s), ProofStatistics::new());
        }
    };
    let contains_conjectures = !conjectures.is_empty();

    // Choose the correct form for the combination of axioms and conjectures.
    let f = if !axioms.is_empty() && !conjectures.is_empty() {
        axioms.push(Formula::Not(Box::new(Formula::And(conjectures))));
        Formula::And(axioms)
    } else if !axioms.is_empty() {
        Formula::And(axioms)
    } else if conjectures.is_empty() {
        panic!("We should always have some clauses");
    } else {
        Formula::And(vec![Formula::Not(Box::new(Formula::And(conjectures)))])
    };
    // And finally transform the whole thing into CNF.
    let cnf_f = cnf(f, &mut renaming_info, renaming_limit);

    if cnf_f == Formula::False {
        (ProofResult::new_refutation(contains_conjectures),
         ProofStatistics::new())
    } else if cnf_f == Formula::True {
        (ProofResult::new_saturation(contains_conjectures),
         ProofStatistics::new())
    } else {
        let flattened_cnf_f = flatten_cnf(cnf_f);
        let term_ordering = create_term_ordering(use_lpo, &flattened_cnf_f);
        let proof_state = ProofState::new(flattened_cnf_f, term_ordering);
        serkr_loop(proof_state,
                   renaming_info.get_newest_variable_id(),
                   max_time_in_s * 1000,
                   contains_conjectures)
    }
}

#[cfg(test)]
mod test {
    use super::prove;
    use prover::proof_result::ProofResult;

    #[test]
    fn pelletier_1() {
        let (result, _) = prove("test_problems/p1.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_1_negated() {
        let (result, _) = prove("test_problems/p1n.p", false, 300, 32);
        assert_eq!(result, ProofResult::CounterSatisfiable);
    }

    #[test]
    fn pelletier_2() {
        let (result, _) = prove("test_problems/p2.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_3() {
        let (result, _) = prove("test_problems/p3.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_4() {
        let (result, _) = prove("test_problems/p4.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_5() {
        let (result, _) = prove("test_problems/p5.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_6() {
        let (result, _) = prove("test_problems/p6.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_7() {
        let (result, _) = prove("test_problems/p7.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_8() {
        let (result, _) = prove("test_problems/p8.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_8_negated() {
        let (result, _) = prove("test_problems/p8n.p", false, 300, 32);
        assert_eq!(result, ProofResult::CounterSatisfiable);
    }

    #[test]
    fn pelletier_9() {
        let (result, _) = prove("test_problems/p9.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_10() {
        let (result, _) = prove("test_problems/p10.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_11() {
        let (result, _) = prove("test_problems/p11.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_12() {
        let (result, _) = prove("test_problems/p12.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_13() {
        let (result, _) = prove("test_problems/p13.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_14() {
        let (result, _) = prove("test_problems/p14.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_15_original() {
        let (result, _) = prove("test_problems/p15o.p", false, 300, 32);
        assert_eq!(result, ProofResult::CounterSatisfiable);
    }

    #[test]
    fn pelletier_15_errata() {
        let (result, _) = prove("test_problems/p15e.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_16() {
        let (result, _) = prove("test_problems/p16.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_17() {
        let (result, _) = prove("test_problems/p17.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_18() {
        let (result, _) = prove("test_problems/p18.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_18_negated() {
        let (result, _) = prove("test_problems/p18n.p", false, 300, 32);
        assert_eq!(result, ProofResult::CounterSatisfiable);
    }

    #[test]
    fn pelletier_19() {
        let (result, _) = prove("test_problems/p19.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_20() {
        let (result, _) = prove("test_problems/p20.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_21() {
        let (result, _) = prove("test_problems/p21.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_22() {
        let (result, _) = prove("test_problems/p22.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_22_negated() {
        let (result, _) = prove("test_problems/p22n.p", false, 300, 32);
        assert_eq!(result, ProofResult::CounterSatisfiable);
    }

    #[test]
    fn pelletier_23() {
        let (result, _) = prove("test_problems/p23.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_24() {
        let (result, _) = prove("test_problems/p24.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_25() {
        let (result, _) = prove("test_problems/p25.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_26() {
        let (result, _) = prove("test_problems/p26.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_27() {
        let (result, _) = prove("test_problems/p27.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_28_original() {
        let (result, _) = prove("test_problems/p28o.p", false, 300, 32);
        assert_eq!(result, ProofResult::CounterSatisfiable);
    }

    #[test]
    fn pelletier_28_errata() {
        let (result, _) = prove("test_problems/p28e.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_29() {
        let (result, _) = prove("test_problems/p29.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_30() {
        let (result, _) = prove("test_problems/p30.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_30_negated() {
        let (result, _) = prove("test_problems/p30n.p", false, 300, 32);
        assert_eq!(result, ProofResult::CounterSatisfiable);
    }

    #[test]
    fn pelletier_31() {
        let (result, _) = prove("test_problems/p31.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_32() {
        let (result, _) = prove("test_problems/p32.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_33() {
        let (result, _) = prove("test_problems/p33.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_35() {
        let (result, _) = prove("test_problems/p35.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_35_negated() {
        let (result, _) = prove("test_problems/p35n.p", false, 300, 32);
        assert_eq!(result, ProofResult::CounterSatisfiable);
    }

    #[test]
    fn pelletier_36() {
        let (result, _) = prove("test_problems/p36.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_37() {
        let (result, _) = prove("test_problems/p37.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_39() {
        let (result, _) = prove("test_problems/p39.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_40_original() {
        let (result, _) = prove("test_problems/p40o.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_40_errata() {
        let (result, _) = prove("test_problems/p40e.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_41() {
        let (result, _) = prove("test_problems/p41.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_42() {
        let (result, _) = prove("test_problems/p42.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_43() {
        let (result, _) = prove("test_problems/p43.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_44() {
        let (result, _) = prove("test_problems/p44.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_45() {
        let (result, _) = prove("test_problems/p45.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_46() {
        let (result, _) = prove("test_problems/p46.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_48() {
        let (result, _) = prove("test_problems/p48.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_49() {
        let (result, _) = prove("test_problems/p49.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_50() {
        let (result, _) = prove("test_problems/p50.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_50_negated() {
        let (result, _) = prove("test_problems/p50n.p", false, 300, 32);
        assert_eq!(result, ProofResult::CounterSatisfiable);
    }

    #[test]
    fn pelletier_51() {
        let (result, _) = prove("test_problems/p51.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_52() {
        let (result, _) = prove("test_problems/p52.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_55() {
        let (result, _) = prove("test_problems/p55.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_56() {
        let (result, _) = prove("test_problems/p56.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_56_negated() {
        let (result, _) = prove("test_problems/p56n.p", false, 300, 32);
        assert_eq!(result, ProofResult::CounterSatisfiable);
    }

    #[test]
    fn pelletier_57() {
        let (result, _) = prove("test_problems/p57.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_58() {
        let (result, _) = prove("test_problems/p58.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_59() {
        let (result, _) = prove("test_problems/p59.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_60() {
        let (result, _) = prove("test_problems/p60.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_61() {
        let (result, _) = prove("test_problems/p61.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_62_errata_errata() {
        let (result, _) = prove("test_problems/p62ee.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_63() {
        let (result, _) = prove("test_problems/p63.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_64() {
        let (result, _) = prove("test_problems/p64.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_65() {
        let (result, _) = prove("test_problems/p65.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn pelletier_72() {
        let (result, _) = prove("test_problems/p72.p", false, 300, 32);
        assert_eq!(result, ProofResult::Unsatisfiable);
    }

    #[test]
    fn davis_putnam() {
        let (result, _) = prove("test_problems/davis_putnam.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn los() {
        let (result, _) = prove("test_problems/los.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn ewd() {
        let (result, _) = prove("test_problems/ewd.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn djikstra() {
        let (result, _) = prove("test_problems/djikstra.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn djikstra_negated() {
        let (result, _) = prove("test_problems/djikstra_negated.p", false, 300, 32);
        assert_eq!(result, ProofResult::CounterSatisfiable);
    }

    #[test]
    fn group_left_inverse_means_right_inverse() {
        let (result, _) = prove("test_problems/group_left_inverse_means_right_inverse.p",
                                false,
                                300,
                                32);
        assert_eq!(result, ProofResult::Theorem);
    }

    #[test]
    fn set_union_is_commutative() {
        let (result, _) = prove("test_problems/set_union_is_commutative.p", false, 300, 32);
        assert_eq!(result, ProofResult::Theorem);
    }
}
