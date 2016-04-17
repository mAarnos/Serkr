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
use prover::clause_selection::clause_weight::ClauseWeight;

/// Different heuristics for ordering clauses.
#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum Heuristic {
    /// Heuristic based on the symbol count of the clause.
    /// First number is the value to give to functions.
    /// The other number is the value to give to variables.
    Size(u64, u64),
    /// Heuristic based on clause age.
    Age,
}

impl Heuristic {
    /// Creates a new ClauseWeight corresponding to the heuristic for a given clause.
    pub fn new_clauseweight(&self, cl: &Clause) -> ClauseWeight {
        match *self {
            Heuristic::Size(f_val, v_val) => ClauseWeight::new_size_weight(cl, f_val, v_val),
            Heuristic::Age => ClauseWeight::new_age_weight(cl)
        }
    }
}
