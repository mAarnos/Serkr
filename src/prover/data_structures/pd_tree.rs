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

use std::slice::Iter as VIter;
use std::collections::hash_map::Iter as MIter;
use prover::data_structures::clause::Clause;
use prover::data_structures::term::Term;
use prover::unification::substitution::Substitution;
use prover::ordering::term_ordering::TermOrdering;
use prover::unification::matching::term_match_with_subst;
use utils::either::Either;
use utils::hash_map::HashMap;

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
    let mut m = HashMap::default();
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
    Leaf(Vec<(Term, Term, bool, bool)>),
    Node(HashMap<i64, PDTree>)
}

impl PDTree {
    /// Creates an empty perfect discrimination tree.
    pub fn new() -> PDTree {
        PDTree::Node(HashMap::default())
    }

    /// Adds a clause to the index.
    /// Does nothing if the clause is not unit.
    pub fn add_clause_to_index(&mut self, term_ordering: &TermOrdering, cl: &Clause) {
        if cl.is_unit() {
            let lit = &cl[0];
            let pos = lit.is_positive();
            let l = &lit.get_lhs();
            let r = &lit.get_rhs();
            if term_ordering.gt(l, r) {
                self.add_eq_to_index(l, r, pos, true);
            } else if term_ordering.gt(r, l) {
                self.add_eq_to_index(r, l, pos, true);
            } else {
                self.add_eq_to_index(l, r, pos, false);
                self.add_eq_to_index(r, l, pos, false);
            }
        }
    }
    
    /// Adds an equation to the index without taking into account symmetry.
    pub fn add_eq_to_index(&mut self, s: &Term, t: &Term, pos: bool, oriented: bool) {
        let mut s_n = s.clone();
        let mut t_n = t.clone();
        normalize_variables(&mut s_n, &mut t_n);
        self.insert_at_leaf(PrefixOrderIterator::new(&s_n), &s_n, t_n, pos, oriented);
    }

    /// Inserts l = r (or l <> r) into the tree, constructing the path if it doesn't exist.
    /// Needs to be implemented iteratively as otherwise we get stack overflows.
    #[cfg_attr(feature="clippy", allow(while_let_on_iterator))]
    fn insert_at_leaf(&mut self, mut iter: PrefixOrderIterator, l: &Term, r: Term, sign: bool, oriented: bool) {
        let mut current = Some(self);
        
        while let Some(t) = iter.next() {
            current = current.take().and_then(|tree| {
                match *tree {
                    PDTree::Node(ref mut m) => {
                        Some(m.entry(t.get_id()).or_insert_with(|| {
                            if iter.peek().is_some() {
                                PDTree::new()
                            } else {
                                PDTree::Leaf(Vec::new())
                            }
                        }))
                    },
                    _ => None,
                }
            });
            
            if current.is_none() {
                break;
            }
        }
        
        if let Some(tree) = current {
            if let PDTree::Leaf(ref mut v) = *tree {
                v.push((l.clone(), r, sign, oriented));
            }
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

/// Either an iterator of the children of an node, or the stuff at a leaf node.
type LevelIter<'a> = Either<MIter<'a, i64, PDTree>, VIter<'a, (Term, Term, bool, bool)>>;

struct StackFrame<'a> {
    subst: Substitution,
    iter: PrefixOrderIterator<'a>, 
    tree_iter: LevelIter<'a>
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
    type Item = (&'a Term, &'a Term, Substitution, bool);
    
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
                    while let Some(&(ref l, ref r, sign, oriented)) = leaf_iter.next() {
                        if self.sign == sign {
                            let subst_copy = subst.clone();
                            // Remember to push the stack frame back as we are not done with this leaf.
                            self.stack.push(StackFrame {
                                subst: subst,
                                iter: iter,
                                tree_iter: Either::Right(leaf_iter),
                            });
                            return Some((l, r, subst_copy, oriented));
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
        
        let mut pd_tree = PDTree::new();
        pd_tree.add_eq_to_index(&c, &t, true, true);
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
        
        let mut pd_tree = PDTree::new();
        pd_tree.add_eq_to_index(&f_x_x, &t, true, true);
        let matches = pd_tree.iter_generalizations(&f_x2_x2, true).collect::<Vec<_>>();
        
        assert_eq!(matches.len(), 1);
        assert_eq!(*matches[0].0, f_x_x);
        assert_eq!(*matches[0].1, t);
        assert_eq!(matches[0].2.size(), 1);
    }
    
    #[test]
    fn pd_tree_3() {
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let a = Term::new_constant(1);
        let b = Term::new_constant(2);
        
        let f_a_x = Term::new_function(3, vec!(a.clone(), x.clone()));
        let f_b_x = Term::new_function(3, vec!(b.clone(), x.clone()));
        let f_x_x = Term::new_function(3, vec!(x.clone(), x.clone()));
        let f_y_x = Term::new_function(3, vec!(y.clone(), x.clone()));
        let f_x_y = Term::new_function(3, vec!(x.clone(), y.clone()));
        let g_f_x_x = Term::new_function(4, vec!(f_x_x));
        let g_f_x_y = Term::new_function(4, vec!(f_x_y));
        let g_x = Term::new_function(4, vec!(x.clone()));
        
        let g_b = Term::new_function(4, vec!(b.clone()));
        let f_a_g_b = Term::new_function(3, vec!(a.clone(), g_b));
        let g_f_a_g_b = Term::new_function(4, vec!(f_a_g_b));
        
        let mut pd_tree = PDTree::new();
        pd_tree.add_eq_to_index(&f_a_x, &a, true, true);
        pd_tree.add_eq_to_index(&f_b_x, &x, true, true);
        pd_tree.add_eq_to_index(&g_f_x_x, &f_y_x, true, true);
        pd_tree.add_eq_to_index(&g_f_x_y, &g_x, true, true);
        
        let matches = pd_tree.iter_generalizations(&g_f_a_g_b, true).collect::<Vec<_>>();
        
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].0, &g_f_x_y);
        assert_eq!(matches[0].1, &g_x);
        assert_eq!(matches[0].2.size(), 2);
    }
}