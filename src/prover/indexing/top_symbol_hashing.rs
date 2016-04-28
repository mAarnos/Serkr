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

use std::collections::HashMap;
use std::slice::Iter;
use prover::data_structures::clause::Clause;
use prover::data_structures::term::Term;
use prover::ordering::term_ordering::TermOrdering;

/// Top symbol hashing is almost certainly the simplest indexing technique.
/// The idea is that if t = f(t0, ..., tn), n >= 0, 
/// then any s that matches to t has to start with f.
/// We partition all rewrite rules so that two rules are in the same partition
/// if they start with the same symbol. 
/// For a non-variable query term we then only need to test all rewrite rules in the partition.
pub struct TopSymbolHashIndex {
    index: HashMap<i64, Vec<(Term, Term)>>
}

impl TopSymbolHashIndex {
    /// Creates an empty index.
    pub fn new() -> TopSymbolHashIndex {
        TopSymbolHashIndex { index: HashMap::new() }
    }
    
    /// Adds a rewrite rule to the index, if possible.
    fn add_rewrite_rule_to_index(&mut self, s: &Term, t: &Term) {
        if s.is_function() {
            // And once again we run into the if-let borrow bug.
            if let Some(vec) = self.index.get_mut(&s.get_id()) {
                vec.push((s.clone(), t.clone()));
                return;
            }
            self.index.insert(s.get_id(), vec![(s.clone(), t.clone())]); 
        }    
    }
    
    /// Removes a rewrite rule from the index, if possible.
    fn remove_rewrite_rule_from_index(&mut self, s: &Term, t: &Term) {
        if s.is_function() {
            if let Some(vec) = self.index.get_mut(&s.get_id()) {
                for i in 0..vec.len() {
                    if *s == vec[i].0 && *t == vec[i].1 {
                        vec.swap_remove(i);
                        break;
                    }
                }
            }
        }    
    }
    
    /// Adds a clause to the index, if possible.
    pub fn add_clause_to_index(&mut self, term_ordering: &TermOrdering, cl: &Clause) {
        if cl.is_positive_unit() {
            let l = cl[0].get_lhs();
            let r = cl[0].get_rhs();
            // Check if the equation is orientable.
            // If it is we can only add one rewrite rule to the index instead of two.
            if term_ordering.gt(l, r) {
                self.add_rewrite_rule_to_index(l, r);
            } else if term_ordering.gt(r, l) {
                self.add_rewrite_rule_to_index(r, l);
            } else {
                self.add_rewrite_rule_to_index(l, r);
                self.add_rewrite_rule_to_index(r, l);
            }
        }
    }
    
    // Removes a clause from the index, if possible.
    pub fn remove_clause_from_index(&mut self, cl: &Clause) {
        if cl.is_positive_unit() {
            let l = cl[0].get_lhs();
            let r = cl[0].get_rhs();
            self.remove_rewrite_rule_from_index(l, r);
            self.remove_rewrite_rule_from_index(r, l);
        }
    }
    
    /// Returns an iterator over the possible matches to a term t.
    /// We have to wrap the iterator in an Option due to some problems with the type system.
    pub fn possible_matches(&self, t: &Term) -> Option<Iter<(Term, Term)>> {
        if t.is_variable() {
            None
        } else {
            if let Some(vec) = self.index.get(&t.get_id()) {
                Some(vec.iter())
            } else {
                None
            }
        }
    }
}

impl Default for TopSymbolHashIndex {
    fn default() -> TopSymbolHashIndex {
        TopSymbolHashIndex::new()
    }
}

#[cfg(test)]
mod test {}