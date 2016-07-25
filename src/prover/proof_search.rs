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

use prover::proof_state::ProofState;
use prover::proof_statistics::*;

use prover::simplification::literal_deletion::cheap_simplify;
use prover::simplification::tautology_deletion::trivial;
use prover::simplification::unit_subsumption::unit_subsumed;
use prover::simplification::non_unit_subsumption::non_unit_subsumed;
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
    unit_subsumed(proof_state.get_term_index(), cl) ||
    non_unit_subsumed(proof_state.get_used(), cl)
}

/// A more expensive version of cheap_simplify with more effective rules.
fn simplify(proof_state: &ProofState, cl: &mut Clause) {
    rewrite_literals(proof_state.get_term_ordering(), 
                     proof_state.get_term_index(), 
                     cl);
    cheap_simplify(cl);
    simplify_reflect(proof_state.get_term_index(), cl);
}

/// The main proof search loop.
/// Note that we use the DISCOUNT version of the given clause algorithm.
/// Also note that this function might NEVER terminate, time handling should be done elsewhere.
fn serkr_loop(mut proof_state: ProofState,
              mut var_cnt: i64) {
    assert_eq!(proof_state.get_used_size(), 0);
    set_initial_clauses(proof_state.get_unused_size());
    
    while let Some(mut chosen_clause) = proof_state.pick_best_clause() {
        increment_iteration_count();

        // We start by simplifying the chosen clause as much as possible.
        simplify(&proof_state, &mut chosen_clause);
                 
        // If we derived a contradiction we are done.
        if chosen_clause.is_empty() {
            refutation_was_found();
            return; 
        }

        // Check if the clause is redundant in some way. If it is no need to process it more.
        if trivial(&chosen_clause) {
            increment_trivial_count()
        } else if forward_subsumed(&proof_state, &chosen_clause) {
            increment_forward_subsumed_count();
        } else {
            proof_state.add_to_used(chosen_clause.clone());
            rename_clause(&mut chosen_clause, &mut var_cnt);

            let mut inferred_clauses = Vec::new();
            let sp_count = superposition(proof_state.get_term_ordering(),
                                         &chosen_clause,
                                         proof_state.get_used(),
                                         &mut inferred_clauses);
            let ef_count = equality_factoring(proof_state.get_term_ordering(),
                                              &chosen_clause,
                                              &mut inferred_clauses);
            let er_count = equality_resolution(proof_state.get_term_ordering(),
                                               &chosen_clause,
                                               &mut inferred_clauses);
            add_superposition_inferred_count(sp_count);
            add_equality_factoring_inferred_count(ef_count);
            add_equality_resolution_inferred_count(er_count);

            for mut cl in inferred_clauses.into_iter() {
                // Simplification need to be done before triviality checking.
                // Consider the clause x <> y, y <> z, x = z which is clearly a tautology.
                // We cannot detect it as a tautology with a pure syntactical check,
                // unless we first simplify it with destructive equality resolution.
                cheap_simplify(&mut cl);
                if trivial(&cl) {
                    increment_trivial_inference_count();
                } else {
                    proof_state.add_to_unused(cl);
                }
            }
        }
    }
}

/// Attempts to prove the stuff in the TPTP file at the location given.
/// First we can decide whether we want to use LPO or KBO.
/// Then there is the option for the renaming limit for CNF translation.
/// Note that this function might NEVER terminate, time handling should be done outside this.
#[cfg_attr(feature="clippy", allow(single_match_else))]
pub fn prove(s: &str,
             use_lpo: bool,
             renaming_limit: u64) {
    reset_statistics();
    // First we obviously need to parse the file.
    let (mut axioms, conjectures, mut renaming_info) = match tptp_to_cnf_ast(s) {
        Ok(res) => res,
        Err(_) => {
            search_has_finished();
            return; // (ProofResult::Error(s), ProofStatistics::new());
        }
    };
    set_parsing_finished();
    set_contains_conjectures(!conjectures.is_empty());

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
        refutation_was_found();
    } else if cnf_f == Formula::True {
    } else {
        let flattened_cnf_f = flatten_cnf(cnf_f);
        let term_ordering = create_term_ordering(use_lpo, &flattened_cnf_f);
        let proof_state = ProofState::new(flattened_cnf_f, term_ordering);
        serkr_loop(proof_state,
                   renaming_info.get_newest_variable_id());
    }
    search_has_finished();
}

#[cfg(test)]
mod test {
    use super::prove;
    use prover::proof_result::ProofResult;
    use prover::proof_statistics::get_proof_result;
    
    // Due to the use of global variables we cannot run the tests here in parallel.
    // Instead we have one gigantic test.
    
    #[test]
    fn test_problem_suite() {
        // The normal pelletier problems, with some missing for various reasons.
        pelletier_1();
        pelletier_2();
        pelletier_3();
        pelletier_4();
        pelletier_5();
        pelletier_6();
        pelletier_7();
        pelletier_8();
        pelletier_9();
        pelletier_10();
        pelletier_11();
        pelletier_12();
        pelletier_13();
        pelletier_14();
        pelletier_15_original();
        pelletier_15_errata();
        pelletier_16();
        pelletier_17();
        pelletier_18();
        pelletier_19();
        pelletier_20();
        pelletier_21();
        pelletier_22();
        pelletier_23();
        pelletier_24();
        pelletier_25();
        pelletier_26();
        pelletier_27();
        pelletier_28_original();
        pelletier_28_errata();
        pelletier_29();
        pelletier_30();
        pelletier_31();
        pelletier_32();
        pelletier_33();
        // pelletier_34();
        pelletier_35();
        pelletier_36();
        pelletier_37();
        // pelletier_38();
        pelletier_39();
        pelletier_40_original();
        pelletier_40_errata();
        pelletier_41();
        pelletier_42();
        pelletier_43();
        pelletier_44();
        pelletier_45();
        pelletier_46();
        // pelletier_47();
        pelletier_48();
        pelletier_49();
        pelletier_50();
        pelletier_51();
        pelletier_52();
        // pelletier_53();
        // pelletier_54();
        pelletier_55();
        pelletier_56();
        pelletier_57();
        pelletier_58();
        pelletier_59();
        pelletier_60();
        pelletier_61();
        pelletier_62_errata_errata();
        pelletier_63();
        pelletier_64();
        pelletier_65();
        pelletier_72();
        
        // Some negated problems as well for variety.
        pelletier_1_negated();
        pelletier_8_negated();
        pelletier_18_negated();
        pelletier_22_negated();
        pelletier_30_negated();
        pelletier_35_negated();
        pelletier_50_negated();
        pelletier_56_negated();
        
        // Other problems.
        davis_putnam();
        los();
        ewd();
        djikstra();
        djikstra_negated();
        group_left_inverse_means_right_inverse();
        set_union_is_commutative();
    }

