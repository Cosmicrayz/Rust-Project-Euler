/*
Prompt: The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?

Link: https://projecteuler.net/problem=3
 */
use primes;

fn main() {
    let target_num: u128 = 600851475143;
    let factors = primes::filter_sieve(&((target_num as f64).sqrt() as usize + 1));
    let biggest_factor = factors
        .iter()
        .filter(|&x| target_num % (*x as u128) == 0)
        .max();
    match biggest_factor {
        Some(factor) => println!("The largest prime factor of {} is {}", target_num, factor),
        None => println!(""),
    };
    // The largest prime factor of 600851475143 is 6857
}
