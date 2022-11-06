use std::io;

fn main() {
    println!("What element of the Fibonacci sequence do you want?");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: i32 = n.trim().parse().expect("Please type a number!");

    println!("The {}th element of the Fibonacci sequence is {}", n, fibonacci_sequence(n));
}

fn fibonacci_sequence(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci_sequence(n - 1) + fibonacci_sequence(n - 2);
    }
}