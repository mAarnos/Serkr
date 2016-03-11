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

/// A TPTP input is either an annotated formula or an include.
#[derive(Debug)]
#[allow(variant_size_differences, missing_docs)]
pub enum TptpInput {
    AnnForm(AnnotatedFormula),
    Incl(Include),
}

/// Wrapper for different kinds of annotated formulae.
#[derive(Debug)]
#[allow(missing_docs)]
pub enum AnnotatedFormula {
    Cnf(CnfAnnotated),
    Fof(FofAnnotated),
}

/// The first part is the name, the second is the formula role, the third is the actual formula.
/// The fourth part is missing because I don't know what to do with it even if it exists.
pub type CnfAnnotated = (String, String, Formula);

/// Similar to the above.
pub type FofAnnotated = (String, String, Formula);

/// The first part is the path.
/// The second optional part is a vector of the names of formulas to take from that file.
pub type Include = (String, Option<Vec<String>>);

/// Data type for FOL terms.
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
#[allow(missing_docs)]
pub enum Term {
    Variable(String),
    Function(String, Vec<Term>),
}

/// Data type for FOL formulas.
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
#[allow(missing_docs)]
pub enum Formula {
    Predicate(String, Vec<Term>),
    Not(Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Implies(Box<Formula>, Box<Formula>),
    Equivalent(Box<Formula>, Box<Formula>),
    Forall(String, Box<Formula>),
    Exists(String, Box<Formula>),
}
