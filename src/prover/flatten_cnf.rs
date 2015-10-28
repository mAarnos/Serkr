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

use prover::clause::Clause;
use prover::literal::Literal;
use utils::formula::{Term, Formula};
use std::collections::HashMap;

/// Turns a formula in CNF into the flat representation more suited for the prover.
pub fn flatten_cnf_formula(_: Formula) -> Vec<Clause> {
    Vec::new()
}

/*
fn collect(f: Formula) -> Vec<Clause> {
    match f {
        Formula::Predicate(_, _) |
        Formula::Not(box Formula::Predicate(_, _)) => vec!(vec!(f)),
        Formula::Or(_, _) => vec!(collect_or(f)),
        Formula::And(box p, box q) => { let mut left = collect(p); left.append(&mut collect(q)); left }
        _ => panic!("The CNF transformation failed due to some kind of a bug")
    }
}
*/

/*
fn collect_or(f: Formula, mapping: &mut HashMap<String, i64>, n: &mut i64) -> Clause {
    match f {
        Formula::Predicate(s, args) => Clause::new_from_vec(vec!(f)),
        Formula::Not(box Formula::Predicate(s, args)) => Clause::new_from_vec(vec!(f)),
        Formula::Or(box p, box q) => { let mut left = collect_or(p); left.add_literals(collect_or(q)); left }
        _ => panic!("The CNF transformation failed due to some kind of a bug")
    }
}
*/

fn create_literal(s: String, args: Vec<Term>, mapping: &mut HashMap<String, i64>, n: &mut i64) -> Literal {
    if let Some(&id) = mapping.get(&s) {
        Literal::new_from_id_and_args(id, args)
    } else {
        let ret = Literal::new_from_id_and_args(*n, args);
        mapping.insert(s, *n);
        *n += 1;
        ret
    }
}

#[cfg(test)]
mod test {
    
} 

