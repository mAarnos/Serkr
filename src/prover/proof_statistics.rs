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

/// Contains statistics for the current proof search.
#[derive(Eq, PartialEq, Clone, Debug, Copy)]
#[allow(missing_docs)]
pub struct ProofStatistics {
    pub initial_clauses: usize,
    pub elapsed_ms: u64,
    pub iterations: u64,
    pub trivial_count: u64,
    pub fs_count: u64,
    pub bs_count: u64,
    pub sp_count: u64,
    pub ef_count: u64,
    pub er_count: u64,
    pub trivial_inference_count: u64,
}

impl ProofStatistics {
    /// Creates a new statistics container.
    pub fn new() -> ProofStatistics {
        ProofStatistics {
            initial_clauses: 0,
            elapsed_ms: 0,
            iterations: 0,
            trivial_count: 0,
            fs_count: 0,
            bs_count: 0,
            sp_count: 0,
            ef_count: 0,
            er_count: 0,
            trivial_inference_count: 0,
        }
    }
    
    /// Returns the amount of nonredundant processed clauses.
    pub fn nonredudant_processed_count(&self) -> u64 {
        self.iterations - self.trivial_count - self.fs_count
    }
    
    /// Returns the amount of inferred clauses.
    pub fn inferred_clauses_count(&self) -> u64 {
        self.sp_count + self.ef_count + self.er_count
    }
    
    /// Returns the amount of nontrivial inferred clauses.
    /// That is, the amount of clauses not immediately discarded.
    pub fn nontrivial_inferred_clauses_count(&self) -> u64 {
        self.inferred_clauses_count() - self.trivial_inference_count
    }
}

impl Default for ProofStatistics {
    fn default() -> ProofStatistics {
        ProofStatistics::new()
    }
}

#[cfg(test)]
mod test {}
