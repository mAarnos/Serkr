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

#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(hashmap_hasher)]
#![feature(plugin)]
#![plugin(peg_syntax_ext)]
#![plugin(clippy)]

// Some lints which are pretty useful.
// Try to add "unstable_features" when possible.
#![deny(missing_docs,
        missing_debug_implementations, 
        missing_copy_implementations,
        trivial_casts, 
        trivial_numeric_casts,
        unsafe_code,
        unused_import_braces,
        unused_qualifications)]
#![deny(clippy)]

// Remove these two whenever possible.
#![allow(len_zero)]  
#![allow(dead_code)]

pub mod parser;
pub mod cnf;
pub mod prover_full;
pub mod utils;

fn main() {
    println!("Serkr 0.1.0, (C) 2015 Mikko Aarnos");
    
    /*
    let mut sw = utils::stopwatch::Stopwatch::new();
    sw.start();
    // ((exists y. forall x. (F(x, y) <=> F(x, x))) ==> ~forall x. exists y. forall z. (F(x, y) <=> ~F(z, x)))
    let res = prover::resolution::resolution("(((exists x. forall y. (P(x) <=> P(y))) <=> 
                                                exists x. (Q(x) <=> forall y. Q(y))) <=> 
                                                 ((exists x. forall y. (Q(x) <=> Q(y))) <=>
                                                  (exists x. P(x) <=> forall y. P(y))))");
    sw.stop();
    println!("{:?}", res);
    println!("Time elapsed (in ms): {}", sw.elapsed_ms());
    */
}
