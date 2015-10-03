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

peg! term(r#"
    use parser::formula::{Term};

    #[pub]
    term -> Term
         = function / variable 
                               
    function -> Term
             = s:term_name [(] tl:term_list [)] { Term::Function(s, tl) }
            
    term_list -> Vec<Term>
             = term ** [,]
     
    variable -> Term
             = s:term_name { Term::Variable(s) }     
        
    digit -> String
          = [0-9] { match_str.parse().unwrap() }
        
    lowerletter -> String
                = [a-z] { match_str.parse().unwrap() }
          
    upperletter -> String
                = [A-Z] { match_str.parse().unwrap() }
    
    letter -> String
            = upperletter / lowerletter
     
    letter_or_digit -> String
                     = letter / digit
     
    term_name -> String
                     = s:lowerletter xs:letter_or_digit* { 
                        let mut name = s;
                        for x in xs {
                            name.push_str(&x);
                        }                     
                        name
                     }
"#);

use parser::formula::{Term};

pub fn parse(s: &str) -> Result<Term, term::ParseError> {
    term::term(s)
}

#[cfg(test)]
mod test {
    use super::term::term;
    use parser::formula::{Term};
 
    #[test]
    fn term_test_1() {
        assert_eq!(term("p"), Ok(Term::Variable("p".to_string())));
        assert_eq!(term("healthy"), Ok(Term::Variable("healthy".to_string())));
        assert!(term("P").is_err());
    }
    
    #[test]
    fn term_test_2() {
        assert_eq!(term("f()"), Ok(Term::Function("f".to_string(), Vec::new())));
        assert_eq!(term("f(x,y)"), Ok(Term::Function("f".to_string(), vec!(Term::Variable("x".to_string()), Term::Variable("y".to_string())))));
        assert_eq!(term("f(g(x),y)"), Ok(Term::Function("f".to_string(), vec!(Term::Function("g".to_string(), vec!(Term::Variable("x".to_string()))), Term::Variable("y".to_string())))));
    }
}    