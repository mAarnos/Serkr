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

peg! formula(r#"
    use parser::formula::{Term, Formula};
    
    #[pub]
    formula -> Formula
         = [ ]* f:complex_formula [ ]* { f }
          
    complex_formula -> Formula
        = parenthesis_formula
        / predicate
        / not_formula
        / and_formula
        / or_formula
        / implies_formula
        / equivalent_formula
        / forall_formula
        / exists_formula
        
    parenthesis_formula -> Formula
        = [(] f:formula [)] { f }
        
    not_formula -> Formula
        = "~" f:formula { Formula::Not(box f) }
        
    and_formula -> Formula
        = [(] f1:formula "/\\" f2:formula [)] { Formula::And(box f1, box f2) }   
    
    or_formula -> Formula
        = [(] f1:formula "\\/" f2:formula [)] { Formula::Or(box f1, box f2) }    

    implies_formula -> Formula
        = [(] f1:formula "==>" f2:formula [)] { Formula::Implies(box f1, box f2) }  

    equivalent_formula -> Formula
        = [(] f1:formula "<=>" f2:formula [)] { Formula::Equivalent(box f1, box f2) }   

    forall_formula -> Formula
        = "forall " [ ]* v:term_name [ ]* [.] f:formula { Formula::Forall(v, box f) }
        
    exists_formula -> Formula
        = "exists " [ ]* v:term_name [ ]* [.] f:formula { Formula::Exists(v, box f) }
        
    predicate -> Formula
        = s:predicate_name [(] tl:term_list [)] { Formula::Predicate(s, tl) }    

    term -> Term
        = function / variable 
                               
    function -> Term
        = s:term_name [(] tl:term_list [)] { Term::Function(s, tl) }
            
    term_list -> Vec<Term>
        = spaced_term ** [,]
        
    spaced_term -> Term
        = [ ]* t:term [ ]* { t }
     
    variable -> Term
        = s:term_name { Term::Variable(s) }    
        
    term_name -> String
        = s:lowerletter xs:letter_or_digit* { 
            let mut name = s;
            for x in xs {
                name.push_str(&x);
            }                     
            name
        }
                               
    predicate_name -> String
        = s:upperletter xs:letter_or_digit* { 
            let mut name = s;
            for x in xs {
                name.push_str(&x);
            }                     
            name
        }       
          
    letter_or_digit -> String
        = letter / digit
          
    letter -> String
        = upperletter / lowerletter
        
    upperletter -> String
        = [A-Z] { match_str.parse().unwrap() }
        
    lowerletter -> String
        = [a-z] { match_str.parse().unwrap() }
        
    digit -> String
        = [0-9] { match_str.parse().unwrap() }              
"#);

pub fn parse(s: &str) -> Result<Formula, formula::ParseError> {
    formula::formula(s)
}

#[cfg(test)]
mod test {
    use super::*;
 
    #[test]
    fn parser_1() {
        assert!(parse("P()").is_ok());
        assert!(parse("(P())").is_ok());
        assert!(parse("((P()))").is_ok());
        assert!(parse("(((P())))").is_ok());
    }
    
    #[test]
    fn parser_2() {
        assert!(parse("~P()").is_ok());
        assert!(parse("~(P())").is_ok());
        assert!(parse("(~~(P()))").is_ok());
    }
    
    #[test]
    fn parser_3() {
        assert!(parse("((Equal(x, y) /\\ Equal(y, z)) ==> Equal(x, z))").is_ok());
        assert!(parse("(P(x) <=> (Q(x) <=> R(x)))").is_ok());
        assert!(parse("(P()/\\Q())").is_ok());
        assert!(parse("forall x. P(x)").is_ok());
        assert!(parse("forall x. exists y. Equal(x, y)").is_ok());
        assert!(parse("forall x. (exists y. (Equal(x, y)))").is_ok());
        assert!(parse("P( f(x) )").is_ok());
    }  
}    