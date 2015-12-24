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
use parser::internal_parser::parser::{parse, ParseError};
use parser::internal_parser::ast::Term as ParserTerm;
use parser::internal_parser::ast::Formula as ParserFormula;
use cnf::ast::Term as CnfTerm;
use cnf::ast::Formula as CnfFormula;

/// Mappings from literals, terms and variables to IDs.
/// Variables get a negative ID, functions get a positive ID while equality gets 0.
#[derive(Debug, Eq, PartialEq, Clone)]
#[allow(missing_docs)]
pub struct RenamingInfo {
    pub fun_map: HashMap<(String, usize), i64>,
    pub var_map: HashMap<String, i64>,
    pub fun_cnt: i64,
    pub var_cnt: i64,
}

impl RenamingInfo {
    /// Creates a new struct for containing renaming info.
    pub fn new() -> RenamingInfo {
        RenamingInfo { var_map: HashMap::new(), fun_map: HashMap::new(),
                       var_cnt: 0, fun_cnt: 0 }
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
    pub fn get_function_id(&mut self, s: String, arity: usize) -> i64 {
        if let Some(&id) = self.fun_map.get(&(s.clone(), arity)) {
            id
        } else {
            self.create_new_function_id(s, arity)
        }
    }
    
    /// Creates a new variable ID matching the given string.
    pub fn create_new_variable_id(&mut self, s: String) -> i64 {
        self.var_cnt -= 1;
        self.var_map.insert(s, self.var_cnt);
        self.var_cnt
    }
    
    /// Creates a new function ID matching the given string and arity.
    pub fn create_new_function_id(&mut self, s: String, arity: usize) -> i64 {
        self.fun_cnt += 1;
        self.fun_map.insert((s, arity), self.fun_cnt);
        self.fun_cnt
    }
}

/// Transforms the AST format given by the internal parser into the AST format of the CNF transformer.
fn parser_ast_to_cnf_ast(f: ParserFormula, mut renaming_info: RenamingInfo) -> (CnfFormula, RenamingInfo) {
    let transformed_f = transform_ast(f, &mut renaming_info);
    (transformed_f, renaming_info)
}

/// Parses the given string into the CNF AST, if possible.
pub fn parse_to_cnf_ast(s: &str) -> Result<(CnfFormula, RenamingInfo), ParseError> { 
    parse_to_cnf_ast_general(s, RenamingInfo::new())
}

/// Parses the given string into the CNF AST, if possible.
/// Uses the given renaming info as a starting point for the IDs.
pub fn parse_to_cnf_ast_general(s: &str, renaming_info: RenamingInfo) -> Result<(CnfFormula, RenamingInfo), ParseError> { 
    let f_result = parse(s);
    if let Ok(f) = f_result {
        Ok(parser_ast_to_cnf_ast(f, renaming_info))
    } else {
        Err(f_result.unwrap_err())
    }
}

fn transform_ast(f: ParserFormula, ri: &mut RenamingInfo) -> CnfFormula {
    match f {
        ParserFormula::True => CnfFormula::True,
        ParserFormula::False => CnfFormula::False,
        ParserFormula::Predicate(s, args) => transform_literal(s, args, ri),
        ParserFormula::Not(p) => CnfFormula::Not(Box::new(transform_ast(*p, ri))),
        ParserFormula::And(p, q) => CnfFormula::And(Box::new(transform_ast(*p, ri)), Box::new(transform_ast(*q, ri))),
        ParserFormula::Or(p, q) => CnfFormula::Or(Box::new(transform_ast(*p, ri)), Box::new(transform_ast(*q, ri))),
        ParserFormula::Implies(p, q) => CnfFormula::Implies(Box::new(transform_ast(*p, ri)), Box::new(transform_ast(*q, ri))),
        ParserFormula::Equivalent(p, q) => CnfFormula::Equivalent(Box::new(transform_ast(*p, ri)), Box::new(transform_ast(*q, ri))),
        ParserFormula::Forall(s, p) => transform_quantifier(s, *p, ri, true),
        ParserFormula::Exists(s, p) => transform_quantifier(s, *p, ri, false),
    }
}

fn transform_literal(s: String, args: Vec<ParserTerm>, ri: &mut RenamingInfo) -> CnfFormula {
    if s == "=" {
        assert_eq!(args.len(), 2);
        CnfFormula::Predicate(0, args.into_iter().map(|t| transform_term(t, ri)).collect())
    } else {
        let id = ri.get_function_id(s, args.len());
        CnfFormula::Predicate(id, args.into_iter().map(|t| transform_term(t, ri)).collect())
    }
}

fn transform_term(t: ParserTerm, ri: &mut RenamingInfo) -> CnfTerm {
    match t {
        ParserTerm::Variable(s) => CnfTerm::Variable(ri.get_variable_id(s)),
        ParserTerm::Function(s, args) => {
            let id = ri.get_function_id(s, args.len());
            CnfTerm::Function(id, args.into_iter().map(|t| transform_term(t, ri)).collect())
        }
    }
}

fn transform_quantifier(s: String, p: ParserFormula, ri: &mut RenamingInfo, forall_quantifier: bool) -> CnfFormula {
    let id = ri.get_variable_id(s);
    let new_p = Box::new(transform_ast(p, ri));

    if forall_quantifier {
        CnfFormula::Forall(id, new_p)
    } else {
        CnfFormula::Exists(id, new_p)
    }
}

#[cfg(test)]
mod test {
    
} 