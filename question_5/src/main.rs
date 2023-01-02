/* Prompt: 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

Link: https://projecteuler.net/problem=5
 */

fn get_highest_power(num: usize, target: usize) -> usize {
    let mut count = 0;
    let mut current_num: usize = num;
    while current_num < target {
        current_num *= num;
        count += 1;
    }
    count
}

fn main() {
    let target = 20;
    let mut prod = 1;
    for prime in primes::filter_sieve(&target) {
        prod *= prime.pow(get_highest_power(prime, target) as u32);
    }
    println!(
        "The smallest number divisible by all numbers 1 to {} is {}",
        target, prod
    );
    // The smallest number divisible by all numbers 1 to 20 is 232792560
}
