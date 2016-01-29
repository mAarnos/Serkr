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

/// A TPTP input is either an annotated formula or an include.
#[derive(Debug)]
#[allow(variant_size_differences)]
pub enum TptpInput {
    AnnForm(AnnotatedFormula),
    Incl(Include)
}

/// Wrapper for different kinds of annotated formula.
#[derive(Debug)]
pub enum AnnotatedFormula {
    Cnf(CnfAnnotated)
}

/// The first part is the name, the second is the formula role, the third is the actual formula and the fourth part is the annotations.
pub type CnfAnnotated = (String, String, Formula, Option<Annotation>);

/// Annotation.
pub type Annotation = (String, String);

/// The first part is the path, the second optional part is a vector of the names of formulas to take from that file.
pub type Include = (String, Option<Vec<String>>);

/// Data type for CNF terms. 
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
#[allow(missing_docs)]
pub enum Term {
    Variable(String),
    Function(String, Vec<Term>)
}

/// Data type for CNF formulas. 
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
#[allow(missing_docs)]
pub enum Formula {
    Predicate(String, Vec<Term>),
    Not(Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
}
