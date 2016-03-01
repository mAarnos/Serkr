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

use parser::internal_parser::parser_grammar;
use parser::internal_parser::ast::Formula;

/// Parses a string into a FOL formula, if possible.
pub fn parse(s: &str) -> Result<Formula, String> {
    parser_grammar::parse_Formula(s).map_err(|x| format!("{:?}", x))
}

#[cfg(test)]
mod test {
    use parser::internal_parser::ast::Formula;
    use parser::internal_parser::parser_grammar::{parse_Term, parse_Formula};

    #[test]
    fn parser_term_erroneous() {
        assert!(parse_Term("   ").is_err());
        assert!(parse_Term("P").is_err());
        assert!(parse_Term("P(x, y)").is_err());
        assert!(parse_Term("f(x,)").is_err());
    }

    #[test]
    fn parser_term_whitespace_variance() {
        assert!(parse_Term("  x  ").is_ok());
        assert!(parse_Term("g(  x, y  )").is_ok());
        assert!(parse_Term(" f  ( x )  ").is_ok());
    }

    #[test]
    fn parser_variable() {
        assert!(parse_Term("x").is_ok());
        assert!(parse_Term("identityVariable1").is_ok());
    }

    #[test]
    fn parser_function() {
        assert!(parse_Term("f()").is_ok());
        assert!(parse_Term("f(x)").is_ok());
        assert!(parse_Term("f(x, y, z)").is_ok());
        assert!(parse_Term("f(f(g(x), z), h(y))").is_ok());
    }

    #[test]
    fn parser_true_false() {
        assert_eq!(parse_Formula("T").unwrap(), Formula::True);
        assert_eq!(parse_Formula("F").unwrap(), Formula::False);
        assert_neq!(parse_Formula("T").unwrap(), parse_Formula("T()").unwrap());
        assert_neq!(parse_Formula("F").unwrap(), parse_Formula("F()").unwrap());
    }

    #[test]
    fn true_and_false_2() {
        // Make sure that predicates starting with a T or F won't get parsed as True or False.
        assert_eq!(parse_Formula("Taken").unwrap(), Formula::Predicate("Taken".to_owned(), Vec::new()));
        assert_eq!(parse_Formula("Free").unwrap(), Formula::Predicate("Free".to_owned(), Vec::new()));
    }

    #[test]
    fn parser_predicate() {
        assert!(parse_Formula("P(x, y)").is_ok());
        assert!(parse_Formula("P(f(x), f(g(y)))").is_ok());
        assert!(parse_Formula("PredicateName14(x)").is_ok());
    }

    #[test]
    fn predicates_without_parenthesis() {
        let f1 = parse_Formula("P <=> Q").unwrap();
        let f2 = parse_Formula("Q(x, y) ==> Q").unwrap();

        assert_eq!(f1, parse_Formula("(P() <=> Q())").unwrap());
        assert_eq!(f2, parse_Formula("(Q(x, y) ==> Q())").unwrap());
    }

    #[test]
    fn parser_equals() {
        assert!(parse_Formula("x = f(x)").is_ok());
        assert!(parse_Formula("f(g(x)) = f(x)").is_ok());
    }

    #[test]
    fn parser_not_equals() {
        assert!(parse_Formula("x <> y").is_ok());
        assert!(parse_Formula("f(x) <> g(y)").is_ok());
    }

    #[test]
    fn parser_and() {
        assert!(parse_Formula("P(x) /\\ Q(x)").is_ok());
        assert!(parse_Formula("P(x) /\\ Q(x) /\\ R(x, y)").is_ok());
    }

    #[test]
    fn parser_or() {
        assert!(parse_Formula("P0(x) \\/ P1(x)").is_ok());
        assert!(parse_Formula("P0(x) \\/ P1(x) \\/ P2(x)").is_ok());
    }

    #[test]
    fn parser_implies() {
        assert!(parse_Formula("P ==> Q").is_ok());
        assert!(parse_Formula("P ==> Q ==> R").is_ok());
    }

    #[test]
    fn parser_equivalent() {
        assert!(parse_Formula("P <=> Q").is_ok());
        assert!(parse_Formula("P <=> Q <=> R").is_ok());
    }

    #[test]
    fn parser_quantifiers() {
        assert!(parse_Formula("forall x. P(x)").is_ok());
        assert!(parse_Formula("exists x. P(x)").is_ok());
        assert!(parse_Formula("forall x. exists x. P(x)").is_ok());
    }

    #[test]
    fn parser_not_and_precedence() {
        let f1 = parse_Formula("~ P /\\ Q");
        let f2 = parse_Formula("(~P) /\\ Q");
        
        assert!(f1.is_ok());
        assert!(f2.is_ok());
        assert_eq!(f1.unwrap(), f2.unwrap());
    }

    #[test]
    fn parser_and_or_precedence() {
        let f1 = parse_Formula("P /\\ Q \\/ R");
        let f2 = parse_Formula("((P /\\ Q) \\/ R)");
        
        assert!(f1.is_ok());
        assert!(f2.is_ok());
        assert_eq!(f1.unwrap(), f2.unwrap());
    }

    #[test]
    fn parser_or_implies_precedence() {
        let f1 = parse_Formula("P(x) \\/ Q(x) ==> R(x)");
        let f2 = parse_Formula("((P(x) \\/ Q(x)) ==> R(x))");
        
        assert!(f1.is_ok());
        assert!(f2.is_ok());
        assert_eq!(f1.unwrap(), f2.unwrap());
    }

    #[test]
    fn parser_implies_equivalent_precedence() {
        let f1 = parse_Formula("P ==> Q <=> ~Q ==> ~P");
        let f2 = parse_Formula("((P ==> Q) <=> (~Q ==> ~P))");
        
        assert!(f1.is_ok());
        assert!(f2.is_ok());
        assert_eq!(f1.unwrap(), f2.unwrap());
    }

    #[test]
    fn parser_equivalent_quantifier_precedence() {
        let f1 = parse_Formula("forall x. P(x) <=> Q(x)");
        let f2 = parse_Formula("((forall x. P(x)) <=> Q(x))");
        
        assert!(f1.is_ok());
        assert!(f2.is_ok());
        assert_eq!(f1.unwrap(), f2.unwrap());
    }
}    