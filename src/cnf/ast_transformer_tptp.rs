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

use parser::tptp_parser::parser::parse_tptp_file;
use parser::tptp_parser::ast::AnnotatedFormula;
use parser::tptp_parser::ast::Term as ParserTerm;
use parser::tptp_parser::ast::Formula as ParserFormula;
use cnf::ast::Term as CnfTerm;
use cnf::ast::Formula as CnfFormula;
use cnf::renaming_info::RenamingInfo;

/// Transforms the AST format of the TPTP parser into the AST format of the CNF transformer.
/// Also returns info on whether the problem contains conjectures or not.
/// This information is important for correct proof output.
fn tptp_ast_to_cnf_ast(f_list: Vec<AnnotatedFormula>)
                       -> Result<(CnfFormula, RenamingInfo, bool), String> {
    // Check if any of the formula roles is incorrect.
    // Also, we hit the if let borrow bug again.
    if let Some(f) = f_list.iter().find(|&f| !formula_role_valid(f)) {
        match *f {
            AnnotatedFormula::Cnf(_) => return Err(cnf_formula_role_error(f)),
            AnnotatedFormula::Fof(_) => return Err(fof_formula_role_error(f)),
        }
    }

    // Check that we don't have both conjectures and negated conjectures in a problem.
    let contains_negated_conj = f_list.iter().any(|x| get_formula_role(x) == "negated_conjecture");
    let contains_conj = f_list.iter().any(|x| get_formula_role(x) == "conjecture");
    if contains_negated_conj && contains_conj {
        return Err("A problem shouldn't have both negated_conjectures and conjectures".to_owned());
    }

    let mut renaming_info = RenamingInfo::new();
    let (conj_annotated, other_annotated): (Vec<_>, Vec<_>) = f_list.into_iter().partition(|x| {
        get_formula_role(x) == "conjecture"
    });
    let conj = conj_annotated.into_iter()
                             .map(strip_annotations)
                             .map(|x| transform_ast(x, &mut renaming_info))
                             .collect::<Vec<_>>();
    let other = other_annotated.into_iter()
                               .map(strip_annotations)
                               .map(|x| transform_ast(x, &mut renaming_info))
                               .collect::<Vec<_>>();

    if conj.is_empty() {
        if other.len() == 1 {
            Ok((other[0].clone(), renaming_info, false))
        } else {
            Ok((CnfFormula::And(other), renaming_info, false))
        }
    } else if other.is_empty() {
        if conj.len() == 1 {
            Ok((conj[0].clone(), renaming_info, true))
        } else {
            Ok((CnfFormula::And(conj), renaming_info, true))
        }
    } else {
        let left = if other.len() == 1 {
            other[0].clone()
        } else {
            CnfFormula::And(other)
        };
        let right = if conj.len() == 1 {
            conj[0].clone()
        } else {
            CnfFormula::And(conj)
        };
        Ok((CnfFormula::Implies(Box::new(left), Box::new(right)),
            renaming_info,
            true))
    }
}

fn cnf_formula_role_error(f: &AnnotatedFormula) -> String {
    format!("Formula role was expected to be one of \
             'axiom|hypothesis|definition|assumption|lemma|theorem|negated_conjecture' instead of \
             {}",
            get_formula_role(f))
}

fn fof_formula_role_error(f: &AnnotatedFormula) -> String {
    format!("Formula role was expected to be one of \
             'axiom|hypothesis|definition|assumption|lemma|theorem|negated_conjecture|conjecture' \
             instead of {}",
            get_formula_role(f))
}

/// Strips all annotations from a single annotated formula.
fn strip_annotations(f: AnnotatedFormula) -> ParserFormula {
    match f {
        AnnotatedFormula::Cnf(cnf_f) | AnnotatedFormula::Fof(cnf_f) => cnf_f.2,
    }
}

/// Hack to get access to the formula role string.
fn get_formula_role(f: &AnnotatedFormula) -> String {
    match *f {
        AnnotatedFormula::Cnf(ref cnf_f) | AnnotatedFormula::Fof(ref cnf_f) => cnf_f.1.clone(),
    }
}

/// Checks if the formula role the given annotated formula has is valid.
fn formula_role_valid(f: &AnnotatedFormula) -> bool {
    match *f {
        AnnotatedFormula::Cnf(ref cnf_f) => formula_role_valid_cnf(&cnf_f.1),
        AnnotatedFormula::Fof(ref fof_f) => formula_role_valid_fof(&fof_f.1),
    }
}

/// Returns true if the formula role for a CNF formula is valid.
fn formula_role_valid_cnf(s: &str) -> bool {
    s == "axiom" || s == "hypothesis" || s == "definition" || s == "assumption" ||
    s == "lemma" || s == "theorem" || s == "negated_conjecture"
}

/// Returns true if the formula role for a FOF formula is valid.
fn formula_role_valid_fof(s: &str) -> bool {
    formula_role_valid_cnf(s) || s == "conjecture"
}

/// Parses the file at the location given by the string into a CNF AST, if possible.
/// Returns the renaming info used during the transformation.
/// Also returns information on whether the problem has conjectures or not.
pub fn tptp_to_cnf_ast(s: &str) -> Result<(CnfFormula, RenamingInfo, bool), String> {
    Ok(try!(tptp_ast_to_cnf_ast(try!(parse_tptp_file(s)))))
}

fn transform_ast(f: ParserFormula, ri: &mut RenamingInfo) -> CnfFormula {
    match f {
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
    } else if s == "$true" {
        CnfFormula::True
    } else if s == "$false" {
        CnfFormula::False
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
