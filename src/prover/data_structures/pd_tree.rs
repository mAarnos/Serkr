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

// This file is too complicated, figure out a simper way.
// To be precise, a lazy recursive iterator is very painful to write.

use std::collections::HashMap;
use std::slice::Iter as VIter;
use std::collections::hash_map::Iter as MIter;
use prover::data_structures::clause::Clause;
use prover::data_structures::term::Term;
use prover::unification::substitution::Substitution;
use prover::unification::matching::term_match_with_subst;
use utils::either::Either;

/// Used for traveling the positions of a term in prefix order lazily.
#[derive(Clone, Debug)]
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
    
    fn next(&mut self) -> Option<Self::Item> {
        // If we have peeked this is all we have to do.
        if self.peek_term.is_some() {
            let temp = self.peek_term;
            self.peek_term = None;
            return temp;
        }
        
        if !self.started {
            self.started = true;
            self.stack.push(self.term.iter());
            Some(self.term)
        } else if self.stack.is_empty() {
            None
        } else {
            for i in (0..self.stack.len()).rev() {
                if let Some(t) = self.stack[i].next() {
                    self.stack.push(t.iter());
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

/// A perfect discrimination tree is used for fast retrieval of 
/// generalizations (an equation where one side matches the query term) 
/// and specializations (an equation where the query term matches one side)
/// of equations. This is critical for efficient use of many inference and simplification rules.
#[derive(Debug, Clone)]
pub enum PDTree {
    Leaf(Vec<(Term, Term, bool)>),
    Node(HashMap<i64, PDTree>)
}

impl PDTree {
    /// Creates an empty perfect discrimination tree.
    pub fn new() -> PDTree {
        PDTree::Node(HashMap::new())
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
                self.insert_at_leaf(PrefixOrderIterator::new(&l), &l, r, pos);
            }
            if !cl[0].get_rhs().is_truth() {
                let mut l = cl[0].get_lhs().clone();
                let mut r = cl[0].get_rhs().clone();
                normalize_variables(&mut r, &mut l);
                self.insert_at_leaf(PrefixOrderIterator::new(&r), &r, l, pos);
            }
        }
    }
    
    /// Inserts l = r (or l <> r) into the tree, constructing the path if it doesn't exist.
    fn insert_at_leaf(&mut self, mut iter: PrefixOrderIterator, l: &Term, r: Term, sign: bool) {
        match *self {
            PDTree::Node(ref mut m) => if let Some(t) = iter.next() {
                let id = t.get_id();
                if let Some(subtree) = m.get_mut(&id) {
                    subtree.insert_at_leaf(iter, l, r, sign);
                    return;
                } 
                
                // And once again the if-let borrow bug.
                if iter.peek().is_some() {
                    let mut subtree = PDTree::new();
                    subtree.insert_at_leaf(iter, l, r, sign);
                    m.insert(id, subtree);
                } else {
                    m.insert(id, PDTree::Leaf(vec!((l.clone(), r, sign))));
                }
            },
            PDTree::Leaf(ref mut v) => v.push((l.clone(), r, sign)),
        } 
    }

    /// Finds generalizations of a given term with the given sign.
    pub fn iter_generalizations<'a>(&'a self, t: &'a Term, sign: bool) -> GeneralizationIterator<'a> {
        GeneralizationIterator::new(self, t, sign)
    }
}

pub struct GeneralizationIterator<'a> {
    sign: bool,
    stack: Vec<StackFrame<'a>>,
}

struct StackFrame<'a> {
    subst: Substitution,
    iter: PrefixOrderIterator<'a>, 
    tree_iter: Either<MIter<'a, i64, PDTree>, VIter<'a, (Term, Term, bool)>>
}

impl<'a> GeneralizationIterator<'a> {
    fn new(pd_tree: &'a PDTree, t: &'a Term, sign: bool) -> GeneralizationIterator<'a> {
        if let PDTree::Node(ref m) = *pd_tree {
            GeneralizationIterator {
                sign: sign, 
                stack: vec!(StackFrame {
                    subst: Substitution::new(),
                    iter: PrefixOrderIterator::new(t),
                    tree_iter: Either::Left(m.iter())
                }),
            }
        } else {
            panic!("PDTree root should always be a node!");
        }
    }  
}

