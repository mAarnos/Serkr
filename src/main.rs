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

#[macro_use]
pub mod utils;

pub mod parser;
pub mod cnf;
pub mod prover;

fn main() {
    println!("Serkr 0.1.0, (C) 2015 Mikko Aarnos");
    
    let mut sw = utils::stopwatch::Stopwatch::new();
    sw.start();
    /*
    let res = prover::prove::prove("forall x. (P1(x) ==> P0(x)) /\\ exists x. P1(x) /\\ 
                                    forall x. (P2(x) ==> P0(x)) /\\ exists x. P2(x) /\\ 
                                    forall x. (P3(x) ==> P0(x)) /\\ exists x. P3(x) /\\
                                    forall x. (P4(x) ==> P0(x)) /\\ exists x. P4(x) /\\
                                    forall x. (P5(x) ==> P0(x)) /\\ exists x. P5(x) /\\ 
                                    exists x. Q1(x) /\\ forall x. (Q1(x) ==> Q0(x)) /\\
                                    forall x. (P0(x) ==> (forall y. (Q0(y) ==> R(x, y)) \\/ forall y. (P0(y) /\\ S(y, x) /\\ exists z. (Q0(z) /\\ R(y, z)) ==> R(x, y)))) /\\
                                    forall x. forall y. (P3(y) /\\ (P5(x) \\/ P4(x)) ==> S(x, y)) /\\ 
                                    forall x. forall y. (P3(x) /\\ P2(y) ==> S(x, y)) /\\ 
                                    forall x. forall y. (P2(x) /\\ P1(y) ==> S(x, y)) /\\
                                    forall x. forall y. (P1(x) /\\ (P2(y) \\/ Q1(y)) ==> ~R(x, y)) /\\ 
                                    forall x. forall y. (P3(x) /\\ P4(y) ==> R(x, y)) /\\ 
                                    forall x. forall y. (P3(x) /\\ P5(y) ==> ~R(x, y)) /\\
                                    forall x. (P4(x) \\/ P5(x) ==> exists y. (Q0(y) /\\ R(x, y)))
                                    ==> exists x. exists y. (P0(x) /\\ P0(y) /\\ exists z. (Q1(z) /\\ R(y, z) /\\ R(x, y)))");
    */
    let res = prover::prove::prove("forall x. forall y. forall z. f(f(x, y), z) = f(x, f(y, z)) /\\ 
                                    forall x. f(a(), x) = x /\\ 
                                    forall x. exists y. f(y, x) = a()
                                    ==> forall x. forall y. forall z. (f(x, y) = f(z, y) ==> x = z)");
    sw.stop();
    println!("{:?}", res);
    println!("Time elapsed (in ms): {}", sw.elapsed_ms());
}
