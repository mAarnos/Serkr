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
use parser::tokenizer::*;
use parser::formula::{Term, Formula};

/// Get the precedence of a token representing a connective.
fn get_precedence(token: Token) -> isize {
    match token.get_type() {
        TokenType::Not => 7,
        TokenType::And => 6,
        TokenType::Or => 5,
        TokenType::Implies => 4,
        TokenType::Equivalent => 3,
        TokenType::Forall => 2,
        TokenType::Exists => 1,
        _ => 0,
    }
}

/// Returns the next token, if it exists.
fn lookahead(tokens: &Vec<Token>) -> Token { 
    if tokens.len() > 0 {
        tokens[0].clone()
    } else {
        Token::new(TokenType::Invalid, "".to_string())
    }
}

/// Returns and deletes the next token, if it exists.
fn consume(tokens: &mut Vec<Token>) -> Token
{ 
    if tokens.len() > 0 {
        let ret = tokens[0].clone();
        tokens.remove(0);
        ret
    } else {
        Token::new(TokenType::Invalid, "".to_string())
    }
}

fn parse_binary_connective(tokens: &mut Vec<Token>, current_token: Token, lhs: Formula) -> Result<Formula,  &'static str> {
    let rhs = try!(parse(tokens, get_precedence(current_token.clone())));
    match current_token.get_type() {
        TokenType::And => Ok(Formula::And(box lhs, box rhs)),
        TokenType::Or => Ok(Formula::Or(box lhs, box rhs)),
        TokenType::Implies => Ok(Formula::Implies(box lhs, box rhs)),
        TokenType::Equivalent => Ok(Formula::Equivalent(box lhs, box rhs)),
        _ => Err("Expected binary connective")
    }
}

fn parse_parenthesised_formula(tokens: &mut Vec<Token>) -> Result<Formula,  &'static str> {
    let formula = try!(parse(tokens, 0));
    if lookahead(tokens).get_type() != TokenType::RightParenthesis {
        Err("Expected a closing parenthesis")
    } else {
        consume(tokens);
        Ok(formula)
    }    
}

fn parse_not_formula(tokens: &mut Vec<Token>) -> Result<Formula,  &'static str> {
    Ok(Formula::Not(box try!(parse(tokens, 7))))  
}

fn parse_predicate(_tokens: &mut Vec<Token>, _current_token: Token) -> Result<Formula,  &'static str> {
    panic!("unimplemented");
}

fn parse_quantifier(_tokens: &mut Vec<Token>, _current_token: Token) -> Result<Formula,  &'static str> {
    panic!("unimplemented");
}

fn parse_everything_else(tokens: &mut Vec<Token>, current_token: Token) -> Result<Formula,  &'static str> {
    match current_token.get_type() {
        TokenType::LeftParenthesis => parse_parenthesised_formula(tokens),
        TokenType::Not => parse_not_formula(tokens),
        TokenType::Name => parse_predicate(tokens, current_token),
        TokenType::Forall | TokenType::Exists => parse_quantifier(tokens, current_token),
        _ => Err("???")
    }
}

fn parse_term_list(tokens: &mut Vec<Token>) -> Result<Vec<Term>,  &'static str> {
    let mut term_list = Vec::<Term>::new();
    loop {
        if lookahead(tokens).get_type() != TokenType::Name {
            return Ok(term_list);
        } 
            
        let term_name = consume(tokens).get_name();
        if term_name.chars().next().unwrap().is_lowercase() {
            return Err("Expected a term, not a predicate");
        }
        
        let new_term = try!(parse_term(tokens, term_name));
        term_list.push(new_term);
                
        // Make sure that there is a comma separating arguments.
        if lookahead(tokens).get_type() != TokenType::Comma {
            return Err("Comma separating terms is missing");
        }
        consume(tokens);
    }
}

fn parse_term(tokens: &mut Vec<Token>, term_name: String) -> Result<Term,  &'static str> {
    if lookahead(tokens).get_type() == TokenType::LeftParenthesis { 
        consume(tokens);
        let subterms = try!(parse_term_list(tokens));
        if lookahead(tokens).get_type() != TokenType::RightParenthesis {
            Err("Expected a closing parenthesis")
        } else {
            consume(tokens);
            Ok(Term::Function(term_name, subterms))
        }
    } else {
        Ok(Term::Variable(term_name))
    }
}

fn parse(tokens: &mut Vec<Token>, precedence: isize) -> Result<Formula,  &'static str> {
    let mut current_token = consume(tokens);
    let mut current_formula = try!(parse_everything_else(tokens, current_token));

    while precedence < get_precedence(lookahead(tokens)) {
        current_token = consume(tokens);
        current_formula = try!(parse_binary_connective(tokens, current_token, current_formula));
    }
    
    Ok(current_formula)
}

fn parse_string_to_formula(s: &str) -> Result<Formula,  &'static str> {
    let mut tokens = try!(tokenize(s));
    let formula = try!(parse(&mut tokens, 0));
    if !tokens.is_empty() {
        Err("Could not parse all input")
    } else {
        Ok(formula)
    }
}

#[cfg(test)]
mod test {

}