impl<'a> Iterator for GeneralizationIterator<'a> {
    type Item = (&'a Term, &'a Term, Substitution);
    
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(StackFrame { subst, mut iter, tree_iter }) = self.stack.pop() {
            match tree_iter {
                Either::Left(mut subtree_iter) => if let Some(t) = iter.peek() {
                    let id = t.get_id();
                    // Cannot use a for-loop as that moves the iterator.
                    while let Some((&k, subtree)) = subtree_iter.next() {
                        // If the function symbols are the same we can skip it.
                        if k == id && k >= 0 {
                            let mut new_iter = iter.clone();
                            new_iter.next(); // Need to advance one for the next level.
                            let new_tree_iter = match *subtree {
                                PDTree::Leaf(ref l) => Either::Right(l.iter()),
                                PDTree::Node(ref m) => Either::Left(m.iter()),
                            };
                            // Remember to push the stack frame back as we are not done with this node.
                            self.stack.push(StackFrame {
                                subst: subst.clone(),
                                iter: iter,
                                tree_iter: Either::Left(subtree_iter),
                            });
                            // Now add the stack frame for the recursive call.
                            self.stack.push(StackFrame {
                                subst: subst,
                                iter: new_iter,
                                tree_iter: new_tree_iter,
                            });
                            break;
                        } else if k < 0 {
                            // Otherwise check if we can bind and continue.
                            if let Some(new_subst) = term_match_with_subst(subst.clone(), &Term::new_variable(k), t) {
                                let mut new_iter = iter.clone();
                                new_iter.next(); // Again, need to advance once
                                new_iter.skip_subtree(); // Since we matched we can skip a subtree.
                                let new_tree_iter = match *subtree {
                                    PDTree::Leaf(ref l) => Either::Right(l.iter()),
                                    PDTree::Node(ref m) => Either::Left(m.iter()),
                                };
                                // Remember to push the stack frame back as we are not done with this node.
                                self.stack.push(StackFrame {
                                    subst: subst,
                                    iter: iter,
                                    tree_iter: Either::Left(subtree_iter),
                                });
                                // Now add the stack frame for the recursive call.
                                self.stack.push(StackFrame {
                                    subst: new_subst,
                                    iter: new_iter,
                                    tree_iter: new_tree_iter,
                                });
                                break;
                            }
                        }
                    }
                },
                Either::Right(mut leaf_iter) => if iter.next().is_none() {
                    // Cannot use a for-loop as that moves the iterator.
                    while let Some(&(ref l, ref r, sign)) = leaf_iter.next() {
                        if self.sign == sign {
                            let subst_copy = subst.clone();
                            // Remember to push the stack frame back as we are not done with this leaf.
                            self.stack.push(StackFrame {
                                subst: subst,
                                iter: iter,
                                tree_iter: Either::Right(leaf_iter),
                            });
                            return Some((l, r, subst_copy));
                        }
                    }
                },
            }
        }

        None
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
    
    #[test]
    fn pd_tree_2() {
        let x = Term::new_variable(-1);
        let x2 = Term::new_variable(-2);
        let f_x_x = Term::new_function(1, vec!(x.clone(), x.clone()));
        let f_x2_x2 = Term::new_function(1, vec!(x2.clone(), x2.clone()));
        let t = Term::new_truth();
        let lit = Literal::new(false, f_x_x.clone(), t.clone());
        let cl = Clause::new(vec!(lit));
        
        let mut pd_tree = PDTree::new();
        pd_tree.add_clause_to_index(&cl);
        let matches = pd_tree.iter_generalizations(&f_x2_x2, true).collect::<Vec<_>>();
        
        assert_eq!(matches.len(), 1);
        assert_eq!(*matches[0].0, f_x_x);
        assert_eq!(*matches[0].1, t);
        assert_eq!(matches[0].2.size(), 1);
    }
}