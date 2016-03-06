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

/// Mappings from literals, terms and variables to IDs.
/// Variables get a negative ID, functions get a positive ID while equality gets 0.
#[allow(missing_docs)]
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RenamingInfo {
    pub fun_map: HashMap<(String, usize, bool), i64>,
    pub var_map: HashMap<String, i64>,
    pub fun_cnt: i64,
    pub var_cnt: i64,
}

impl RenamingInfo {
    /// Creates a new struct for containing renaming info.
    pub fn new() -> RenamingInfo {
        RenamingInfo {
            var_map: HashMap::new(),
            fun_map: HashMap::new(),
            var_cnt: 0,
            fun_cnt: 0,
        }
    }

    /// If there already is an ID for the given variable name, return that.
    /// Otherwise create a new ID for it.
    pub fn get_variable_id(&mut self, s: String) -> i64 {
        if let Some(&id) = self.var_map.get(&s) {
            id
        } else {
            self.create_new_variable_id(s)
        }
    }

    /// If there already is an ID for the given function name and arity, return that.
    /// Otherwise create a new ID for it.
    pub fn get_function_id(&mut self, s: String, arity: usize, predicate: bool) -> i64 {
        if let Some(&id) = self.fun_map.get(&(s.clone(), arity, predicate)) {
            id
        } else {
            self.create_new_function_id(s, arity, predicate)
        }
    }

    /// Creates a new variable ID matching the given string.
    pub fn create_new_variable_id(&mut self, s: String) -> i64 {
        self.var_cnt -= 1;
        self.var_map.insert(s, self.var_cnt);
        self.var_cnt
    }

    /// Creates a new function ID matching the given string and arity.
    pub fn create_new_function_id(&mut self, s: String, arity: usize, predicate: bool) -> i64 {
        self.fun_cnt += 1;
        self.fun_map.insert((s, arity, predicate), self.fun_cnt);
        self.fun_cnt
    }
}
