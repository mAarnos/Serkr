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

use utils::formula::{Term, Formula};

/// Renames variables so that different occurences of quantifiers bind different variables.
pub fn rename(f: Formula, n: &mut isize) -> Formula {
    match f {
        Formula::Not(box p) => Formula::Not(box rename(p, n)),
        Formula::And(box p, box q) => Formula::And(box rename(p, n), box rename(q, n)),
        Formula::Or(box p, box q) => Formula::Or(box rename(p, n), box rename(q, n)),
        Formula::Forall(s, box p) => rename_quantifier(s, p, n, true),
        Formula::Exists(s, box p) => rename_quantifier(s, p, n, false),
        _ => f
    }
}

fn rename_quantifier(s: String, p: Formula, n: &mut isize, universal_quantifier: bool) -> Formula {
    let new_var = format!("v{}", *n); 
    *n += 1;
    let renamed_p = rename(rename_variable(p, &s, &new_var), n);
    if universal_quantifier {
        Formula::Forall(new_var, box renamed_p)
    } else {
        Formula::Exists(new_var, box renamed_p)
    }
}

/// Renames a single variable in a formula. 
/// We assume that the new variable does not exist already.
fn rename_variable(f: Formula, from: &str, to: &str) -> Formula {
    match f {
        Formula::Predicate(s, terms) => Formula::Predicate(s, terms.into_iter().map(|t| rename_variable_in_term(t, from, to)).collect()),
        Formula::Not(box p) => Formula::Not(box rename_variable(p, from, to)),
        Formula::And(box p, box q) => Formula::And(box rename_variable(p, from, to), box rename_variable(q, from, to)),
        Formula::Or(box p, box q) => Formula::Or(box rename_variable(p, from, to), box rename_variable(q, from, to)),
        Formula::Forall(s, box p) => Formula::Forall(if s == from { to.to_owned() } else { s }, box rename_variable(p, from, to)),
        Formula::Exists(s, box p) => Formula::Exists(if s == from { to.to_owned() } else { s }, box rename_variable(p, from, to)),
        _ => f
    }
}

/// Renames a single variable in a term. 
/// We assume that the new variable does not exist already.
fn rename_variable_in_term(t: Term, from: &str, to: &str) -> Term {
    match t {
        Term::Variable(s) => if from == s { Term::Variable(to.to_owned()) } else { Term::Variable(s) },
        Term::Function(s, args) => Term::Function(s, args.into_iter().map(|t2| rename_variable_in_term(t2, from, to)).collect())
    }
}

#[cfg(test)]
mod test {
    use super::rename;
    use parser::internal_parser::parse;
    
    #[test]
    fn rename_1() {
        let mut n = 0;
        let f = parse("forall x. exists x. P(x)").unwrap(); 
        let correct_f = parse("forall v0. exists v1. P(v1)").unwrap(); 
        assert_eq!(rename(f, &mut n), correct_f);
    }
    
    #[test]
    fn rename_2() {
        let mut n = 0;
        let f = parse("forall x. forall y. (P(x, y) /\\ Q(y, z))").unwrap(); 
        let correct_f = parse("forall v0. forall v1. (P(v0, v1) /\\ Q(v1, z))").unwrap(); 
        assert_eq!(rename(f, &mut n), correct_f);
    }
    
    #[test]
    fn rename_3() {
        let mut n = 0;
        let f = parse("forall x. (R(x, x) /\\ exists y. (P(y) \\/ forall x. exists y. (R(x, y) \\/ forall z. Q(z))))").unwrap(); 
        let correct_f = parse("forall v0. (R(v0, v0) /\\ exists v1. (P(v1) \\/ forall v2. exists v3. (R(v2, v3) \\/ forall v4. Q(v4))))").unwrap(); 
        assert_eq!(rename(f, &mut n), correct_f);
    }
}    
