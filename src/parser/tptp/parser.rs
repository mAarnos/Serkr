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

#[cfg(test)]
mod test {
    use parser::tptp::parser_grammar::*;
    use parser::tptp::ast::*;
    
    #[test]
    fn parse_cnf_annotated_propositional() {
        let res = parse_cnf_annotated("cnf(propositional,axiom,( p0 | ~ q0 | r0 | ~ s0 )).");
        
        let p0 = Formula::Predicate("p0".to_owned(), Vec::new());
        let not_q0 = Formula::Not(Box::new(Formula::Predicate("q0".to_owned(), Vec::new())));
        let r0 = Formula::Predicate("r0".to_owned(), Vec::new());
        let not_s0 = Formula::Not(Box::new(Formula::Predicate("s0".to_owned(), Vec::new())));
        let fm = Formula::Or(Box::new(Formula::Or(Box::new(Formula::Or(Box::new(p0), Box::new(not_q0))), Box::new(r0))), Box::new(not_s0));
        
        assert_eq!(res.unwrap(), ("propositional".to_owned(), "axiom".to_owned(), fm, None));
    }
    
    #[test]
    fn parse_cnf_annotated_first_order() {
        let res = parse_cnf_annotated("cnf(first_order,axiom,( p(X) | ~ q(X,a) | r(X,f(Y),g(X,f(Y),Z)) | ~ s(f(f(f(b)))) )).");
        
        let x = Term::Variable("X".to_owned());
        let y = Term::Variable("Y".to_owned());
        let z = Term::Variable("Z".to_owned());
        let a = Term::Function("a".to_owned(), Vec::new());
        let b = Term::Function("b".to_owned(), Vec::new());
        let f_y = Term::Function("f".to_owned(), vec!(y.clone()));
        let g_x_f_y_z = Term::Function("g".to_owned(), vec!(x.clone(), f_y.clone(), z.clone()));
        let f_f_f_b = Term::Function("f".to_owned(), vec!(Term::Function("f".to_owned(), vec!(Term::Function("f".to_owned(), vec!(b))))));
        
        let p_x = Formula::Predicate("p".to_owned(), vec!(x.clone()));        
        let not_q_x_a = Formula::Not(Box::new(Formula::Predicate("q".to_owned(), vec!(x.clone(), a))));  
        let r_x_f_y_g_x_f_y_z = Formula::Predicate("r".to_owned(), vec!(x, f_y, g_x_f_y_z));  
        let not_s_f_f_f_b = Formula::Not(Box::new(Formula::Predicate("s".to_owned(), vec!(f_f_f_b))));  
        
        let fm = Formula::Or(Box::new(Formula::Or(Box::new(Formula::Or(Box::new(p_x), Box::new(not_q_x_a))), Box::new(r_x_f_y_g_x_f_y_z))), Box::new(not_s_f_f_f_b));
        
        assert_eq!(res.unwrap(), ("first_order".to_owned(), "axiom".to_owned(), fm, None));
    }
    
    #[test]
    fn parse_cnf_annotated_equality() {
        let res = parse_cnf_annotated("cnf(equality,axiom,( f(Y) = g(X,f(Y),Z) | f(f(f(b))) != a | X = f(Y) )).");
        
        let x = Term::Variable("X".to_owned());
        let y = Term::Variable("Y".to_owned());
        let z = Term::Variable("Z".to_owned());
        let a = Term::Function("a".to_owned(), Vec::new());
        let b = Term::Function("b".to_owned(), Vec::new());
        let f_y = Term::Function("f".to_owned(), vec!(y.clone()));
        let g_x_f_y_z = Term::Function("g".to_owned(), vec!(x.clone(), f_y.clone(), z.clone()));
        let f_f_f_b = Term::Function("f".to_owned(), vec!(Term::Function("f".to_owned(), vec!(Term::Function("f".to_owned(), vec!(b))))));
        
        let first_eq = Formula::Predicate("=".to_owned(), vec!(f_y.clone(), g_x_f_y_z));        
        let second_eq = Formula::Not(Box::new(Formula::Predicate("=".to_owned(), vec!(f_f_f_b, a))));  
        let third_eq = Formula::Predicate("=".to_owned(), vec!(x, f_y));
        
        let fm = Formula::Or(Box::new(Formula::Or(Box::new(first_eq), Box::new(second_eq))), Box::new(third_eq));
        
        assert_eq!(res.unwrap(), ("equality".to_owned(), "axiom".to_owned(), fm, None));
    }
    
