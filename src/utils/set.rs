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

use std::hash::{self, Hash, Hasher, SipHasher};
use std::collections::HashSet;
use std::collections::hash_set::{Iter, IntoIter, SymmetricDifference, Difference, Union, Intersection};
use std::collections::hash_map::RandomState;
use std::iter::FromIterator;
use std::fmt::{Debug, Formatter, Result};
use std::default::Default;
use std::borrow::Borrow;

#[derive(Clone, PartialEq, Eq)]
pub struct Set<T: Eq + Hash + Clone + Debug>(HashSet<T>);

impl<T: Eq + Hash + Clone + Debug> Set<T> {
    pub fn new() -> Set<T> {
        Set(HashSet::<T>::new())
    }
    
    // Returns the one-element set containing only x.
    pub fn singleton(value: T) -> Set<T> {
        let mut s = Set(HashSet::<T>::new());
        s.insert(value);
        s
    }
    
    pub fn iter(&self) -> Iter<T> {
        self.0.iter()
    }
    
    pub fn difference<'a>(&'a self, other: &'a Set<T>) -> Difference<'a, T, RandomState> {
        self.0.difference(&other.0)
    }
    
    pub fn symmetric_difference<'a>(&'a self, other: &'a Set<T>) -> SymmetricDifference<'a, T, RandomState> {
        self.0.symmetric_difference(&other.0)
    }
    
    pub fn intersection<'a>(&'a self, other: &'a Set<T>) -> Intersection<'a, T, RandomState> {
        self.0.intersection(&other.0)
    }
    
    pub fn union<'a>(&'a self, other: &'a Set<T>) -> Union<'a, T, RandomState> {
        self.0.union(&other.0)
    }
    
    // Returns the cardinality of the set.
    pub fn cardinality(&self) -> usize {
        self.0.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    
    // Returns true if the set is a singleton set.
    pub fn is_singleton(&self) -> bool {   
        self.cardinality() == 1
    }
    
    pub fn clear(&mut self) {
        self.0.clear();
    }
    
    pub fn contains<Q: ?Sized>(&self, value: &Q) -> bool where T: Borrow<Q>, Q: Hash + Eq {
        self.0.contains(value)
    }
    
    pub fn is_disjoint(&self, other: &Set<T>) -> bool {
        self.0.is_disjoint(&other.0)
    }
    
    pub fn is_subset(&self, other: &Set<T>) -> bool {
        self.0.is_subset(&other.0)
    }
    
    // Returns true if the set is a proper subset of another.
    pub fn is_proper_subset(&self, other: &Set<T>) -> bool {
        self != other && self.is_subset(other)
    }
    
    pub fn is_superset(&self, other: &Set<T>) -> bool {
        self.0.is_superset(&other.0)
    }
    
    // Returns true if the set is a proper superset of another.
    pub fn is_proper_superset(&self, other: &Set<T>) -> bool {
        self != other && self.is_superset(other)
    }
    
    pub fn insert(&mut self, value: T) -> bool {
        self.0.insert(value)
    }
    
    pub fn remove<Q: ?Sized>(&mut self, value: &Q) -> bool where T: Borrow<Q>, Q: Hash + Eq {
        self.0.remove(value)
    }
}

// If we want to have a set of sets (or anything similar) this is necessary.
// This also happens to be an utter and complete hack which undermines the security properties provided by SipHash.
impl<T: Eq + Hash + Clone + Debug> Hash for Set<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        let mut h = 0;
        for x in self.iter() {
            let mut s = SipHasher::new_with_keys(0, 0);
            x.hash(&mut s);
            h ^= s.finish();
        }
        Hash::hash(&h, state);
    }
}

impl<T: Eq + Hash + Clone + Debug> FromIterator<T> for Set<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iterable: I) -> Set<T> {
        let iter = iterable.into_iter();
        let lower = iter.size_hint().0;
        let mut set = HashSet::with_capacity_and_hash_state(lower, Default::default());
        set.extend(iter);
        Set(set)
    }
}

impl<T: Eq + Hash + Clone + Debug> IntoIterator for Set<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        self.0.into_iter()
    }
}

impl<T: Eq + Hash + Clone + Debug> Extend<T> for Set<T> {
    fn extend<I: IntoIterator<Item=T>>(&mut self, iter: I) {
        self.0.extend(iter)
    }
}

impl<T: Eq + Hash + Clone + Debug> Default for Set<T> {
    fn default() -> Set<T> {
        Set(HashSet::default())
    }
}

impl<T: Eq + Hash + Clone + Debug> Debug for Set<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.0.fmt(f)
    }
}
