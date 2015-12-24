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

/// All occurences of String are replaced with i64 for convenience.
/// We could theoretically have different ASTs for different phases of the CNF transformation.
/// However, it seems to me that some invariants are impossible to capture.

/// Data type for FOL terms. 
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
#[allow(missing_docs)]
#[allow(variant_size_differences)]
pub enum Term {
    Variable(i64),
    Function(i64, Vec<Term>)
}

/// Data type for FOL formulas. 
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
#[allow(missing_docs)]
pub enum Formula {
    True,
    False,
    Predicate(i64, Vec<Term>),
    Not(Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Implies(Box<Formula>, Box<Formula>),
    Equivalent(Box<Formula>, Box<Formula>),
    Forall(i64, Box<Formula>),
    Exists(i64, Box<Formula>),
}