    #[test]
    fn parse_cnf_annotated_true_false() {
        let res = parse_cnf_annotated("cnf(true_false,axiom,( $true | $false )).");
        
        let t = Formula::Predicate("$true".to_owned(), Vec::new());
        let f = Formula::Predicate("$false".to_owned(), Vec::new());
        let fm = Formula::Or(Box::new(t), Box::new(f));
        
        assert_eq!(res.unwrap(), ("true_false".to_owned(), "axiom".to_owned(), fm, None));
    }
    
    #[test]
    fn parse_cnf_annotated_single_quoted() {
        let res = parse_cnf_annotated("cnf(single_quoted,axiom,( 'A proposition' | 'A predicate'(Y) | p('A constant') | p('A function'(a)) | p('A \\'quoted \\ escape\\'') )).");
        
        let first = Formula::Predicate("A proposition".to_owned(), Vec::new());
        let second = Formula::Predicate("A predicate".to_owned(), vec!(Term::Variable("Y".to_owned())));
        let third = Formula::Predicate("p".to_owned(), vec!(Term::Function("A constant".to_owned(), Vec::new())));
        let fourth = Formula::Predicate("p".to_owned(), vec!(Term::Function("A function".to_owned(), vec!(Term::Function("a".to_owned(), Vec::new())))));
        let fifth = Formula::Predicate("p".to_owned(), vec!(Term::Function("A \\'quoted \\ escape\\'".to_owned(), Vec::new())));
        let fm = Formula::Or(Box::new(Formula::Or(Box::new(Formula::Or(Box::new(Formula::Or(Box::new(first), Box::new(second))), Box::new(third))), Box::new(fourth))), Box::new(fifth));
        
        assert_eq!(res.unwrap(), ("single_quoted".to_owned(), "axiom".to_owned(), fm, None));
    }
    
    #[test]
    fn parse_cnf_annotated_distinct_object() {
        let res = parse_cnf_annotated("cnf(distinct_object,axiom,( \"An Apple\" != \"A \\\"Microsoft \\ escape\\\"\" )).");
        
        let fst_p = Term::Function("An Apple".to_owned(), Vec::new());
        let snd_p = Term::Function("A \\\"Microsoft \\ escape\\\"".to_owned(), Vec::new());
        let fm = Formula::Not(Box::new(Formula::Predicate("=".to_owned(), vec!(fst_p, snd_p))));
        
        assert_eq!(res.unwrap(), ("distinct_object".to_owned(), "axiom".to_owned(), fm, None));
    }
    
    #[test]
    fn parse_cnf_annotated_integers() {
        let res = parse_cnf_annotated("cnf(integers,axiom,( p(12) | p(-12) )).").unwrap();
        
        let fst_p = Formula::Predicate("p".to_owned(), vec!(Term::Function("12".to_owned(), Vec::new())));
        let snd_p = Formula::Predicate("p".to_owned(), vec!(Term::Function("-12".to_owned(), Vec::new())));
        let fm = Formula::Or(Box::new(fst_p), Box::new(snd_p));
        
        assert_eq!(res, ("integers".to_owned(), "axiom".to_owned(), fm, None));
    }
    
