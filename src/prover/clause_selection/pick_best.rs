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

use std::collections::{HashMap, BinaryHeap};
use prover::data_structures::clause::Clause;
use prover::clause_selection::clause_weight::ClauseWeight;

/// Chooses the best clause according to the ordering given, if possible.
pub fn pick_best_clause(clauses: &mut HashMap<u64, Clause>, clause_order: &mut BinaryHeap<ClauseWeight>) -> Option<Clause> {
    assert!(clauses.len() <= clause_order.len());
    
    // Other heuristics might have already picked the best one, so we need to loop.
    while let Some(clause_weight) = clause_order.pop() {
        if let Some(clause) = clauses.remove(&clause_weight.get_id()) {
            assert_eq!(clause.get_id(), clause_weight.get_id());
            return Some(clause);
        }
    }
    
    None
}

/// Picks the next heuristic to use. Also updates `current_heuristic_count`.
pub fn choose_heuristic(heuristic_use_count: &[usize], current_heuristic_count: &mut usize) -> usize {
    let mut running_total = 0;
    
    for (i, x) in heuristic_use_count.iter().enumerate() {
        running_total += *x;
        if *current_heuristic_count < running_total {
            *current_heuristic_count += 1;
            return i;
        }
    }
    
    // If we didn't find anything we need to reset the counter.
    assert_eq!(*current_heuristic_count, heuristic_use_count.iter().fold(0, |acc, x| acc + x));
    *current_heuristic_count = 1;
    0
}