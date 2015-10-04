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

#![allow(dead_code)]
#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(plugin)]
#![plugin(peg_syntax_ext)]

mod parser;
mod cnf;

fn main() {
    println!("Serkr 0.1.0, (C) 2015 Mikko Aarnos");
    
    println!("{}", parser::grammar::parse("  P(x, y)  ").unwrap_err());
}
