extern crate csv;

use std::io;

fn main() {
    // Read csv file from stdin
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_reader(io::stdin());

    let mut sum: u64 = 0;

    // Iterate over each row in csv
    for result in rdr.records() {
        match result {
            Ok(r) => {
                'outer: for x in r.iter() {
                    for y in r.iter() {
                        let x_int = x.parse::<u64>().unwrap();
                        let y_int = y.parse::<u64>().unwrap();

                        if x_int != y_int && x_int % y_int == 0 {
                            sum += x_int / y_int;
                            break 'outer;
                        }
                    }
                }
            }
            Err(err) => panic!("Invalid csv file! Error: {}", err),
        }
    }

    println!("Sum: {}", sum);
}
