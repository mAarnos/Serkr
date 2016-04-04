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

use cnf::ast::Formula;
use cnf::renaming_info::RenamingInfo;
use cnf::nnf::nnf;
use cnf::standard_skolemization::skolemize;
use cnf::distribute_ors_over_ands::distribute_ors_over_ands;

/// Turns a formula into CNF.
pub fn cnf(f: Formula, renaming_info: &mut RenamingInfo) -> Formula {
    if f == Formula::True || f == Formula::False || is_in_cnf(&f) {
        f
    } else {
        let nnf_f = nnf(f, renaming_info);
        let skolemized_f = skolemize(nnf_f, renaming_info);
        let cnf_f = distribute_ors_over_ands(skolemized_f);
        assert!(cnf_f == Formula::True || cnf_f == Formula::False || is_in_cnf(&cnf_f));
        cnf_f
    }
}

/// Tests whether a formula is in CNF.
fn is_in_cnf(f: &Formula) -> bool {
    match *f {
        Formula::Or(ref l) => l.iter().all(is_disjunction),
        Formula::And(ref l) => l.iter().all(is_in_cnf),
        _ => is_literal(f),
    }
}

/// Checks whether a formula is disjunction of literals.
fn is_disjunction(f: &Formula) -> bool {
    match *f {
        Formula::Or(ref l) => l.iter().all(is_disjunction),
        _ => is_literal(f),
    }
}

/// Checks whether a formula is a literal.
fn is_literal(f: &Formula) -> bool {
    match *f {
        Formula::Predicate(_, _) => true,
        Formula::Not(ref p) => is_atomic(&p),
        _ => false,
    }
}

/// Checks whether a formula is a literal.
fn is_atomic(f: &Formula) -> bool {
    match *f {
        Formula::Predicate(_, _) => true,
        _ => false,
    }
}

#[cfg(test)]
mod test {}