    #[test]
    fn parse_cnf_annotated_rationals() {
        let res = parse_cnf_annotated("cnf(rationals ,axiom , ( p(123/456) | p(-123/456) | p(+123/456) )).").unwrap();
        
        let fst_p = Formula::Predicate("p".to_owned(), vec!(Term::Function("123/456".to_owned(), Vec::new())));
        let snd_p = Formula::Predicate("p".to_owned(), vec!(Term::Function("-123/456".to_owned(), Vec::new())));
        let trd_p = Formula::Predicate("p".to_owned(), vec!(Term::Function("+123/456".to_owned(), Vec::new())));
        let fm = Formula::Or(Box::new(Formula::Or(Box::new(fst_p), Box::new(snd_p))), Box::new(trd_p));
        
        assert_eq!(res, ("rationals".to_owned(), "axiom".to_owned(), fm, None));
    }
    
    #[test]
    fn parse_cnf_annotated_reals() {
        let res = parse_cnf_annotated("cnf(reals,axiom,
                                           ( p(123.456 )
                                           | p(-123.456 )
                                           | p(123.456E789 )
                                           | p(123.456e789 )
                                           | p(-123.456E789 )
                                           | p(123.456E-789 )
                                           | p(-123.456E-789 ) )).").unwrap();
        
        let p1 = Formula::Predicate("p".to_owned(), vec!(Term::Function("123.456".to_owned(), Vec::new())));
        let p2 = Formula::Predicate("p".to_owned(), vec!(Term::Function("-123.456".to_owned(), Vec::new())));
        let p3 = Formula::Predicate("p".to_owned(), vec!(Term::Function("123.456E789".to_owned(), Vec::new())));
        let p4 = Formula::Predicate("p".to_owned(), vec!(Term::Function("123.456e789".to_owned(), Vec::new())));
        let p5 = Formula::Predicate("p".to_owned(), vec!(Term::Function("-123.456E789".to_owned(), Vec::new())));
        let p6 = Formula::Predicate("p".to_owned(), vec!(Term::Function("123.456E-789".to_owned(), Vec::new())));
        let p7 = Formula::Predicate("p".to_owned(), vec!(Term::Function("-123.456E-789".to_owned(), Vec::new())));
        let fm = Formula::Or(Box::new(Formula::Or(Box::new(Formula::Or(Box::new(Formula::Or(Box::new(Formula::Or(Box::new(Formula::Or(Box::new(p1), Box::new(p2))), Box::new(p3))), Box::new(p4))), Box::new(p5))), Box::new(p6))), Box::new(p7));
        
        assert_eq!(res, ("reals".to_owned(), "axiom".to_owned(), fm, None));
    }
    
    #[test]
    fn parse_include_test() {
        assert_eq!(parse_include("include('Axioms/SYN000-0.ax').").unwrap(), ("Axioms/SYN000-0.ax".to_owned(), None));
        assert_eq!(parse_include("include('Axioms/SYN000+0.ax',[ia1,ia3]).").unwrap(), ("Axioms/SYN000+0.ax".to_owned(), Some(vec!("ia1".to_owned(), "ia3".to_owned()))));
    }

    #[test]
    fn parse_single_quoted_test() {
        assert_eq!(parse_single_quoted("'This is a single quoted string'").unwrap(), "This is a single quoted string");
        assert_eq!(parse_single_quoted("'A \\'quoted \\ escape\\''").unwrap(), "A \\'quoted \\ escape\\'");
        assert_eq!(parse_single_quoted("'The character \\' is quoted'").unwrap(), "The character \\' is quoted");
    }
    
    #[test]
    fn parse_distinct_object_test() {
        assert_eq!(parse_distinct_object("\"A \\\"Microsoft \\ escape\\\"\"").unwrap(), "A \\\"Microsoft \\ escape\\\"");
    }
    
    #[test]
    fn dollar_word_test() {
        assert_eq!(parse_dollar_word("$aWord").unwrap(), "$aWord");
        assert!(parse_dollar_word("$ aWord").is_err());
    }
}