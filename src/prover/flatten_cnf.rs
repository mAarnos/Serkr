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

/// Turns a formula in CNF into the flat representation more suited for the prover.
/// We assume that the trivial cases of True and False have been handled already.
pub fn flatten_cnf(f: Formula) -> Vec<Clause> {
    let mut lit_map = HashMap::<String, i64>::new();
    let mut fun_map = HashMap::<String, i64>::new();
    let mut var_map = HashMap::<String, i64>::new();
    let mut lit_cnt = 0;
    let mut var_cnt = 0;
    let mut fun_cnt = 0;
    collect(f, &mut lit_map, &mut lit_cnt, &mut var_map, &mut var_cnt, &mut fun_map, &mut fun_cnt)
}

// TODO: clean this crap up.
fn collect(f: Formula, lit_map: &mut HashMap<String, i64>, lit_cnt: &mut i64,
                       var_map: &mut HashMap<String, i64>, var_cnt: &mut i64,
                       fun_map: &mut HashMap<String, i64>, fun_cnt: &mut i64) -> Vec<Clause> {
    match f {
        Formula::Predicate(s, args) => vec!(Clause::new_from_vec(vec!(create_literal(s, args, lit_map, lit_cnt, var_map, var_cnt, fun_map, fun_cnt)))),
        Formula::Not(box Formula::Predicate(ref s, ref args)) => vec!(Clause::new_from_vec(vec!(create_literal(s.clone(), args.clone(), lit_map, lit_cnt, var_map, var_cnt, fun_map, fun_cnt).negate()))),
        Formula::Or(_, _) => vec!(collect_or(f, lit_map, lit_cnt, var_map, var_cnt, fun_map, fun_cnt)),
        Formula::And(box p, box q) => { let mut left = collect(p, lit_map, lit_cnt, var_map, var_cnt, fun_map, fun_cnt); left.append(&mut collect(q, lit_map, lit_cnt, var_map, var_cnt, fun_map, fun_cnt)); left }
        _ => panic!("The CNF transformation failed due to some kind of a bug")
    }
}

// TODO: clean this crap up.
fn collect_or(f: Formula, lit_map: &mut HashMap<String, i64>, lit_cnt: &mut i64,
                          var_map: &mut HashMap<String, i64>, var_cnt: &mut i64,
                          fun_map: &mut HashMap<String, i64>, fun_cnt: &mut i64) -> Clause {
    match f {
        Formula::Predicate(s, args) => Clause::new_from_vec(vec!(create_literal(s, args, lit_map, lit_cnt, var_map, var_cnt, fun_map, fun_cnt))),
        Formula::Not(box Formula::Predicate(ref s, ref args)) => Clause::new_from_vec(vec!(create_literal(s.clone(), args.clone(), lit_map, lit_cnt, var_map, var_cnt, fun_map, fun_cnt).negate())),
        Formula::Or(box p, box q) => { let mut left = collect_or(p, lit_map, lit_cnt, var_map, var_cnt, fun_map, fun_cnt); left.add_literals(collect_or(q, lit_map, lit_cnt, var_map, var_cnt, fun_map, fun_cnt)); left }
        _ => panic!("The CNF transformation failed due to some kind of a bug")
    }
}

fn create_literal(s: String, args: Vec<Term>, lit_map: &mut HashMap<String, i64>, lit_cnt: &mut i64,
                                              var_map: &mut HashMap<String, i64>, var_cnt: &mut i64,
                                              fun_map: &mut HashMap<String, i64>, fun_cnt: &mut i64) -> Literal {
    if let Some(&id) = lit_map.get(&s) {
        Literal::new_from_id_and_args(id, args.into_iter().map(|t| create_term(t, var_map, var_cnt, fun_map, fun_cnt)).collect())
    } else {
        *lit_cnt += 1;
        lit_map.insert(s, *lit_cnt);
        Literal::new_from_id_and_args(*lit_cnt, args.into_iter().map(|t| create_term(t, var_map, var_cnt, fun_map, fun_cnt)).collect())
    }
}

fn create_term(t: Term, var_map: &mut HashMap<String, i64>, var_cnt: &mut i64,
                        fun_map: &mut HashMap<String, i64>, fun_cnt: &mut i64) -> term::Term {
    match t {
         Term::Variable(s) => 
            if let Some(&x) = var_map.get(&s) {
                term::Term::new(x, Vec::new())
            } else {
                *var_cnt -= 1;
                var_map.insert(s, *var_cnt);
                term::Term::new(*var_cnt, Vec::new())
            },
        Term::Function(s, args) =>             
            if let Some(&x) = fun_map.get(&s) {
                term::Term::new(x, args.into_iter().map(|t2| create_term(t2, var_map, var_cnt, fun_map, fun_cnt)).collect())
            } else {
                *fun_cnt += 1;
                fun_map.insert(s, *fun_cnt);
                term::Term::new(*fun_cnt, args.into_iter().map(|t2| create_term(t2, var_map, var_cnt, fun_map, fun_cnt)).collect())
            },
    }
}

#[cfg(test)]
mod test {
    
} 

