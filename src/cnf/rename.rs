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

use parser::formula::{Term, Formula};

/// Renames variables so that different occurences of quantifiers bind different variables.
pub fn rename(f: Formula, n: &mut isize) -> Formula {
    match f {
        Formula::Not(box p) => Formula::Not(box rename(p, n)),
        Formula::And(box p, box q) => Formula::And(box rename(p, n), box rename(q, n)),
        Formula::Or(box p, box q) => Formula::Or(box rename(p, n), box rename(q, n)),
        Formula::Forall(s, box p) => { let new_var = format!("v_{}", *n); *n += 1; Formula::Forall(new_var.clone(), box rename(rename_variable(p, &s, &new_var), n)) },
        Formula::Exists(s, box p) => { let new_var = format!("v_{}", *n); *n += 1; Formula::Exists(new_var.clone(), box rename(rename_variable(p, &s, &new_var), n)) }
        _ => f
    }
}

/// Renames a single variable in a formula. 
/// We assume that the new variable does not exist already.
fn rename_variable(f: Formula, from: &str, to: &str) -> Formula {
    match f {
        Formula::Predicate(s, terms) => Formula::Predicate(s, terms.into_iter().map(|term| rename_variable_in_term(term, from, to)).collect()),
        Formula::Not(box p) => Formula::Not(box rename_variable(p, from, to)),
        Formula::And(box p, box q) => Formula::And(box rename_variable(p, from, to), box rename_variable(q, from, to)),
        Formula::Or(box p, box q) => Formula::Or(box rename_variable(p, from, to), box rename_variable(q, from, to)),
        Formula::Forall(s, box p) => Formula::Forall(if s == from { to.to_string() } else { s }, box rename_variable(p, from, to)),
        Formula::Exists(s, box p) => Formula::Exists(if s == from { to.to_string() } else { s }, box rename_variable(p, from, to)),
        _ => f
    }
}

/// Renames a single variable in a term. 
/// We assume that the new variable does not exist already.
fn rename_variable_in_term(t: Term, from: &str, to: &str) -> Term {
    match t {
        Term::Variable(s) => if from == s { Term::Variable(to.to_string()) } else { Term::Variable(s) },
        Term::Function(s, subterms) => Term::Function(s, subterms.into_iter().map(|term| rename_variable_in_term(term, from, to)).collect())
    }
}

#[cfg(test)]
mod test {
    use super::{rename};
    use parser::formula::{Term, Formula};
    
    #[test]
    fn rename_1() {
        let mut n = 0;
        // forall x. exists x. P(x)
        let f = Formula::Forall("x".to_string(), 
                                box Formula::Exists("x".to_string(), 
                                                    box Formula::Predicate("P".to_string(), 
                                                                            vec!(Term::Variable("x".to_string())))));
        // forall v_0. exists v_1. P(v_1)
        let correct_f = Formula::Forall("v_0".to_string(), 
                                        box Formula::Exists("v_1".to_string(), 
                                                            box Formula::Predicate("P".to_string(), 
                                                                                   vec!(Term::Variable("v_1".to_string())))));
        assert_eq!(rename(f, &mut n), correct_f);
    }
}    
