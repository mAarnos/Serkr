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

use parser::token::{Token, TokenType};
use std::ascii::AsciiExt;

/// An enum for all the different tokenizer states there are.
enum TokenizerState {
    AlphaNumericCharacter,
    SpecialCharacter,
    Error
}

/// Tokenizes the input string and returns the generated tokens.
///
/// The time complexity of this function is O(n) where n is the length of input in characters.
/// The input string should only contain alphanumeric characters, ')', '(', ',', '., or whitespace.
pub fn tokenize(input: &str) -> Result<Vec<Token>, &'static str> {
    let mut tokens = Vec::<Token>::new();
    let mut s = String::new();

    for c in input.chars() {
        if !c.is_ascii() {
            return Err("Non-ASCII character encountered");
        }
        
        match new_state(c) {
            TokenizerState::AlphaNumericCharacter => s.push(c),
            TokenizerState::SpecialCharacter => {
                                                    if let Some(tok) = string_to_token(s.clone()) {
                                                        tokens.push(tok);
                                                    }
                                                    s.clear();
                                                    s.push(c);
                                                    if let Some(tok) = string_to_token(s.clone()) {
                                                        tokens.push(tok);
                                                    }
                                                    s.clear();
                                                },
            TokenizerState::Error => return Err("Invalid ASCII character encountered"),
        }
    }
   
    // Deals with the case when the end of the string is reached while there is still something in s.
    if let Some(tok) = string_to_token(s) {
        tokens.push(tok);
    }
    
    Ok(tokens)
}

/// Transitions into the next tokenizer state.
fn new_state(c: char) -> TokenizerState {
    if c.is_alphanumeric() {
        TokenizerState::AlphaNumericCharacter
    } else if c == ' ' || c == '(' || c == ')' || c == ',' || c == '.' {
        TokenizerState::SpecialCharacter
    } else {
        TokenizerState::Error
    }
}

/// Turns a string into a token.    
fn string_to_token(s: String) -> Option<Token> {
    match &*s {
        "(" => Some(Token::new(TokenType::LeftParenthesis, s)),
        ")" => Some(Token::new(TokenType::RightParenthesis, s)),
        "," => Some(Token::new(TokenType::Comma, s)),
        "." => Some(Token::new(TokenType::Period, s)),
        " " => None,
        "not" => Some(Token::new(TokenType::Not, s)),
        "and" => Some(Token::new(TokenType::And, s)),
        "or" => Some(Token::new(TokenType::Or, s)),
        "implies" => Some(Token::new(TokenType::Implies, s)),
        "equivalent" => Some(Token::new(TokenType::Equivalent, s)),
        "forall" => Some(Token::new(TokenType::Forall, s)),
        "exists" => Some(Token::new(TokenType::Exists, s)),
        _ => if s.is_empty() { 
                None 
             } else { 
                Some(Token::new(TokenType::Name, s)) 
             },
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use parser::token::{Token, TokenType};
    
    #[test]
    fn non_ascii() {
        let res = tokenize("Löwe(x)");
        assert!(res.is_err()); 
    }

    #[test]
    fn invalid_character() {
        let res = tokenize("P(x) implies ?(x)");
        assert!(res.is_err()); 
    }
    
    #[test]
    fn successful_tokenization() {
        let res = tokenize("forall x. Equals(x, x)");
        assert!(res.is_ok()); 
        assert_eq!(res.unwrap().len(), 9); 
    }
    
    #[test]
    fn successful_tokenization_2() {
        let res = tokenize("forall x. forall y. Equals(x, y) equivalent Equals(y, x)");
        assert!(res.is_ok()); 
        assert_eq!(res.unwrap().len(), 19); 
    }
    
    #[test]
    fn successful_tokenization_3() {
        let res = tokenize("forall x. forall y. forall z. P(x, y) and P(y, z) implies P(x, z)");
        assert!(res.is_ok()); 
        assert_eq!(res.unwrap().len(), 29); 
    }
    
    #[test]
    fn successful_tokenization_4() {
        let tokens = tokenize("forall x. forall y. Equals(x, y) implies Equals(f(x), f(y))").unwrap();
        let correct_tokens = vec!(Token::new(TokenType::Forall, "forall".to_string()),
                                  Token::new(TokenType::Name, "x".to_string()),
                                  Token::new(TokenType::Period, ".".to_string()),
                                  
                                  Token::new(TokenType::Forall, "forall".to_string()),
                                  Token::new(TokenType::Name, "y".to_string()),
                                  Token::new(TokenType::Period, ".".to_string()),
                                  
                                  Token::new(TokenType::Name, "Equals".to_string()),
                                  Token::new(TokenType::LeftParenthesis, "(".to_string()),
                                  Token::new(TokenType::Name, "x".to_string()),
                                  Token::new(TokenType::Comma, ",".to_string()),
                                  Token::new(TokenType::Name, "y".to_string()),
                                  Token::new(TokenType::RightParenthesis, ")".to_string()),
                                  
                                  Token::new(TokenType::Implies, "implies".to_string()),
                                  
                                  Token::new(TokenType::Name, "Equals".to_string()),
                                  Token::new(TokenType::LeftParenthesis, "(".to_string()),
                                  Token::new(TokenType::Name, "f".to_string()),
                                  Token::new(TokenType::LeftParenthesis, "(".to_string()),
                                  Token::new(TokenType::Name, "x".to_string()),
                                  Token::new(TokenType::RightParenthesis, ")".to_string()),
                                  Token::new(TokenType::Comma, ",".to_string()),
                                  Token::new(TokenType::Name, "f".to_string()),
                                  Token::new(TokenType::LeftParenthesis, "(".to_string()),
                                  Token::new(TokenType::Name, "y".to_string()),
                                  Token::new(TokenType::RightParenthesis, ")".to_string()),
                                  Token::new(TokenType::RightParenthesis, ")".to_string()),
                                  );
                                  
        assert_eq!(tokens.len(), correct_tokens.len());

        for i in 0..tokens.len() {
            assert_eq!(tokens[i], correct_tokens[i]);
        }
    }
}