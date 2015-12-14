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

/// Defines a substitution of variables for terms.
pub mod substitution;

/// Contains functions for matching, i.e. finding a sigma so that s\sigma = t
pub mod matching;

/// Contains functions for unification, i.e. finding a sigma so that s\sigma = t\sigma
/// In a certain sense matching is a cut-down version of unification, hence the name for the module.
pub mod full_unification;

