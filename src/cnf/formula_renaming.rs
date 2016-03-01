/*
    Serkr - An automated theorem prover. Copyright (C) 2015-2016 Mikko Aarnos.

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

use num::{BigUint, Zero, One};
use cnf::ast::Formula;

pub fn estimate_size_positive(f: &Formula) -> BigUint {
    match *f {
        Formula::Predicate(_, _) => One::one(),
        Formula::Not(ref p) => estimate_size_negative(&p),
        Formula::And(ref l) => l.iter().fold(Zero::zero(), |acc, x| acc + estimate_size_positive(x)),
        Formula::Or(ref l) => l.iter().fold(One::one(), |acc, x| acc * estimate_size_positive(x)),
        Formula::Implies(ref p, ref q) => estimate_size_negative(&p) * estimate_size_positive(&q),
        Formula::Equivalent(ref p, ref q) => estimate_size_positive(&p) * estimate_size_negative(&q)
                                           + estimate_size_negative(&p) * estimate_size_positive(&q),
        Formula::Forall(_, ref p) | Formula::Exists(_, ref p) => estimate_size_positive(&p),
        _ => panic!("True and false should've been eliminated already")
    }
}

fn estimate_size_negative(f: &Formula) -> BigUint {
    match *f {
        Formula::Predicate(_, _) => One::one(),
        Formula::Not(ref p) => estimate_size_positive(&p),
        Formula::And(ref l) => l.iter().fold(One::one(), |acc, x| acc * estimate_size_negative(x)),
        Formula::Or(ref l) => l.iter().fold(Zero::zero(), |acc, x| acc + estimate_size_negative(x)),
        Formula::Implies(ref p, ref q) => estimate_size_positive(&p) + estimate_size_negative(&q),
        Formula::Equivalent(ref p, ref q) => estimate_size_positive(&p) * estimate_size_positive(&q)
                                           + estimate_size_negative(&p) * estimate_size_negative(&q),
        Formula::Forall(_, ref p) | Formula::Exists(_, ref p) => estimate_size_negative(&p),
        _ => panic!("True and false should've been eliminated already")
    }
}

#[cfg(test)]
mod test {

}    
