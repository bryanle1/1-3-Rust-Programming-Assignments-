// Assignment 2: Number Analyzer

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    // 1) Array of 10 integers
    let numbers: [i32; 10] = [12, 7, 15, 22, 30, 9, 5, 18, 11, 40];

    // 3) For loop: even/odd and Fizz/Buzz/FizzBuzz rules
    for n in numbers {
        // FizzBuzz checks first
        if n % 3 == 0 && n % 5 == 0 {
            println!("{} -> FizzBuzz", n);
        } else if n % 3 == 0 {
            println!("{} -> Fizz", n);
        } else if n % 5 == 0 {
            println!("{} -> Buzz", n);
        } else {
            // Otherwise even/odd
            if is_even(n) {
                println!("{} -> even", n);
            } else {
                println!("{} -> odd", n);
            }
        }
    }

    // 4) While loop: sum of all numbers
    let mut sum: i32 = 0;
    let mut idx: usize = 0;
    while idx < numbers.len() {
        sum = sum + numbers[idx];
        idx = idx + 1;
    }
    println!("Sum = {}", sum);

    // 5) loop: find the largest number
    let mut largest: i32 = numbers[0];
    let mut j: usize = 1;

    loop {
        if j >= numbers.len() {
            break;
        }

        if numbers[j] > largest {
            largest = numbers[j];
        }

        j = j + 1;
    }

    println!("Largest = {}", largest);
}
