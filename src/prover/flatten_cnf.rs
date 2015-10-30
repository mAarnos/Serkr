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

use std::collections::HashMap;
use prover::term;
use prover::clause::Clause;
use prover::literal::Literal;
use utils::formula::{Term, Formula};

pub struct RenamingInfo {
    pub lit_map: HashMap<(String, usize), i64>,
    pub fun_map: HashMap<(String, usize), i64>,
    pub var_map: HashMap<String, i64>,
    pub lit_cnt: i64,
    pub fun_cnt: i64,
    pub var_cnt: i64,
}

impl RenamingInfo {
    pub fn new() -> RenamingInfo {
        RenamingInfo { lit_map: HashMap::new(), var_map: HashMap::new(), fun_map: HashMap::new(),
                       lit_cnt: 0, var_cnt: 0, fun_cnt: 0 }
    }
}

/// Turns a formula in CNF into the flat representation more suited for the prover.
/// We assume that the trivial cases of True and False have been handled already.
pub fn flatten_cnf(f: Formula) -> Vec<Clause> {
    let mut renaming_info = RenamingInfo::new();
    collect(f, &mut renaming_info)
}

// TODO: clean this crap up.
fn collect(f: Formula, renaming_info: &mut RenamingInfo) -> Vec<Clause> {
    match f {
        Formula::Predicate(s, args) => vec!(Clause::new_from_vec(vec!(create_literal(s, args, renaming_info)))),
        Formula::Not(box Formula::Predicate(ref s, ref args)) => vec!(Clause::new_from_vec(vec!(create_literal(s.clone(), args.clone(), renaming_info).negate()))),
        Formula::Or(_, _) => vec!(collect_or(f, renaming_info)),
        Formula::And(box p, box q) => { let mut left = collect(p, renaming_info); left.append(&mut collect(q, renaming_info)); left }
        _ => panic!("The CNF transformation failed due to some kind of a bug")
    }
}

// TODO: clean this crap up.
fn collect_or(f: Formula, renaming_info: &mut RenamingInfo) -> Clause {
    match f {
        Formula::Predicate(s, args) => Clause::new_from_vec(vec!(create_literal(s, args, renaming_info))),
        Formula::Not(box Formula::Predicate(ref s, ref args)) => Clause::new_from_vec(vec!(create_literal(s.clone(), args.clone(), renaming_info).negate())),
        Formula::Or(box p, box q) => { let mut left = collect_or(p, renaming_info); left.add_literals(collect_or(q, renaming_info)); left }
        _ => panic!("The CNF transformation failed due to some kind of a bug")
    }
}

fn create_literal(s: String, args: Vec<Term>, ri: &mut RenamingInfo) -> Literal {
    if let Some(&id) = ri.lit_map.get(&(s.clone(), args.len())) {
        Literal::new_from_id_and_args(id, args.into_iter().map(|t| create_term(t, ri)).collect())
    } else {
        ri.lit_cnt += 1;
        ri.lit_map.insert((s, args.len()), ri.lit_cnt);
        Literal::new_from_id_and_args(ri.lit_cnt, args.into_iter().map(|t| create_term(t, ri)).collect())
    }
}

fn create_term(t: Term, ri: &mut RenamingInfo) -> term::Term {
    match t {
         Term::Variable(s) => 
            if let Some(&x) = ri.var_map.get(&s) {
                term::Term::new(x, Vec::new())
            } else {
                ri.var_cnt -= 1;
                ri.var_map.insert(s, ri.var_cnt);
                term::Term::new(ri.var_cnt, Vec::new())
            },
        Term::Function(s, args) =>             
            if let Some(&x) = ri.fun_map.get(&(s.clone(), args.len())) {
                term::Term::new(x, args.into_iter().map(|t2| create_term(t2, ri)).collect())
            } else {
                ri.fun_cnt += 1;
                ri.fun_map.insert((s, args.len()), ri.fun_cnt);
                term::Term::new(ri.fun_cnt, args.into_iter().map(|t2| create_term(t2, ri)).collect())
            },
    }
}

#[cfg(test)]
mod test {
    
} 

