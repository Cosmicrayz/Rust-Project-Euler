/* Prompt: By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?

Link: https://projecteuler.net/problem=7
 */

fn main() {
    // n^2 is a rough upper bound for the nth prime number
    let target: usize = 10001;
    println!(
        "The {} prime is {}",
        target,
        primes::filter_sieve(&target.pow(2))[target - 1]
    );
    // The 10001 prime is 104743
}
