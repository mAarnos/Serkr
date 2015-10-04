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
    use super::*;
    
    #[test]
    fn dont_allow_pure_terms() {
        assert!(parse("x").is_err());
        assert!(parse("f(x)").is_err());
        assert!(parse("f(g(x), c)").is_err());
    }
 
    #[test]
    fn nested_predicates_and_parenthesis() {
        assert!(parse("P()").is_ok());
        assert!(parse("(P())").is_ok());
        assert!(parse("((P()))").is_ok());
        assert!(parse("(((P())))").is_ok());
    }
    
    #[test]
    fn nested_not() {
        assert!(parse("~P()").is_ok());
        assert!(parse("~(~P())").is_ok());
        assert!(parse("~(~~(P()))").is_ok());
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