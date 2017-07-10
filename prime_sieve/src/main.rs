// Simple prime sieve algorithm as my first finger-practice in Rust.
// Tobias Baldauf, mail@tobias.is, @tbaldauf
// 20170709

// Use process to be able to exit the program at defined positions with an appropriate exit code
use std::process;

// Use environment variable support so that we can take CLI parameters into account
use std::env;

fn main() {

	// Ingest all CLI arguments in a vector for processing 
    let cli_arguments: Vec<String> = env::args().collect();

    // Since [0] is the program call itself, positions [1] and [2] contain our lower + upper bounds to search for primes
    //  Lets define our lower bound by converting the string into an 32bit integer
    let search_range_min = cli_arguments[1].parse::<i32>().unwrap();

    // Sanity check: since primes are natural numbers and 1 is not a prime by definition,
    // lets make sure that the lower bound is > 1.
    if search_range_min <= 1 {
    	println!("Primes are natural numbers > 1. Please choose a valid lower search range limit.");
        process::exit(1);
    }

    // Lets also define our upper bound by converting the string into an 32bit integer
    // Workaround: adding a +1 to the upper bound because inclusive ranges in Rust are still experimental
	let search_range_max = cli_arguments[2].parse::<i32>().unwrap()+1;

    // Same sanity check as before: our upper bound must not be smaller than 2 
    if search_range_max < 2 {
    	println!("Primes are natural numbers >= 2. Please choose a valid upper search range limit.");
        process::exit(1);
    }

    // Fill a vector with all numbers that exist in between the defined lower and upper bounds for processing
	let mut prime_candidates: Vec<i32> = (search_range_min..search_range_max).collect();

	// Set up our divisor range by which each prime candidate within the search range will be divided
	// We only need to divide from 2 until the half of the candidate range because no divisor larger than half of the candidate range can have a result without remainder 
	let divisors_max: i32 = search_range_max/2;

	// Let's populate a vector with all numbers between 2 and the half of the search range to iterate upon
	let divisors: Vec<i32> = (2..divisors_max).collect();

	// This is the heart of the prime sieve:
	// Instead of trying to check for primes, we're checking for non-primes and throw them out. This is far easier to implement. 
 	for current_divisor in divisors { // By having an outer for loop of our divisors starting at 2, we have fewer divisions to run
	 	for current_prime_candidate in search_range_min..search_range_max {
	 		if current_divisor <= current_prime_candidate/2 { // Same optimization as for divisors: save unnecessary divsions for each candidate
	 			// Use Rust's remainder functionality to check if a candidate can cleanly be divided
	 			// Needs a '&' as a prefix because of Rust's variable scope / ownership model: http://xion.io/post/code/rust-for-loop.html 
	 			if current_prime_candidate % &current_divisor == 0 { // If there's no remainder, it's not a prime and will be deleted from the vector 
			    	prime_candidates.retain(|&x| x != current_prime_candidate); // delete the currently identified non-prime from the vector
			    	continue; // skip the rest of the iteration
			    }
	 		}
		}
	}

	// Print the primes:
	println!("{:?}", &prime_candidates);

}
