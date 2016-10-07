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
use std::slice::Iter as VIter;
use std::collections::hash_map::Iter as MIter;
use prover::data_structures::clause::Clause;
use prover::data_structures::term::Term;
use prover::unification::substitution::Substitution;
use prover::unification::matching::term_match_with_subst;

/// Used for traveling the positions of a term in prefix order lazily.
#[derive(Clone)]
struct PrefixOrderIterator<'a> {
    term: &'a Term,
    peek_term: Option<&'a Term>,
    started: bool,
    stack: Vec<VIter<'a, Term>>
}

impl<'a> PrefixOrderIterator<'a> {
    fn new(t: &'a Term) -> PrefixOrderIterator<'a> {
        PrefixOrderIterator {
            term: t,
            peek_term: None,
            started: false,
            stack: Vec::new()
        }
    }
    
    /// Implementing peek by hand like this is a lot easier than using Peekable.
    fn peek(&mut self) -> Option<&'a Term> {
        if !self.peek_term.is_some() {
            self.peek_term = self.next();
        } 
        self.peek_term
    }
    
    /// Skips the current subtree.
    fn skip_subtree(&mut self) {
        self.peek_term = None;
        self.stack.pop();
    }
}

impl<'a> Iterator for PrefixOrderIterator<'a> {
    type Item = &'a Term;
    
    fn next(&mut self) -> Option<&'a Term> {
        // If we have peeked this is all we have to do.
        if self.peek_term.is_some() {
            let temp = self.peek_term;
            self.peek_term = None;
            return temp;
        }
        
        if !self.started {
            self.started = true;
            if self.term.get_arity() > 0 {
                self.stack.push(self.term.iter());
            } 
            Some(self.term)
        } else if self.stack.is_empty() {
            None
        } else {
            for i in (0..self.stack.len()).rev() {
                if let Some(t) = self.stack[i].next() {
                    if t.get_arity() > 0 {
                        self.stack.push(t.iter());
                    } 
                    return Some(t);
                } 
                self.stack.pop();
            }
            assert!(self.stack.is_empty());
            None
        }
    }
}

fn normalize_variables(l: &mut Term, r: &mut Term) {
    let mut m = HashMap::new();
    let mut x = 0;
    l.rename_no_common(&mut m, &mut x);
    r.rename_no_common(&mut m, &mut x);
}

#[derive(Debug, Clone)]
enum Trie {
    Leaf(Term, Term, bool),
    Node(HashMap<i64, Trie>)
}

impl Trie {
    /// Creates a new empty trie.
    pub fn new() -> Trie {
        Trie::Node(HashMap::new())
    }
}

/// A perfect discrimination tree is used for fast retrieval of 
/// generalizations (an equation where one side matches the query term) 
/// and specializations (an equation where the query term matches one side)
/// of equations. This is critical for efficient use of many inference and simplification rules.
#[derive(Debug, Clone)]
pub struct PDTree {
    data: Trie
}

pub struct GeneralizationIterator<'a> {
    sign: bool,
    stack: Vec<(Substitution, 
                PrefixOrderIterator<'a>, 
                MIter<'a, i64, Trie>)>
}

impl<'a> GeneralizationIterator<'a> {
    fn new(trie: &'a Trie, t: &'a Term, sign: bool) -> GeneralizationIterator<'a> {
        if let Trie::Node(ref m) = *trie {
            GeneralizationIterator {
                sign: sign, 
                stack: vec!((Substitution::new(), 
                             PrefixOrderIterator::new(t), 
                             m.iter()))
            }
        } else {
            panic!("Trie root should always be a node!");
        }
    }
}

impl<'a> Iterator for GeneralizationIterator<'a> {
    type Item = (&'a Term, &'a Term, Substitution);
    
    fn next(&mut self) -> Option<(&'a Term, &'a Term, Substitution)> {
        while let Some((subst, mut t_iter, mut trie_iter)) = self.stack.pop() {
            // Check if there are symbols left. If there aren't, backtrack by doing nothing (see the loop).
            if let Some(t) = t_iter.peek() {
                let id = t.get_id();
                // Analyze the current level.
                while let Some((&k, subtrie)) = trie_iter.next() {
                    // If the function symbols are the same we can skip it.
                    if k == id && k >= 0 {
                        let mut new_t_iter = t_iter.clone();
                        new_t_iter.next(); // need to advance one for the next level
                        match subtrie {
                            &Trie::Leaf(ref l, ref r, sign) => if new_t_iter.peek().is_none() && sign == self.sign {
                                let subst2 = subst.clone();
                                self.stack.push((subst, t_iter, trie_iter));
                                return Some((l, r, subst2));
                            },
                            &Trie::Node(ref m) => if new_t_iter.peek().is_some() {
                                let new_subst = subst.clone();
                                // Push the thing we popped out back in.
                                // All this is done to avoid borrow checker trouble.
                                self.stack.push((subst, t_iter, trie_iter));
                                self.stack.push((new_subst, new_t_iter, m.iter()));
                                break;
                            } 
                        } 
                    } else if k < 0 {
                        // Otherwise check if we can bind and continue.
                        if let Some(new_subst) = term_match_with_subst(subst.clone(), &Term::new_variable(k), t) {
                            let mut new_t_iter = t_iter.clone();
                            new_t_iter.next(); // again, need to advance once
                            new_t_iter.skip_subtree(); // since we matched we can skip a subtree
                            match subtrie {
                                &Trie::Leaf(ref l, ref r, sign) => if new_t_iter.peek().is_none() && sign == self.sign {
                                    self.stack.push((subst, t_iter, trie_iter));
                                    return Some((l, r, new_subst));
                                },
                                &Trie::Node(ref m) => if new_t_iter.peek().is_some() {
                                    self.stack.push((subst, t_iter, trie_iter));
                                    self.stack.push((new_subst, new_t_iter, m.iter()));
                                    break;
                                } 
                            } 
                        }
                    }   
                }
            }
        }
        
        None
    }
}

