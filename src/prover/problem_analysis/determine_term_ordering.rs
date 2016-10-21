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

use prover::data_structures::clause::Clause;
use prover::data_structures::term::Term;
use prover::ordering::term_ordering::TermOrdering;
use prover::ordering::precedence::Precedence;
use prover::ordering::weight::Weight;
use utils::hash_map::HashMap;

fn single_unary_function_in_term(t: &Term, found_unary: &mut Option<i64>) -> bool {
    if t.is_function() {
        if t.get_arity() == 1 {
            if let Some(id) = *found_unary {
                if id != t.get_id() {
                    *found_unary = None;
                    return true;
                }
            } else {
                *found_unary = Some(t.get_id());
            }
        }
        
        t.iter().any(|sub_t| single_unary_function_in_term(sub_t, found_unary))
    } else {
        false
    }
}

/// If the problem contains one unary function, this function finds it.
fn single_unary_function(clauses: &[Clause]) -> Option<i64> {
    let mut found_unary = None;

    for cl in clauses {
        for l in cl.iter() {
            for t in l.iter() {
                if single_unary_function_in_term(t, &mut found_unary) {
                    assert!(found_unary.is_none());
                    break;
                }
            }
        }
    }

    found_unary
}

/// Updates the function symbol count based on the term t.
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

/// Runs through all terms in a problem, and counts how many times each function symbol appears.
fn create_function_symbol_count(clauses: &[Clause]) -> HashMap<i64, i64> {
    let mut counts = HashMap::default();

    for cl in clauses {
        for l in cl.iter() {
            for t in l.iter() {
                update_function_symbol_count(&mut counts, t)
            }
        }
    }

    counts
}

/// Creates an appropriate term ordering for a given problem.
/// Currently we let the user choose whether to use LPO or KBO, with the default being KBO.
/// Automatically choosing between them in case the user doesn't care would be nice.
pub fn create_term_ordering(lpo_over_kbo: bool, clauses: &[Clause]) -> TermOrdering {
    if lpo_over_kbo {
        TermOrdering::LPO(Precedence::default())
    } else {
        let counts = create_function_symbol_count(clauses);
        TermOrdering::KBO(Precedence::ArityFrequency(counts),
                          Weight::SimpleWeight,
                          single_unary_function(clauses))
    }
}
