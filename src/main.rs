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

// Some lints which are pretty useful.
// Try to add "unstable_features" when possible.
// We allow dead code due to it warning about things too much.
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
#![deny(warnings)]
#![allow(dead_code)]

// Clippy lints.
// We allow len_zero due to rust-peg.
// TODO: add clippy_pedantic 
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", deny(clippy))]
#![cfg_attr(feature="clippy", allow(len_zero))]

#[macro_use]
pub mod utils;

pub mod parser;
pub mod cnf;
pub mod prover;

fn main() {
    println!("Serkr 0.1.0, (C) 2015 Mikko Aarnos");
    
    let mut sw = utils::stopwatch::Stopwatch::new();
    sw.start();
    let res = prover::prove::prove("((forall y. exists z. forall x. (F(x, z) <=> x = y)) 
                              ==> ~exists w. forall x. (F(x, w) <=> forall u. (F(x, u) ==> 
                                  exists y. (F(y, u) /\\ ~exists z. (F(z, u) /\\ F(z, y))))))");
    sw.stop();
    println!("{:?}", res);
    println!("Time elapsed (in ms): {}", sw.elapsed_ms());
}
