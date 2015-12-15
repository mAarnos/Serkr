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

use std::collections::BinaryHeap;
use prover::clause::Clause;
use prover::ordering::term_ordering::TermOrdering;

/// Contains the current proof state, that is, the used, the unused clauses and the term ordering used.
pub struct ProofState {
    used_clauses: Vec<Clause>,
    unused_clauses: BinaryHeap<Clause>,
    term_ordering: TermOrdering,
}

impl ProofState {
    /// Creates a new proof state with the used clauses empty, the preprocessed_clauses in unused and the given term ordering.
    pub fn new(preprocessed_clauses: Vec<Clause>, term_order: TermOrdering) -> ProofState {
        ProofState { used_clauses: Vec::new(), 
                     unused_clauses: preprocessed_clauses.into_iter().collect(), 
                     term_ordering: term_order }
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
        self.unused_clauses.pop()
    }
          
    /// Adds the given clause to used clauses.
    pub fn add_to_used(&mut self, cl: Clause) {
        self.used_clauses.push(cl);
    }
    
    /// Adds the given clause to unused clauses.
    pub fn add_to_unused(&mut self, cl: Clause) {
        self.unused_clauses.push(cl);
    }
  
    /// Get a reference term ordering used.
    pub fn get_term_ordering(&self) -> &TermOrdering {
        &self.term_ordering
    }
    
    /// Get a reference to the used clauses.
    pub fn get_used(&self) -> &Vec<Clause> {
        &self.used_clauses
    }
   
    /// Get a mutable reference to the used clauses.
    // TODO: can we get rid of this?
    pub fn get_used_mut(&mut self) -> &mut Vec<Clause> {
        &mut self.used_clauses
    }
}