/*
    Serkr - An automated theorem prover. Copyright (C) 2015-2016 Mikko Aarnos.

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
extern crate clap;

#[macro_use]
pub mod utils;
pub mod parser;
pub mod cnf;
pub mod prover;

fn main() {
    let matches = clap::App::new("Serkr")
                                .version(crate_version!())
                                .author("Mikko Aarnos <mikko.aarnos@gmail.com>")
                                .about("An automated theorem prover for first order logic with equality")
                                .args_from_usage("<INPUT> 'The TPTP file the program should analyze'
                                                  -t, --time_limit=[time_limit] 'The time limit for the prover (default=300s)'")
                                .arg(clap::Arg::with_name("lpo")
                                    .help("Use LPO as the term ordering") 
                                    .short("l") 
                                    .long("lpo") 
                                    .conflicts_with("kbo"))
                                .arg(clap::Arg::with_name("kbo")
                                    .help("Use KBO as the term ordering (default)") 
                                    .short("k") 
                                    .long("kbo") 
                                    .conflicts_with("lpo"))
                                .get_matches();
                                
    println!("Serkr {}, (C) 2015-2016 Mikko Aarnos", crate_version!());
                                 
    let input_file = matches.value_of("INPUT").unwrap();
    let use_lpo = matches.is_present("lpo");
    let time_limit = value_t!(matches, "time_limit", u64).unwrap_or(300);
    let (proof_result, proof_statistics) = prover::prove::prove_tptp(&input_file, use_lpo, time_limit);

    println!("{:?}", proof_result);
    println!("Time elapsed (in ms): {}", proof_statistics.elapsed_ms);
}
