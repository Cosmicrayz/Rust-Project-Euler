/*
Prompt: If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
Find the sum of all the multiples of 3 or 5 below 1000.

Link: https://projecteuler.net/problem=1
 */

fn main() {
    let sum: usize = (1..1000)
        .filter(|&num| (num % 3 == 0) || (num % 5 == 0))
        .sum();
    println!("The sum is {}", sum);
    // The sum is 233168
}
