/* Prompt: Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.

For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.

Evaluate the sum of all the amicable numbers under 10000.

Link: https://projecteuler.net/problem=21
*/
use primes::get_divisor_pairs;
use std::collections::HashMap;

fn divisor_sum_helper(pairs: Vec<(usize, usize)>) -> usize {
    let mut sum = 0;
    for pair in pairs.iter() {
        if pair.0 != pair.1 {
            sum += pair.0 + pair.1
        } else {
            sum += pair.0
        }
    }
    sum
}

fn divisor_sum(num: &usize) -> usize {
    let factor_pairs = get_divisor_pairs(num);
    divisor_sum_helper(factor_pairs) - *num
}

fn main() {
    let mut seen_nums: Vec<usize> = Vec::new();
    let target = 10000;
    let mut total = 0;
    let mut num_totals: HashMap<usize, usize> = HashMap::new();
    for num in 1..target + 1 {
        if seen_nums.contains(&num) {
            continue;
        }
        let ds = divisor_sum(&num);
        num_totals.insert(num, ds);
        if (ds != num) && (ds > 0) && (ds < target) {
            let reverse_ds = divisor_sum(&ds);
            seen_nums.push(ds);
            if reverse_ds == num {
                total += num + ds
            }
        }
    }
    println!(
        "The total divisor sum of all amicable numbers under {} is {}",
        target, total
    )
    // The total divisor sum of all amicable numbers under 10000 is 31626
}