    fn pelletier_1() {
        prove("test_problems/p1.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_1_negated() {
        prove("test_problems/p1n.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::CounterSatisfiable);
    }

    fn pelletier_2() {
        prove("test_problems/p2.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_3() {
        prove("test_problems/p3.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_4() {
        prove("test_problems/p4.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_5() {
        prove("test_problems/p5.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_6() {
        prove("test_problems/p6.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_7() {
        prove("test_problems/p7.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_8() {
        prove("test_problems/p8.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_8_negated() {
        prove("test_problems/p8n.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::CounterSatisfiable);
    }


    fn pelletier_9() {
        prove("test_problems/p9.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_10() {
        prove("test_problems/p10.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_11() {
        prove("test_problems/p11.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_12() {
        prove("test_problems/p12.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_13() {
        prove("test_problems/p13.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_14() {
        prove("test_problems/p14.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_15_original() {
        prove("test_problems/p15o.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::CounterSatisfiable);
    }

    fn pelletier_15_errata() {
        prove("test_problems/p15e.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_16() {
        prove("test_problems/p16.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_17() {
        prove("test_problems/p17.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_18() {
        prove("test_problems/p18.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_18_negated() {
        prove("test_problems/p18n.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::CounterSatisfiable);
    }

    fn pelletier_19() {
        prove("test_problems/p19.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_20() {
        prove("test_problems/p20.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_21() {
        prove("test_problems/p21.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_22() {
        prove("test_problems/p22.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_22_negated() {
        prove("test_problems/p22n.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::CounterSatisfiable);
    }

    fn pelletier_23() {
        prove("test_problems/p23.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_24() {
        prove("test_problems/p24.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_25() {
        prove("test_problems/p25.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_26() {
        prove("test_problems/p26.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_27() {
        prove("test_problems/p27.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_28_original() {
        prove("test_problems/p28o.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::CounterSatisfiable);
    }

    fn pelletier_28_errata() {
        prove("test_problems/p28e.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_29() {
        prove("test_problems/p29.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_30() {
        prove("test_problems/p30.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_30_negated() {
        prove("test_problems/p30n.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::CounterSatisfiable);
    }

    fn pelletier_31() {
        prove("test_problems/p31.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_32() {
        prove("test_problems/p32.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_33() {
        prove("test_problems/p33.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_35() {
        prove("test_problems/p35.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_35_negated() {
        prove("test_problems/p35n.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::CounterSatisfiable);
    }

    fn pelletier_36() {
        prove("test_problems/p36.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_37() {
        prove("test_problems/p37.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_39() {
        prove("test_problems/p39.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_40_original() {
        prove("test_problems/p40o.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_40_errata() {
        prove("test_problems/p40e.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_41() {
        prove("test_problems/p41.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_42() {
        prove("test_problems/p42.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_43() {
        prove("test_problems/p43.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_44() {
        prove("test_problems/p44.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_45() {
        prove("test_problems/p45.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_46() {
        prove("test_problems/p46.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_48() {
        prove("test_problems/p48.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_49() {
        prove("test_problems/p49.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_50() {
        prove("test_problems/p50.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_50_negated() {
        prove("test_problems/p50n.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::CounterSatisfiable);
    }

    fn pelletier_51() {
        prove("test_problems/p51.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_52() {
        prove("test_problems/p52.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_55() {
        prove("test_problems/p55.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_56() {
        prove("test_problems/p56.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_56_negated() {
        prove("test_problems/p56n.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::CounterSatisfiable);
    }

    fn pelletier_57() {
        prove("test_problems/p57.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_58() {
        prove("test_problems/p58.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_59() {
        prove("test_problems/p59.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_60() {
        prove("test_problems/p60.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_61() {
        prove("test_problems/p61.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_62_errata_errata() {
        prove("test_problems/p62ee.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_63() {
        prove("test_problems/p63.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_64() {
        prove("test_problems/p64.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_65() {
        prove("test_problems/p65.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn pelletier_72() {
        prove("test_problems/p72.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Unsatisfiable);
    }

    fn davis_putnam() {
        prove("test_problems/davis_putnam.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn los() {
        prove("test_problems/los.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn ewd() {
        prove("test_problems/ewd.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn djikstra() {
        prove("test_problems/djikstra.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn djikstra_negated() {
        prove("test_problems/djikstra_negated.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::CounterSatisfiable);
    }

    fn group_left_inverse_means_right_inverse() {
        prove("test_problems/group_left_inverse_means_right_inverse.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }

    fn set_union_is_commutative() {
        prove("test_problems/set_union_is_commutative.p", false, 32);
        assert_eq!(get_proof_result(), ProofResult::Theorem);
    }
}
