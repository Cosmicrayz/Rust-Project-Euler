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

pub fn factorise(num: &usize) -> LinkedHashMap<usize, usize> {
    // Only search till sqrt(num)
    let primes: Vec<usize> = filter_sieve(&((*num as f64).sqrt() as usize));
    let mut total = *num;
    let mut output: LinkedHashMap<usize, usize> = LinkedHashMap::<usize, usize>::new();
    for p in primes.iter() {
        while total % p == 0 {
            total = total / p;
            *(output.entry(*p).or_insert(0)) += 1
        }
    }
    // Returns factorisation if it exists otherwise an empty hashmap
    return output;
}

pub fn get_divisor_pairs(num: &usize) -> Vec<(usize, usize)> {
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    (1..(*num as f64).sqrt() as usize + 1)
        .into_iter()
        .for_each(|i| {
            if num % i == 0 {
                pairs.push((i, num / i))
            }
        });
    if pairs.len() == 0 {
        pairs.push((1, *num))
    }
    pairs
}
