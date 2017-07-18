 // Todo:
// * Ingest CLI parameters for the two numbers and their upper bound using Clap

pub mod functions;

pub fn main() {
    let total_sum = functions::findsum();
    println!("{:?}", &total_sum);
}

