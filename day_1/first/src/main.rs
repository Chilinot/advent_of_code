use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut sum = 0;

    if args.len() > 1 {
        let arg = args[1].clone(); // needed to set lifetime of clone to this scope.

        // First char in input
        let first = arg.chars().nth(0).unwrap();

        // Iterate over input
        let mut input_iter = arg.chars().peekable();
        loop {
            let current = input_iter.next();
            let next    = input_iter.peek();
            match current {
                Some(c) => {
                    match next {
                        Some(n) => {
                            if c == *n {
                                sum += c.to_digit(10).unwrap();
                            }
                        }
                        None => {
                            if c == first {
                                sum += c.to_digit(10).unwrap();
                            }
                        }
                    }
                }
                None => {
                    // Iterator is empty
                    break;
                }
            }
        }

        println!("Sum: {}", sum);
    }
    else {
        println!("You need to specify the input!");
    }
}