impl PDTree {
    /// Creates an empty perfect discrimination tree.
    pub fn new() -> PDTree {
        PDTree { data: Trie::new() }
    }
    
    /// Adds a clause to the index.
    /// Does nothing if the clause is not unit.
    pub fn add_clause_to_index(&mut self, cl: &Clause) {
        if cl.is_unit() {
            let pos = cl[0].is_positive();
            if !cl[0].get_lhs().is_truth() {
                let mut l = cl[0].get_lhs().clone();
                let mut r = cl[0].get_rhs().clone();
                normalize_variables(&mut l, &mut r);
                insert_at_leaf(&mut self.data, PrefixOrderIterator::new(&l), &l, r, pos);
            }
            if !cl[0].get_rhs().is_truth() {
                let mut l = cl[0].get_lhs().clone();
                let mut r = cl[0].get_rhs().clone();
                normalize_variables(&mut r, &mut l);
                insert_at_leaf(&mut self.data, PrefixOrderIterator::new(&r), &r, l, pos);
            }
        }
    }
    
    /// Finds generalizations of a given term with the given sign.
    pub fn iter_generalizations<'a>(&'a self, t: &'a Term, sign: bool) -> GeneralizationIterator<'a> {
        GeneralizationIterator::new(&self.data, t, sign)
    }
}

/// Inserts l = r (or l <> r) into the trie, constructing the path if it doesn't exist.
fn insert_at_leaf(trie: &mut Trie, mut iter: PrefixOrderIterator, l: &Term, r: Term, sign: bool) {
    if let Trie::Node(ref mut m) = *trie {
        if let Some(t) = iter.next() {
            let id = t.get_id();
            if let Some(subtrie) = m.get_mut(&id) {
                insert_at_leaf(subtrie, iter, l, r, sign);
                return;
            } 
            
            // And once again the if-let borrow bug.
            if iter.peek().is_some() {
                let mut subtrie = Trie::new();
                insert_at_leaf(&mut subtrie, iter, l, r, sign);
                m.insert(id, subtrie);
            } else {
                m.insert(id, Trie::Leaf(l.clone(), r, sign));
            }
        } 
    } else {
        // Path to leaf already exists if we get here.
        // Doesn't happen that often but must be taken care of.
    }
}

#[cfg(test)]
mod test {
    use super::PrefixOrderIterator;
    use super::PDTree;
    use prover::data_structures::clause::Clause;
    use prover::data_structures::literal::Literal;
    use prover::data_structures::term::Term;
    
    #[test]
    fn prefix_order_iterator_1() {
        let a = Term::new_function(1, Vec::new());
        let b = Term::new_function(2, Vec::new());
        let g_b = Term::new_function(3, vec!(b));
        let f_a_g_b = Term::new_function(4, vec!(a, g_b));
        let t = Term::new_function(3, vec!(f_a_g_b));
        let iter = PrefixOrderIterator::new(&t);
        let flat = iter.map(|t| t.get_id()).collect::<Vec<_>>();
        
        assert_eq!(flat.len(), 5);
        assert_eq!(flat[0], 3);
        assert_eq!(flat[1], 4);
        assert_eq!(flat[2], 1);
        assert_eq!(flat[3], 3);
        assert_eq!(flat[4], 2);
    }
    
    #[test]
    fn prefix_order_iterator_2() {
        let x = Term::new_variable(-1);
        let a = Term::new_function(1, Vec::new());
        let g_a = Term::new_function(2, vec!(a));
        let t = Term::new_function(3, vec!(g_a, x));
        let iter = PrefixOrderIterator::new(&t);
        let flat = iter.map(|t| t.get_id()).collect::<Vec<_>>();
        
        assert_eq!(flat.len(), 4);
        assert_eq!(flat[0], 3);
        assert_eq!(flat[1], 2);
        assert_eq!(flat[2], 1);
        assert_eq!(flat[3], -1);
    }
    
    #[test]
    fn prefix_order_iterator_3() {
        let c = Term::new_function(1, Vec::new());
        let iter = PrefixOrderIterator::new(&c);
        let flat = iter.map(|t| t.get_id()).collect::<Vec<_>>();
        
        assert_eq!(flat.len(), 1);
        assert_eq!(flat[0], 1);
    }
    
    #[test]
    fn prefix_order_iterator_4() {
        let x = Term::new_variable(-1);
        let c = Term::new_function(1, vec!(x));
        let mut iter = PrefixOrderIterator::new(&c);
        
        assert_eq!(iter.peek().unwrap().get_id(), 1);
        assert_eq!(iter.next().unwrap().get_id(), 1);
        assert_eq!(iter.next().unwrap().get_id(), -1);
        assert_eq!(iter.peek(), None);
    }
    
    #[test]
    fn pd_tree_1() {
        let c = Term::new_function(1, Vec::new());
        let t = Term::new_truth();
        let lit = Literal::new(false, c.clone(), t.clone());
        let cl = Clause::new(vec!(lit));
        
        let mut pd_tree = PDTree::new();
        pd_tree.add_clause_to_index(&cl);
        let matches = pd_tree.iter_generalizations(&c, true).collect::<Vec<_>>();
        
        assert_eq!(matches.len(), 1);
        assert_eq!(*matches[0].0, c);
        assert_eq!(*matches[0].1, t);
        assert_eq!(matches[0].2.size(), 0);
    }
}