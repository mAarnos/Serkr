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

use utils::formula::Formula;
use prover::unification::negate;

/// Picks and removes the "best" clause from the unused clauses according to heuristics.
/// Currently just picks the shortest one.
fn pick_clause(unused: &mut Vec<Vec<Formula>>) -> Vec<Formula> {
    let mut best_clause_index = 0;
    let mut best_clause_size = unused[0].len();
    
    for i in 1..unused.len() {
        if unused[i].len() < best_clause_size {
            best_clause_index = i;
            best_clause_size = unused[i].len();
        }
    }
    
    unused.swap_remove(best_clause_index)
}

/// Checks if a clause is trivial, i.e. it is a syntactical tautology.
fn trivial(cl: &Vec<Formula>) -> bool {
    for i in 0..cl.len() {
        let counterpart = negate(cl[i].clone());
        for j in (i + 1)..cl.len() {
            if cl[j] == counterpart {
                return true;
            }
        }
    }  
    false
}

fn resolution_loop(mut used: Vec<Vec<Formula>>, mut unused: Vec<Vec<Formula>>) -> Result<bool, &'static str> {
    while unused.len() > 0 {
        let chosen_clause = pick_clause(&mut unused);
    }
    Err("No proof found.")
}