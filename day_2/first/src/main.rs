extern crate csv;

use std::io;
use std::cmp;

fn main() {
    // Read csv file from stdin
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_reader(io::stdin());

    let mut bigly: u64 = std::u64::MIN;
    let mut smaly: u64 = std::u64::MAX;

    let mut sum: u64 = 0;

    // Iterate over each row in csv
    for result in rdr.records() {
        match result {
            Ok(r) => {
                for val in r.iter() {
                    bigly = cmp::max(bigly, val.parse::<u64>().unwrap());
                    smaly = cmp::min(smaly, val.parse::<u64>().unwrap());
                }
                sum += bigly - smaly;
            }
            Err(err) => panic!("Invalid csv file! Error: {}", err),
        }

        // Reset tmp vars
        bigly = std::u64::MIN;
        smaly = std::u64::MAX;
    }

    println!("Sum: {}", sum);
}
