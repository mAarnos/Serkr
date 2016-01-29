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
    
    #[test]
    fn parse_include_test() {
        assert_eq!(parse_include("include('Axioms/SYN000-0.ax').").unwrap(), ("Axioms/SYN000-0.ax".to_owned(), None));
        assert_eq!(parse_include("include('Axioms/SYN000+0.ax',[ia1,ia3]).").unwrap(), ("Axioms/SYN000+0.ax".to_owned(), Some(vec!("ia1".to_owned(), "ia3".to_owned()))));
    }

    #[test]
    fn parse_single_quoted_test() {
        assert_eq!(parse_single_quoted("'This is a single quoted string'").unwrap(), "This is a single quoted string");
        assert_eq!(parse_single_quoted("'The character \\' is quoted'").unwrap(), "The character \\' is quoted");
    }
    
    #[test]
    fn dollar_word_test() {
        assert_eq!(parse_dollar_word("$aWord").unwrap(), "aWord");
        assert!(parse_dollar_word("$ aWord").is_err());
    }
}