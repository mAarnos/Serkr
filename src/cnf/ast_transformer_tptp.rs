/*
    Serkr - An automated theorem prover. Copyright (C) 2015-2016 Mikko Aarnos.

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

#![allow(dead_code)]

use parser::tptp::parser::parse_tptp_file;
use parser::tptp::ast::AnnotatedFormula;
use parser::tptp::ast::Term as ParserTerm;
use parser::tptp::ast::Formula as ParserFormula;
use cnf::ast::Term as CnfTerm;
use cnf::ast::Formula as CnfFormula;
use cnf::renaming_info::RenamingInfo;

/// Transforms the AST format of the TPTP parser into the AST format of the CNF transformer.
fn tptp_ast_to_cnf_ast(f_list: Vec<ParserFormula>) -> (CnfFormula, RenamingInfo) {
    let mut renaming_info = RenamingInfo::new();
    let transformed_f = transform_vec_ast(f_list, &mut renaming_info);
    (transformed_f, renaming_info)
}

/// Strips all annotations.
/// Probably not a good idea.
fn annotated_to_normal_formula(f_list: Vec<AnnotatedFormula>) -> Vec<ParserFormula> {
    f_list.into_iter().map(|f| match f { AnnotatedFormula::Cnf(cnf_f) => cnf_f.2 }).collect()
}

/// Parses the file at the location given by the string into a CNF AST, if possible.
pub fn tptp_to_cnf_ast(s: &str) -> (CnfFormula, RenamingInfo) { 
    tptp_ast_to_cnf_ast(annotated_to_normal_formula(parse_tptp_file(s)))
}

fn transform_vec_ast(f_list: Vec<ParserFormula>, ri: &mut RenamingInfo) -> CnfFormula {
    assert!(!f_list.is_empty());
    let transformed_f_list = f_list.into_iter().map(|f| transform_ast(f, ri)).collect::<Vec<_>>();
    let starting_f = transformed_f_list[0].clone();
    transformed_f_list.into_iter().skip(1).fold(starting_f, |acc, f| CnfFormula::And(Box::new(acc), Box::new(f)))
}

fn transform_ast(f: ParserFormula, ri: &mut RenamingInfo) -> CnfFormula {
    match f {
        ParserFormula::Predicate(s, args) => transform_literal(s, args, ri),
        ParserFormula::Not(p) => CnfFormula::Not(Box::new(transform_ast(*p, ri))),
        ParserFormula::Or(p, q) => CnfFormula::Or(Box::new(transform_ast(*p, ri)), Box::new(transform_ast(*q, ri))),
    }
}

fn transform_literal(s: String, args: Vec<ParserTerm>, ri: &mut RenamingInfo) -> CnfFormula {
    if s == "=" {
        assert_eq!(args.len(), 2);
        CnfFormula::Predicate(0, args.into_iter().map(|t| transform_term(t, ri)).collect())
    } else if s == "$true" {
        CnfFormula::True
    } else if s == "$false" {
        CnfFormula::False
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

#[cfg(test)]
mod test {
    
} 