use std::io;

fn main() {
    let mut fib_number = String::new();
    println!("Enter a number of Fibonacci sequence: ");
    io::stdin()
        .read_line(&mut fib_number)
        .expect("Failed to read line!");

    let fib_number:u32 = fib_number.trim().parse().expect("Try to use only digits...");
    let result = fib(fib_number);
    println!("The {fib_number} of Fibonacci sequence is: {result}");
}

fn fib(n:u32) -> u128 {
    let mut a = 0;
    let mut b = 1;

    if n == 1 {
        return a;
    } else if n == 2 {
        return b;
    } else {
        for _i in 3..n+1 {
            let c = a + b;
            a = b;
            b = c;
        }
        return b;
    }
}
