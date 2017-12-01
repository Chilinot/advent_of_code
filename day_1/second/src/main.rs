use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut sum = 0;

    if args.len() > 1 {
        let input = args[1].clone(); // needed to set lifetime of clone to this scope.
        let input_vec: Vec<char> = input.chars().collect();
        let look_ahead = input.len() / 2;

        // Iterate over input
        for (i, c) in input.chars().enumerate() {
            if c == input_vec[(i + look_ahead) % input_vec.len()] {
                sum += c.to_digit(10).unwrap();
            }
        }

        println!("Sum: {}", sum);
    }
    else {
        println!("You need to specify the input!");
    }
}
