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

// This entire file is a dirty fucking hack. 
// We now have global state, so we can NEVER have several proof searches running concurrently.
// Notably, proof search tests have to be clustered.
// I haven't figured out a better way to get the statistics out in case the search thread gets stuck.

use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering, ATOMIC_USIZE_INIT, ATOMIC_BOOL_INIT};
use prover::proof_result::ProofResult;

static INITIAL_CLAUSES: AtomicUsize = ATOMIC_USIZE_INIT;
static ITERATIONS: AtomicUsize = ATOMIC_USIZE_INIT;
static TRIVIAL_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;
static FS_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;
static BS_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;
static SP_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;
static EF_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;
static ER_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;
static TRIVIAL_INFERENCE_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

static CONTAINS_CONJECTURES: AtomicBool = ATOMIC_BOOL_INIT;
static PARSING_FINISHED: AtomicBool = ATOMIC_BOOL_INIT;
static SEARCH_FINISHED: AtomicBool = ATOMIC_BOOL_INIT;
static REFUTATION_FOUND: AtomicBool = ATOMIC_BOOL_INIT;

/// Resets all the statistics.
pub fn reset_statistics() {
    set_initial_clauses(0);
    ITERATIONS.store(0, Ordering::SeqCst);
    TRIVIAL_COUNT.store(0, Ordering::SeqCst);
    FS_COUNT.store(0, Ordering::SeqCst);
    BS_COUNT.store(0, Ordering::SeqCst);
    SP_COUNT.store(0, Ordering::SeqCst);
    EF_COUNT.store(0, Ordering::SeqCst);
    ER_COUNT.store(0, Ordering::SeqCst);
    TRIVIAL_INFERENCE_COUNT.store(0, Ordering::SeqCst);
    
    CONTAINS_CONJECTURES.store(false, Ordering::SeqCst);
    PARSING_FINISHED.store(false, Ordering::SeqCst);
    SEARCH_FINISHED.store(false, Ordering::SeqCst);
    REFUTATION_FOUND.store(false, Ordering::SeqCst);
}

/// Set the amount of initial clauses.
pub fn set_initial_clauses(x: usize) {
    INITIAL_CLAUSES.store(x, Ordering::SeqCst);
}

/// Get the amount of initial clauses.
pub fn get_initial_clauses() -> usize {
    INITIAL_CLAUSES.load(Ordering::SeqCst)
}

/// Increment the iteration count.
pub fn increment_iteration_count() {
    ITERATIONS.fetch_add(1, Ordering::SeqCst);
}

/// Get the iteration count.
pub fn get_iteration_count() -> usize {
    ITERATIONS.load(Ordering::SeqCst)
}

/// Increment the amount of trivial clauses discovered during the proof search.
pub fn increment_trivial_count() {
    TRIVIAL_COUNT.fetch_add(1, Ordering::SeqCst);
}

/// Get the amount of trivial clauses discovered during the proof search.
pub fn get_trivial_count() -> usize {
    TRIVIAL_COUNT.load(Ordering::SeqCst)
}

/// Increment the amount of forward subsumed clauses during the proof search.
pub fn increment_forward_subsumed_count() {
    FS_COUNT.fetch_add(1, Ordering::SeqCst);
}

/// Get the amount of forward subsumed clauses during the proof search.
pub fn get_forward_subsumed_count() -> usize {
    FS_COUNT.load(Ordering::SeqCst)
}

/// Add to the amount of backward subsumed clauses during the proof search.
pub fn add_backward_subsumed_count(x: usize) {
    BS_COUNT.fetch_add(x, Ordering::SeqCst);
}

/// Get the amount of backward subsumed clauses during the proof search.
pub fn get_backward_subsumed_count() -> usize {
    BS_COUNT.load(Ordering::SeqCst)
}

/// Add to the amount of clauses inferred by positive and negative superposition.
pub fn add_superposition_inferred_count(x: usize) {
    SP_COUNT.fetch_add(x, Ordering::SeqCst);
}

/// Get the amount of clauses inferred by positive and negative superposition.
pub fn get_superposition_inferred_count() -> usize {
    SP_COUNT.load(Ordering::SeqCst)
}

/// Add to the amount of clauses inferred by equality factoring.
pub fn add_equality_factoring_inferred_count(x: usize) {
    EF_COUNT.fetch_add(x, Ordering::SeqCst);
}

/// Get the amount of clauses inferred by equality factoring.
pub fn get_equality_factoring_inferred_count() -> usize {
    EF_COUNT.load(Ordering::SeqCst)
}

/// Add to the amount of clauses inferred by equality resolution.
pub fn add_equality_resolution_inferred_count(x: usize) {
    ER_COUNT.fetch_add(x, Ordering::SeqCst);
}

/// Get the amount of clauses inferred by equality resolution.
pub fn get_equality_resolution_inferred_count() -> usize {
    ER_COUNT.load(Ordering::SeqCst)
}

/// Increment the amount of trivial inferences during the proof search.
pub fn increment_trivial_inference_count() {
    TRIVIAL_INFERENCE_COUNT.fetch_add(1, Ordering::SeqCst);
}

/// Get the amount of trivial inferences during the proof search.
pub fn get_trivial_inference_count() -> usize {
    TRIVIAL_INFERENCE_COUNT.load(Ordering::SeqCst)
}

/// Set whether the problem contains conjectures or not.
pub fn set_contains_conjectures(x: bool) {
    CONTAINS_CONJECTURES.store(x, Ordering::SeqCst);
}

/// Check if the problem to analyze contains conjectures.
fn contains_conjectures() -> bool {
    CONTAINS_CONJECTURES.load(Ordering::SeqCst)
}

/// Set a flag for finishing parsing.
pub fn set_parsing_finished() {
    PARSING_FINISHED.store(true, Ordering::SeqCst);
}

/// Check if parsing was finished
pub fn has_parsing_finished() -> bool {
    PARSING_FINISHED.load(Ordering::SeqCst)
}

/// Sets a flag for a refutation found during the proof search.
pub fn refutation_was_found() {
    REFUTATION_FOUND.store(true, Ordering::SeqCst)
}

/// Checks if a refutation was found.
pub fn was_refutation_found() -> bool {
    REFUTATION_FOUND.load(Ordering::SeqCst)
}

/// Sets a flag for a finished search.
pub fn search_has_finished() {
    SEARCH_FINISHED.store(true, Ordering::SeqCst)
}

/// Checks if the proof search has finished.
pub fn has_search_finished() -> bool {
    SEARCH_FINISHED.load(Ordering::SeqCst)
}

/// Get the amount of nonredundant analyzed clauses.
pub fn get_nonredundant_analyzed_count() -> usize {
    get_iteration_count() - get_trivial_count() - get_forward_subsumed_count()
}

/// Get the amount of inferred clauses.
pub fn get_inferred_count() -> usize {
    get_superposition_inferred_count() + 
    get_equality_factoring_inferred_count() + 
    get_equality_resolution_inferred_count()
}

/// Get the amount of nontrivial inferred clauses.
pub fn get_nontrivial_inferred_count() -> usize {
    get_inferred_count() - get_trivial_inference_count()
}

/// Get the proof result.
pub fn get_proof_result() -> ProofResult {
    if !has_parsing_finished() && has_search_finished() {
        ProofResult::Error("Parsing error".to_owned())
    } else if !has_parsing_finished() || !has_search_finished() {
        ProofResult::Timeout
    } else if was_refutation_found() {
        ProofResult::new_refutation(contains_conjectures())
    } else {
        ProofResult::new_saturation(contains_conjectures())
    }
}

#[cfg(test)]
mod test {}
