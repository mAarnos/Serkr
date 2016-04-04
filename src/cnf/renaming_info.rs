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
use cnf::ast::{Term, Formula};
use cnf::free_variables::free_variables;

/// An enum for keeping track of the polarity of a formula.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[allow(missing_docs)]
pub enum Polarity {
    Positive,
    Negative,
    Neutral,
}

impl Polarity {
    /// Flips the polarity, if possible.
    /// That is, Positive -> Negative and vice versa, while Neutral stays the same.
    pub fn flip(&self) -> Polarity {
        match *self {
            Polarity::Positive => Polarity::Negative,
            Polarity::Negative => Polarity::Positive,
            Polarity::Neutral => Polarity::Neutral,
        }
    }
}

/// Contains a single definition used in formula renaming.
#[derive(Debug, Eq, PartialEq, Clone)]
struct Definition {
    f: Formula,
    predicate: Formula,
    polarity: Polarity,
}

impl Definition {
    /// Converts a definition into a formula.
    pub fn to_formula(&self) -> Formula {
        let vars = free_variables(&self.predicate);
        let f = match self.polarity {
            Polarity::Positive => {
                Formula::Implies(Box::new(self.predicate.clone()), Box::new(self.f.clone()))
            }
            Polarity::Negative => {
                Formula::Implies(Box::new(self.f.clone()), Box::new(self.predicate.clone()))
            }
            Polarity::Neutral => {
                Formula::Equivalent(Box::new(self.predicate.clone()), Box::new(self.f.clone()))
            }
        };
        vars.into_iter().fold(f, |acc, x| Formula::Forall(x, Box::new(acc)))
    }
}

/// Mappings from literals, terms and variables to IDs.
/// Variables get a negative ID, functions get a positive ID while equality gets 0.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RenamingInfo {
    fun_map: HashMap<(String, usize, bool), i64>,
    var_map: HashMap<String, i64>,
    fun_cnt: i64,
    var_cnt: i64,
    defs: Vec<Definition>,
}

impl RenamingInfo {
    /// Creates a new struct for containing renaming info.
    pub fn new() -> RenamingInfo {
        RenamingInfo {
            var_map: HashMap::new(),
            fun_map: HashMap::new(),
            var_cnt: 0,
            fun_cnt: 0,
            defs: Vec::new(),
        }
    }

    /// If there already is an ID for the given variable name, return that.
    /// Otherwise create a new ID for it.
    pub fn get_variable_id(&mut self, s: String) -> i64 {
        if let Some(&id) = self.var_map.get(&s) {
            id
        } else {
            self.var_cnt -= 1;
            self.var_map.insert(s, self.var_cnt);
            self.var_cnt
        }
    }

    /// If there already is an ID for the given function name and arity, return that.
    /// Otherwise create a new ID for it.
    pub fn get_function_id(&mut self, s: String, arity: usize, predicate: bool) -> i64 {
        if let Some(&id) = self.fun_map.get(&(s.clone(), arity, predicate)) {
            id
        } else {
            self.fun_cnt += 1;
            self.fun_map.insert((s, arity, predicate), self.fun_cnt);
            self.fun_cnt
        }
    }

    /// Creates a new skolem function ID.
    pub fn create_new_skolem_function_id(&mut self) -> i64 {
        self.fun_cnt += 1;
        self.fun_cnt
    }

    /// Creates a new variable ID:
    pub fn create_new_variable_id(&mut self) -> i64 {
        self.var_cnt -= 1;
        self.var_cnt
    }

    /// Returns the newest variable ID created.
    pub fn get_newest_variable_id(&self) -> i64 {
        self.var_cnt
    }

    /// Get a definition corresponding to a given formula, if it exists.
    pub fn get_definition(&mut self, f: &Formula, polarity: Polarity) -> Option<Formula> {
        match self.defs.iter().position(|x| x.f == *f) {
            Some(pos) => {
                // Check if we need to update the polarity of the definition.
                if self.defs[pos].polarity != Polarity::Neutral &&
                   self.defs[pos].polarity != polarity {
                    self.defs[pos].polarity = Polarity::Neutral;
                }
                Some(self.defs[pos].predicate.clone())
            }
            None => None,
        }
    }

    /// Creates a new definition for a given formula and polarity.
    pub fn create_new_definition(&mut self, f: &Formula, polarity: Polarity) -> Formula {
        self.fun_cnt += 1;
        let free_vars = free_variables(f).into_iter().map(Term::Variable).collect();
        let pred = Formula::Predicate(self.fun_cnt, free_vars);
        self.defs.push(Definition {
            f: f.clone(),
            predicate: pred.clone(),
            polarity: polarity,
        });
        pred
    }

    /// Clears all current definitions.
    /// Returns the formulae corresponding to the definitions.
    pub fn clear_definitions(&mut self) -> Vec<Formula> {
        let ret = self.defs.iter().map(|x| x.to_formula()).collect();
        self.defs.clear();
        ret
    }
}

impl Default for RenamingInfo {
    fn default() -> RenamingInfo {
        RenamingInfo::new()
    }
}

#[cfg(test)]
mod test {
    use super::RenamingInfo;

    #[test]
    fn get_variable_id_1() {
        let mut ri = RenamingInfo::new();
        let id1 = ri.get_variable_id("X".to_owned());
        let id2 = ri.get_variable_id("X".to_owned());
        let id3 = ri.get_variable_id("Y".to_owned());

        assert_eq!(id1, -1);
        assert_eq!(id1, id2);
        assert_eq!(id3, -2);
    }

    #[test]
    fn get_function_id_1() {
        let mut ri = RenamingInfo::new();
        let id1 = ri.get_function_id("f".to_owned(), 2, true);
        let id2 = ri.get_function_id("f".to_owned(), 2, true);
        let id3 = ri.get_function_id("f".to_owned(), 2, false);
        let id4 = ri.get_function_id("f".to_owned(), 3, true);
        let id5 = ri.get_function_id("g".to_owned(), 2, true);

        assert_eq!(id1, 1);
        assert_eq!(id1, id2);
        assert_eq!(id3, 2);
        assert_eq!(id4, 3);
        assert_eq!(id5, 4);
    }
}
