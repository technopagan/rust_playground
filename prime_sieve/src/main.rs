// Simple prime sieve algorithm as my first finger-practice in Rust.
// Tobias Baldauf, mail@tobias.is, @tbaldauf
// 20170709

#[macro_use]
extern crate clap;

#[macro_use]
extern crate error_chain;

mod errors;
mod detectors;

use clap::{Arg, App};
use errors::*;

use detectors::sieve::prime_range as primes;


fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {

    let matches = App::new("Prime sieve")
        .arg(
            Arg::with_name("min")
                .help("Sets the minimum of the search range")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("max")
                .help("Sets the maximum of the search range")
                .required(true)
                .index(2),
        )
        .get_matches();

    let search_range_min = value_t!(matches.value_of("min"), usize).expect("Min not provided");
    let search_range_max = value_t!(matches.value_of("max"), usize).expect("Max not provided");


    // Sanity check: since primes are natural numbers and 1 is not a prime by definition,
    // lets make sure that the lower bound is > 1.
    if search_range_min <= 1 {
        return Err(
            "Primes are natural numbers > 1. Please choose a valid lower search range limit."
                .into(),
        );
    }

    // Same sanity check as before: our upper bound must not be smaller than 2
    if search_range_max < 2 {
        return Err(
            "Primes are natural numbers >= 2. Please choose a valid upper search range limit."
                .into(),
        );
    }

    println!("{:?}", &primes(search_range_min, search_range_max));
    Ok(())
}
