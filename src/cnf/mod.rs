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

/// Contains the AST used during the CNF transformation.
#[cfg_attr(rustfmt, rustfmt_skip)]
pub mod ast;

/// Contains stuff dealing with converting function, predicate, and term names to a more suitable format.
pub mod renaming_info;

/// Contains functions for turning the internal parser AST into the CNF AST.
#[cfg_attr(rustfmt, rustfmt_skip)]
pub mod ast_transformer_internal;

/// Contains functions for turning the TPTP parser AST into the CNF AST.
#[cfg_attr(rustfmt, rustfmt_skip)]
pub mod ast_transformer_tptp;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod free_variables;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod simplify;
#[cfg_attr(rustfmt, rustfmt_skip)]
#[allow(dead_code)]
mod formula_renaming;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod nnf;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod variable_renaming;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod standard_skolemization;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod pull_out_quantifiers;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod drop_universal_quantifiers;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod distribute_ors_over_ands;

/// Contains a naive version of the CNF transformation algorithm.
#[cfg_attr(rustfmt, rustfmt_skip)]
pub mod naive_cnf;
