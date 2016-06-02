// Serkr - An automated theorem prover. Copyright (C) 2015-2016 Mikko Aarnos.
//
// Serkr is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Serkr is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Serkr. If not, see <http://www.gnu.org/licenses/>.
//

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
#![cfg_attr(feature="clippy", allow(indexing_slicing, similar_names, 
                                    many_single_char_names, doc_markdown))]

#[macro_use]
extern crate clap;
extern crate num;

#[macro_use]
pub mod utils;
pub mod tptp_parser;
pub mod cnf;
pub mod prover;

use prover::proof_result::ProofResult;
use prover::proof_statistics::ProofStatistics;

#[cfg_attr(feature="clippy", allow(print_stdout))]
fn print_proof_result(proof_result: &ProofResult, input_file: &str) {
    println_szs!("SZS status {} for {}",
                 proof_result.display_type(),
                 input_file);
    if proof_result.is_successful() {
        println_szs!("SZS output None for {} : Proof output is not yet supported",
                     input_file);
    }
    println!("");
}

#[cfg_attr(feature="clippy", allow(print_stdout))]
fn print_statistics(proof_statistics: &ProofStatistics) {
    println_szs!("Time elapsed (in ms): {}", proof_statistics.elapsed_ms);

    println_szs!("Initial clauses: {}", proof_statistics.initial_clauses);
    println_szs!("Analyzed clauses: {}", proof_statistics.iterations);
    println_szs!("  Trivial: {}", proof_statistics.trivial_count);
    println_szs!("  Forward subsumed: {}", proof_statistics.fs_count);
    println_szs!("  Nonredundant: {}",
                 proof_statistics.nonredudant_processed_count());

    println_szs!("Backward subsumptions: {}", proof_statistics.bs_count);

    println_szs!("Inferred clauses: {}",
                 proof_statistics.inferred_clauses_count());
    println_szs!("  Superposition: {}", proof_statistics.sp_count);
    println_szs!("  Equality factoring: {}", proof_statistics.ef_count);
    println_szs!("  Equality resolution: {}", proof_statistics.er_count);
    println_szs!("Nontrivial inferred clauses: {}",
                 proof_statistics.nontrivial_inferred_clauses_count());
}

fn main() {
    let matches = clap::App::new("Serkr")
                      .version(crate_version!())
                      .author("Mikko Aarnos <mikko.aarnos@gmail.com>")
                      .about("An automated theorem prover for first order logic with equality")
                      .args_from_usage("<INPUT> 'The TPTP file the program should analyze'")
                      .arg(clap::Arg::with_name("time-limit")
                               .help("Time limit for the prover (default=300s)")
                               .short("t")
                               .long("time-limit")
                               .value_name("arg"))
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
                      .arg(clap::Arg::with_name("formula-renaming")
                               .help("Adjust the limit for renaming subformulae to avoid \
                                      exponential blowup in the CNF transformer. The default \
                                      (=32) seems to work pretty well. 0 disables formula \
                                      renaming.")
                               .long("formula-renaming")
                               .value_name("arg"))
                      .get_matches();

    // Hack to get around lifetime issues.
    let input_file_name = matches.value_of("INPUT")
                                 .expect("This should always be OK")
                                 .to_owned();
    // Hack to get around the parser/CNF transformer from crashing with very large files.
    let child = std::thread::Builder::new()
                    .stack_size(32 * 1024 * 1024)
                    .spawn(move || {
                        let input_file = matches.value_of("INPUT")
                                                .expect("This should always be OK");
                        let use_lpo = matches.is_present("lpo");
                        let time_limit = value_t!(matches, "time-limit", u64).unwrap_or(300);
                        let renaming_limit = value_t!(matches, "formula-renaming", u64)
                                                 .unwrap_or(32);
                        prover::proof_search::prove(input_file,
                                                    use_lpo,
                                                    time_limit,
                                                    renaming_limit)
                    })
                    .expect("Creating a new thread shouldn't fail");

    let (proof_result, proof_statistics) = child.join().expect("Joining a thread shouldn't fail");

    print_proof_result(&proof_result, &input_file_name);
    if !proof_result.is_err() {
        print_statistics(&proof_statistics);
    }
}
