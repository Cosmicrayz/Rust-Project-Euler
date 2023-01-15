fn main() {
    println!(
        "{:?}",
        //primes::filter_sieve(&(10851475143_f64.sqrt() as usize + 1))
        primes::factorise(&9)
    );
    //println!("{:?}", primes::prime_sieve(&10));
}
