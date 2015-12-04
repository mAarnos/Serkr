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
use parser::formula::{Term, Formula};

/// Mappings from literals, terms and variables to ids.
pub struct RenamingInfo {
    pub fun_map: HashMap<(String, usize), i64>,
    pub var_map: HashMap<String, i64>,
    pub fun_cnt: i64,
    pub var_cnt: i64,
}

impl RenamingInfo {
    pub fn new() -> RenamingInfo {
        RenamingInfo { var_map: HashMap::new(), fun_map: HashMap::new(),
                       var_cnt: 0, fun_cnt: 0 }
    }
}

/// Turns a formula in CNF into a flat representation more suited for the prover.
/// Also converts the formula into pure equational logic.
/// We assume that the trivial cases of True and False have been handled already.
pub fn flatten_cnf(f: Formula) -> (Vec<Clause>, RenamingInfo) {
    let mut renaming_info = RenamingInfo::new();
    let flattened_f = collect(f, &mut renaming_info);
    (flattened_f, renaming_info)
}

// TODO: clean this crap up.
fn collect(f: Formula, ri: &mut RenamingInfo) -> Vec<Clause> {
    match f {
        Formula::Predicate(s, args) => vec!(Clause::new(vec!(create_literal(false, s, args, ri)))),
        Formula::Not(p) => match *p {
                               Formula::Predicate(ref s, ref args) =>  vec!(Clause::new(vec!(create_literal(true, s.clone(), args.clone(), ri)))),
                               _ => panic!("The CNF transformation failed due to some kind of a bug")
                           },
        Formula::Or(_, _) => vec!(collect_or(f, ri)),
        Formula::And(p, q) => { let mut left = collect(*p, ri); left.append(&mut collect(*q, ri)); left }
        _ => panic!("The CNF transformation failed due to some kind of a bug")
    }
}

// TODO: clean this crap up.
fn collect_or(f: Formula, ri: &mut RenamingInfo) -> Clause {
    match f {
        Formula::Predicate(s, args) => Clause::new(vec!(create_literal(false, s, args, ri))),
        Formula::Not(p) => match *p {
                               Formula::Predicate(ref s, ref args) =>  Clause::new(vec!(create_literal(true, s.clone(), args.clone(), ri))),
                               _ => panic!("The CNF transformation failed due to some kind of a bug")
                           },
        Formula::Or(p, q) => { let mut left = collect_or(*p, ri); left.add_literals(collect_or(*q, ri)); left }
        _ => panic!("The CNF transformation failed due to some kind of a bug")
    }
}

/// We convert P(x_1, ..., x_n) to f_P(x_1, ..., x_n) = T() where T() is a special function symbol representing truth.  
fn create_literal(negated: bool, s: String, args: Vec<Term>, ri: &mut RenamingInfo) -> Literal {
    if s == "=" {
        assert_eq!(args.len(), 2);
        Literal::new(negated, create_term(args[0].clone(), ri), create_term(args[1].clone(), ri))
    } else {
        assert!(s.chars().next().unwrap().is_uppercase());
        Literal::new(negated, create_term(Term::Function(s, args), ri), term::Term::new_truth())
    }
}

fn create_term(t: Term, ri: &mut RenamingInfo) -> term::Term {
    match t {
         Term::Variable(s) => 
            if let Some(&x) = ri.var_map.get(&s) {
                term::Term::new(x, false, Vec::new())
            } else {
                ri.var_cnt -= 1;
                ri.var_map.insert(s, ri.var_cnt);
                term::Term::new(ri.var_cnt, false, Vec::new())
            },
        Term::Function(s, args) => {
                let sort_pred = s.chars().next().unwrap().is_uppercase();
                if let Some(&x) = ri.fun_map.get(&(s.clone(), args.len())) {
                    term::Term::new(x, sort_pred, args.into_iter().map(|t2| create_term(t2, ri)).collect())
                } else {
                    ri.fun_cnt += 1;
                    ri.fun_map.insert((s, args.len()), ri.fun_cnt);
                    term::Term::new(ri.fun_cnt, sort_pred, args.into_iter().map(|t2| create_term(t2, ri)).collect())
                }
            },
    }
}

#[cfg(test)]
mod test {
    
} 

