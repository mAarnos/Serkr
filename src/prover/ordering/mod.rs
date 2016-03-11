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

/// Contains a generic term ordering for ordering terms and literals.
pub mod term_ordering;

/// Contains function symbol precedence stuff, used by both LPO and KBO.
pub mod precedence;

/// Contains weight function stuff which is used by KBO.
pub mod weight;

/// Contains an implementation of the lexicographic path ordering.
mod lpo;

/// Contains an implementation of the Knuth-Bendix ordering.
mod kbo;
