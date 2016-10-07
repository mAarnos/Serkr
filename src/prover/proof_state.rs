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
use prover::ordering::term_ordering::TermOrdering;
use prover::clause_selection::clause_weight::ClauseWeight;
use prover::clause_selection::heuristic::Heuristic;
use prover::clause_selection::pick_best::{pick_best_clause, choose_heuristic};
use prover::data_structures::pd_tree::PDTree;

/// Contains the current proof state.
pub struct ProofState {
    used_clauses: Vec<Clause>,
    unused_clauses: HashMap<u64, Clause>,
    term_ordering: TermOrdering,
    clause_order: Vec<BinaryHeap<ClauseWeight>>,
    heuristic_order: Vec<Heuristic>,
    heuristic_use_count: Vec<usize>,
    current_heuristic_count: usize,
    term_index: PDTree,
    id_count: u64
}

impl ProofState {
    /// Creates a new proof state.
    pub fn new(preprocessed_clauses: Vec<Clause>, term_order: TermOrdering) -> ProofState {
        let mut state = ProofState {
            used_clauses: Vec::new(),
            unused_clauses: HashMap::new(),
            term_ordering: term_order,
            clause_order: vec![BinaryHeap::new(), BinaryHeap::new()],
            heuristic_order: vec![Heuristic::Size(2, 1)],
            heuristic_use_count: vec![4],
            current_heuristic_count: 0,
            term_index: PDTree::new(),
            id_count: 0
        };
        
        for cl in preprocessed_clauses.into_iter() {
            state.add_to_unused(cl);
        }
        
        state
    }

    /// Get the amount of used clauses.
    pub fn get_used_size(&self) -> usize {
        self.used_clauses.len()
    }

    /// Get the amount of unused clauses.
    pub fn get_unused_size(&self) -> usize {
        self.unused_clauses.len()
    }

    /// Picks the best clause from unused and removes it from there.
    pub fn pick_best_clause(&mut self) -> Option<Clause> {
        let i = choose_heuristic(&self.heuristic_use_count, &mut self.current_heuristic_count);
        pick_best_clause(&mut self.unused_clauses, &mut self.clause_order[i])
    }

    /// Adds the given clause to used clauses.
    pub fn add_to_used(&mut self, cl: Clause) {
        self.term_index.add_clause_to_index(&cl);
        self.used_clauses.push(cl);
    }

    /// Adds the given clause to unused clauses.
    pub fn add_to_unused(&mut self, mut cl: Clause) {
        // Give a unique ID to the clause.
        cl.set_id(self.id_count);
        self.id_count += 1;
        for i in 0..self.heuristic_order.len() {
            let cw = self.heuristic_order[i].new_clauseweight(&cl);
            self.clause_order[i].push(cw);
        }
        self.unused_clauses.insert(cl.get_id(), cl);
    }

    /// Get a reference term ordering used.
    pub fn get_term_ordering(&self) -> &TermOrdering {
        &self.term_ordering
    }
    
    /// Get a reference to the term index.
    pub fn get_term_index(&self) -> &PDTree {
        &self.term_index
    }
    
    /// Get a reference to the used clauses.
    pub fn get_used(&self) -> &Vec<Clause> {
        &self.used_clauses
    }
}
