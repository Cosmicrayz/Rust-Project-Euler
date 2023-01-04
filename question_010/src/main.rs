/* Prompt: The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.

Link: https://projecteuler.net/problem=10
 */
fn main() {
    let target = 2000000;
    println!(
        "The sum of all primes below {} is {}",
        target,
        &primes::filter_sieve(&target).iter().sum::<usize>(),
    );
}
