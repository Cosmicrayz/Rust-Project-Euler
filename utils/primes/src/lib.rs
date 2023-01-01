use linked_hash_map::LinkedHashMap;

pub fn is_prime(num: usize) -> bool {
    if num == 0 || num == 1 {
        return false;
    }
    for i in 2..(num as f64).sqrt() as usize + 1 {
        if num % i == 0 {
            return false;
        }
    }
    return true;
}
pub fn get_primes(num: &usize) -> Vec<usize> {
    /* Gets the possilbe prime numbers in some range
     */
    let mut primes: Vec<usize> = Vec::new();
    for i in 0..*num {
        if is_prime(i) {
            primes.push(i as usize);
        }
    }
    primes
}

pub fn prime_factors(num: &usize) -> Vec<usize> {
    let all_primes: Vec<usize> = get_primes(num);
    let factors = all_primes
        .into_iter()
        .filter(|&x| num % x == 0)
        .collect::<Vec<_>>();
    factors
}

pub fn prime_sieve(num: &usize) -> LinkedHashMap<usize, bool> {
    let mut all_nums: LinkedHashMap<usize, bool> = (2..*num + 1).map(|x| (x, true)).collect();
    for key in 2..*num + 1 {
        if all_nums[&key] {
            for multiple in (2 * key..*num + 1).step_by(key) {
                all_nums[&multiple] = false;
            }
        }
    }
    all_nums
}

pub fn filter_sieve(num: &usize) -> Vec<usize> {
    let all_nums: LinkedHashMap<usize, bool> = prime_sieve(num);
    let primes_sieved: LinkedHashMap<usize, bool> =
        all_nums.into_iter().filter(|(_k, v)| *v).collect();
    let primes: Vec<usize> = primes_sieved.keys().cloned().collect();
    primes
}
