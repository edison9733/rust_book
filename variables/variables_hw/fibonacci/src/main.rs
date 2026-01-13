use std::io;

fn main() {
    println!("Fibonacci Number Generator");
    println!("==========================");
    println!("Enter n to get the nth Fibonacci number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let n: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid positive integer!");
            return;
        }
    };

    if n == 0 {
        println!("Please enter a number greater than 0!");
        return;
    }

    let result = fibonacci(n);
    println!("The {}th Fibonacci number is: {}", n, result);

    // Also print the sequence up to n
    println!("\nFibonacci sequence up to position {}:", n);
    for i in 1..=n {
        print!("{}", fibonacci(i));
        if i < n {
            print!(", ");
        }
    }
    println!();
}

// Iterative approach (more efficient)
fn fibonacci(n: u32) -> u64 {
    if n == 1 {
        return 0;
    }
    if n == 2 {
        return 1;
    }

    let mut prev = 0u64;
    let mut curr = 1u64;

    for _ in 3..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}

// Alternative: Recursive approach (less efficient but demonstrates recursion)
#[allow(dead_code)]
fn fibonacci_recursive(n: u32) -> u64 {
    match n {
        1 => 0,
        2 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}