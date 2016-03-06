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

//! Contains the main proof search.

/// Contains all the simplification rules used by the proof search.
mod simplification;

/// Contains different term orderings.
mod ordering;

/// Contains all the inference rules used by the proof search.
mod inference;

/// Contains stuff for performing both unification and matching.
mod unification;

/// Contains different (non-generic) data structures used by the proof search.
/// Generic data structures should go to utils.
mod data_structures;

/// Contains functions for automatically determining good prover settings for a given problem.
mod problem_analysis;

// TODO: figure out a new name for this.
// flatten is just not descriptive and it also means something else on top of that.
mod flatten_cnf;

/// Contains stuff on statistics collected during a proof search.
mod proof_statistics;

/// Contains stuff for keeping track of the current proof state during a proof search.
mod proof_state;

/// Contains the core of the program, the proof search.
pub mod proof_search;
