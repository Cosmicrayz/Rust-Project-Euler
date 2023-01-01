fn main() {
    println!(
        "{:?}",
        //primes::filter_sieve(&(10851475143_f64.sqrt() as usize + 1))
        primes::filter_sieve(&(100_f64.sqrt() as usize + 1))
    );
    println!("{:?}", primes::prime_sieve(&10));
}
