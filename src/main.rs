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

//! Serkr is an automated theorem prover for first order logic.

// Some lints which are pretty useful.
#![deny(fat_ptr_transmutes,
        unused_extern_crates,
        variant_size_differences,
        missing_docs,
        missing_debug_implementations, 
        missing_copy_implementations,
        trivial_casts, 
        trivial_numeric_casts,
        unsafe_code,
        unused_import_braces,
        unused_qualifications)]
// Might as well make the warnings errors.
#![deny(warnings)]

// Clippy lints turned to the max.
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", deny(clippy))]
#![cfg_attr(feature="clippy", deny(clippy_pedantic))]
#![cfg_attr(feature="clippy", allow(result_unwrap_used))]

extern crate regex;

#[macro_use]
pub mod utils;
pub mod parser;
pub mod cnf;
pub mod prover;

use std::env;
use parser::tptp::parser::parse_tptp_file;

fn main() {
    println!("Serkr 0.1.0, (C) 2015 Mikko Aarnos");
    
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    
    let parsed_file = parse_tptp_file("examples/RNG009-7.p");
    for input in &parsed_file {
        println!("{:?}\n", input);
    }
  
    /*
    let mut sw = utils::stopwatch::Stopwatch::new();
    sw.start();
    sw.stop();
    println!("{:?}", res);
    println!("Time elapsed (in ms): {}", sw.elapsed_ms());
    */
}
