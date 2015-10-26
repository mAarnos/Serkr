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

 use utils::formula::Formula;

/// The grammar for the internal parser.
peg! formula(r#"
    use utils::formula::{Term, Formula};
    
    #[pub]
    formula -> Formula
        = [ ]* f:spaced_formula [ ]* { f }
              
    spaced_formula -> Formula
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
            
    predicate -> Formula
        = s:predicate_name [(] tl:term_list [)] { Formula::Predicate(s, tl) }   
        / s:predicate_name { 
            match &*s {
                "T" => Formula::True, 
                "F" => Formula::False,
                _ => Formula::Predicate(s, Vec::new()), 
            }
        }
            
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

    term -> Term
        = function 
        / variable 
                                   
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
        = letter 
        / digit
              
    letter -> String
        = upperletter 
        / lowerletter
            
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
     use utils::formula::Formula;
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
        assert_eq!(parse("Taken").unwrap(), Formula::Predicate("Taken".to_owned(), Vec::new()));
        assert_eq!(parse("Free").unwrap(), Formula::Predicate("Free".to_owned(), Vec::new()));
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