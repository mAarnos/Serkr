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
    let res = prover::prove::prove_general("forall x. add(e(), x) = x /\\ 
                                            forall x. add(inv(x), x) = e() /\\
                                            forall x. forall y. forall z. add(add(x, y), z) = add(x, add(y, z)) /\\
                                            forall x. forall y. add(x, y) = add(y, x) /\\
                                            forall x. forall y. forall z. mult(mult(x, y), z) = mult(x, mult(y, z)) /\\
                                            forall x. forall y. forall z. mult(x, add(y, z)) = add(mult(x, y), mult(x, z)) /\\
                                            forall x. forall y. forall z. mult(add(x, y), z) = add(mult(x, z), mult(y, z)) /\\
                                            forall x. mult(mult(x, x), x) = x 
                                            ==> forall x. forall y. mult(x, y) = mult(y, x)", false, true, 300);
    */ 
    /*
    let res = prover::prove::prove_general("forall a. forall b. exists x. exists y. (mult(a, x) = b /\\ mult(y, a) = b /\\ 
                                                                                     forall x2. forall y2. ((mult(a, x2) = b /\\ mult(y2, a) = b) ==> x = x2 /\\ y = y2)) /\\
                                            forall x. forall y. forall z. mult(mult(z, x), mult(y, z)) = mult(z, mult(mult(x, y), z))
                                            ==> exists e. forall a. (mult(e, a) = a /\\ mult(a, e) = a)", false, true, 300);
    */  
    /*
    let res = prover::prove::prove_general("forall a. forall b. forall x. (F(x, intersection(a, b)) <=> F(x, a) /\\ F(x, b)) /\\
                                            forall a. forall b. (forall x. (F(x, a) <=> F(x, b)) ==> a = b)
                                            ==> forall a. forall b. intersection(a, b) = intersection(b, a)", false, true, 60); 
    */                                        
    let res = prover::prove::prove_general("forall x. add(e(), x) = x /\\ 
                                            forall x. add(inv(x), x) = e() /\\
                                            forall x. forall y. forall z. add(add(x, y), z) = add(x, add(y, z)) /\\
                                            forall x. forall y. add(x, y) = add(y, x) /\\
                                            forall x. forall y. forall z. mult(mult(x, y), z) = mult(x, mult(y, z)) /\\
                                            forall x. forall y. forall z. mult(x, add(y, z)) = add(mult(x, y), mult(x, z)) /\\
                                            forall x. forall y. forall z. mult(add(x, y), z) = add(mult(x, z), mult(y, z)) /\\
                                            forall x. mult(e2(), x) = x 
                                            ==> forall x. forall y. mult(x, y) = mult(inv(x), inv(y))", false, true, 30);       
   
    sw.stop();
    println!("{:?}", res);
    println!("Time elapsed (in ms): {}", sw.elapsed_ms());
}
