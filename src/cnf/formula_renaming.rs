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

use num::{BigUint, Zero, One};
use cnf::ast::Formula;

/// Tries to estimate how many clauses would be generated if this formula were turned into CNF.
fn estimate_size(f: &Formula) -> BigUint {
    estimate_size_pos(f)
}

fn estimate_size_pos(f: &Formula) -> BigUint {
    match *f {
        Formula::Predicate(_, _) => One::one(),
        Formula::Not(ref p) => estimate_size_neg(&p),
        Formula::And(ref l) => l.iter().fold(Zero::zero(), |acc, x| acc + estimate_size_pos(x)),
        Formula::Or(ref l) => l.iter().fold(One::one(), |acc, x| acc * estimate_size_pos(x)),
        Formula::Implies(ref p, ref q) => estimate_size_neg(&p) * estimate_size_pos(&q),
        Formula::Equivalent(ref p, ref q) => {
            estimate_size_pos(&p) * estimate_size_neg(&q) +
            estimate_size_neg(&p) * estimate_size_pos(&q)
        }
        Formula::Forall(_, ref p) | Formula::Exists(_, ref p) => estimate_size_pos(&p),
        _ => panic!("True and false should've been eliminated already"),
    }
}

fn estimate_size_neg(f: &Formula) -> BigUint {
    match *f {
        Formula::Predicate(_, _) => One::one(),
        Formula::Not(ref p) => estimate_size_pos(&p),
        Formula::And(ref l) => l.iter().fold(One::one(), |acc, x| acc * estimate_size_neg(x)),
        Formula::Or(ref l) => l.iter().fold(Zero::zero(), |acc, x| acc + estimate_size_neg(x)),
        Formula::Implies(ref p, ref q) => estimate_size_pos(&p) + estimate_size_neg(&q),
        Formula::Equivalent(ref p, ref q) => {
            estimate_size_pos(&p) * estimate_size_pos(&q) +
            estimate_size_neg(&p) * estimate_size_neg(&q)
        }
        Formula::Forall(_, ref p) | Formula::Exists(_, ref p) => estimate_size_neg(&p),
        _ => panic!("True and false should've been eliminated already"),
    }
}

#[cfg(test)]
mod test {}
