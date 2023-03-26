use std::io;

fn main() {
    println!("Please enter a number you want to calculate its factorial:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap();

    let factorial = mini9::factorial(n);

    println!("The factorial of {} is {}", n, factorial);
}
