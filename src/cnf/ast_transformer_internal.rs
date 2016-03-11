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

use cnf::renaming_info::RenamingInfo;
use cnf::ast::Term as CnfTerm;
use cnf::ast::Formula as CnfFormula;
use parser::internal_parser::parser::parse;
use parser::internal_parser::ast::Term as ParserTerm;
use parser::internal_parser::ast::Formula as ParserFormula;

/// Transforms the AST format of the internal parser into the AST format of the CNF transformer.
fn internal_ast_to_cnf_ast(f: ParserFormula,
                           mut renaming_info: RenamingInfo)
                           -> (CnfFormula, RenamingInfo) {
    let transformed_f = transform_ast(f, &mut renaming_info);
    (transformed_f, renaming_info)
}

/// Parses the given string into the CNF AST, if possible.
pub fn internal_to_cnf_ast(s: &str) -> Result<(CnfFormula, RenamingInfo), String> {
    internal_to_cnf_ast_general(s, RenamingInfo::new())
}

/// Parses the given string into the CNF AST, if possible.
/// Uses the given renaming info as a starting point for the IDs.
pub fn internal_to_cnf_ast_general(s: &str,
                                   renaming_info: RenamingInfo)
                                   -> Result<(CnfFormula, RenamingInfo), String> {
    let f_result = parse(s);
    if let Ok(f) = f_result {
        Ok(internal_ast_to_cnf_ast(f, renaming_info))
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
        ParserFormula::And(p, q) => transform_and(*p, *q, ri),
        ParserFormula::Or(p, q) => transform_or(*p, *q, ri),
        ParserFormula::Implies(p, q) => {
            CnfFormula::Implies(Box::new(transform_ast(*p, ri)),
                                Box::new(transform_ast(*q, ri)))
        }
        ParserFormula::Equivalent(p, q) => {
            CnfFormula::Equivalent(Box::new(transform_ast(*p, ri)),
                                   Box::new(transform_ast(*q, ri)))
        }
        ParserFormula::Forall(s, p) => transform_quantifier(s, *p, ri, true),
        ParserFormula::Exists(s, p) => transform_quantifier(s, *p, ri, false),
    }
}

fn transform_and(p: ParserFormula, q: ParserFormula, ri: &mut RenamingInfo) -> CnfFormula {
    let mut l = Vec::<CnfFormula>::new();
    collect_and(p, &mut l, ri);
    collect_and(q, &mut l, ri);
    CnfFormula::And(l)
}

fn collect_and(f: ParserFormula, l: &mut Vec<CnfFormula>, ri: &mut RenamingInfo) {
    match f {
        ParserFormula::And(p, q) => {
            collect_and(*p, l, ri);
            collect_and(*q, l, ri)
        }
        _ => l.push(transform_ast(f, ri)),
    }
}

fn transform_or(p: ParserFormula, q: ParserFormula, ri: &mut RenamingInfo) -> CnfFormula {
    let mut l = Vec::<CnfFormula>::new();
    collect_or(p, &mut l, ri);
    collect_or(q, &mut l, ri);
    CnfFormula::Or(l)
}

fn collect_or(f: ParserFormula, l: &mut Vec<CnfFormula>, ri: &mut RenamingInfo) {
    match f {
        ParserFormula::Or(p, q) => {
            collect_or(*p, l, ri);
            collect_or(*q, l, ri)
        }
        _ => l.push(transform_ast(f, ri)),
    }
}

fn transform_literal(s: String, args: Vec<ParserTerm>, ri: &mut RenamingInfo) -> CnfFormula {
    if s == "=" {
        assert_eq!(args.len(), 2);
        CnfFormula::Predicate(0, args.into_iter().map(|t| transform_term(t, ri)).collect())
    } else {
        let id = ri.get_function_id(s, args.len(), true);
        CnfFormula::Predicate(id,
                              args.into_iter().map(|t| transform_term(t, ri)).collect())
    }
}

fn transform_term(t: ParserTerm, ri: &mut RenamingInfo) -> CnfTerm {
    match t {
        ParserTerm::Variable(s) => CnfTerm::Variable(ri.get_variable_id(s)),
        ParserTerm::Function(s, args) => {
            let id = ri.get_function_id(s, args.len(), false);
            CnfTerm::Function(id,
                              args.into_iter().map(|t| transform_term(t, ri)).collect())
        }
    }
}

fn transform_quantifier(s: String,
                        p: ParserFormula,
                        ri: &mut RenamingInfo,
                        forall_quantifier: bool)
                        -> CnfFormula {
    let id = ri.get_variable_id(s);
    let new_p = Box::new(transform_ast(p, ri));

    if forall_quantifier {
        CnfFormula::Forall(id, new_p)
    } else {
        CnfFormula::Exists(id, new_p)
    }
}

#[cfg(test)]
mod test {}
