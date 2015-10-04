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

use parser::formula::{Formula};

peg_file! formula("fol_grammar.rustpeg");

pub fn parse(s: &str) -> Result<Formula, formula::ParseError> {
    formula::formula(s)
}

#[cfg(test)]
mod test {
    use parser::formula::{Formula};
    use super::*;
    
    #[test]
    fn dont_allow_pure_terms() {
        assert!(parse("x").is_err());
        assert!(parse("f(x)").is_err());
        assert!(parse("f(g(x), c)").is_err());
    }
 
    #[test]
    fn nested_parenthesis() {
        let f1 = parse("P()");
        let f2 = parse("(P())");
        let f3 = parse("((P()))");
        let f4 = parse("(((P())))");
        
        assert!(f1.is_ok());
        assert!(f2.is_ok());
        assert!(f3.is_ok());
        assert!(f4.is_ok());
        assert_eq!(f1, f2);
        assert_eq!(f2, f3);
        assert_eq!(f3, f4);
    }
    
    #[test]
    fn nested_not() {
        assert!(parse("~P()").is_ok());
        assert!(parse("~(~P())").is_ok());
        assert!(parse("~(~~(P()))").is_ok());
    }
    
    #[test]
    fn predicates_without_parenthesis() {
        let f1 = parse("(P <=> Q)");
        let f2 = parse("(Q(x, y) ==> Q)");
        
        assert!(f1.is_ok());
        assert!(f2.is_ok());
        assert_eq!(f1, parse("(P() <=> Q())"));
        assert_eq!(f2, parse("(Q(x, y) ==> Q())"));
    }
    
    #[test]
    fn true_and_false() {
        assert_eq!(parse("T").unwrap(), Formula::True);
        assert_eq!(parse("F").unwrap(), Formula::False);
        assert!(parse("T") != parse("T()"));
        assert!(parse("F") != parse("F()"));
        assert_eq!(parse("(T <=> T)").unwrap(), Formula::Equivalent(box Formula::True, box Formula::True));
    }
    
    #[test]
    fn true_and_false_2() {
        // Make sure that predicates starting with a T or F won't get parsed as True or False.
        assert_eq!(parse("Taken").unwrap(), Formula::Predicate("Taken".to_string(), Vec::new()));
        assert_eq!(parse("Free").unwrap(), Formula::Predicate("Free".to_string(), Vec::new()));
    }
    
    #[test]
    fn general() {
        assert!(parse("((Equal(x, y) /\\ Equal(y, z)) ==> Equal(x, z))").is_ok());
        assert!(parse("(P(x) <=> (Q(x) <=> R(x)))").is_ok());
        assert!(parse("(P()/\\Q())").is_ok());
        assert!(parse("forall x. P(x)").is_ok());
        assert!(parse("forall x. exists y. Equal(x, y)").is_ok());
        assert!(parse("forall x. (exists y. (Equal(x, y)))").is_ok());
        assert!(parse("P( f(x) )").is_ok());
    }  
}    