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

//! Contains stuff for converting raw first order formulas into CNF.
//!
//! Most of the stuff here is based on "Andreas Nonnengart, Christoph Weidenbach, Im Stadtwald, Im Stadtwald (2001). Computing Small Clause Normal Forms".

// A good guideline for algorithms in this module is that everything should be O(n log n) at worst if possible.
// There always is some problem in the TPTP library which will FUCK YOU UP if you do not keep that in mind.

/// Contains the AST used during the CNF transformation.
pub mod ast;

/// Contains stuff dealing with converting predicate and term names to a more suitable format.
pub mod renaming_info;

/// Contains functions for turning the TPTP parser AST into the CNF AST.
pub mod ast_transformer;

mod free_variables;

mod simplify;
mod formula_renaming;
mod nnf;
mod variable_renaming;
mod standard_skolemization;
mod distribute_ors_over_ands;

/// Contains the standard version of the CNF transformation algorithm.
/// Since we use standard (not advanced) skolemization the name is like it is.
pub mod standard_cnf;
