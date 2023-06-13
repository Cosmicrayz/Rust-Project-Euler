/* Prompt: A perfect number is a number for which the sum of its proper divisors is exactly equal to the number. For example, the sum of the proper divisors of 28
 would be 1+2+4+7+14 = 28, which means that  28 is a perfect number.

A number n is called deficient if the sum of its proper divisors is less than n and it is called abundant if this sum exceeds n.

As 12 is the smallest abundant number, the smallest number that can be written as the sum of two abundant numbers is 24. By mathematical analysis, it can be shown that all integers greater than 28123 can be written as the sum of two abundant numbers. However, this upper limit cannot be reduced any further by analysis even though it is known that the greatest number that cannot be expressed as the sum of two abundant numbers is less than this limit.

Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.

Link: https://projecteuler.net/problem=23
*/
use primes::get_divisor_pairs;
use std::collections::HashSet;

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

fn is_abundant(num: &usize) -> bool {
    divisor_sum(num) > *num
}

fn main() {
    let target: isize = 28123;
    let abundant_nums: Vec<isize> = (1..target + 1_isize)
        .into_iter()
        .filter(|num| is_abundant(&(*num as usize)))
        .collect::<Vec<isize>>();
    let mut abundant_sums: HashSet<isize> = HashSet::new();
    for ab_num_1 in &abundant_nums {
        for ab_num_2 in &abundant_nums {
            if ab_num_1 + ab_num_2 <= target {
                abundant_sums.insert(ab_num_1 + ab_num_2);
            }
        }
    }
    let non_abundant_sum: isize =
        (1..target + 1).sum::<isize>() - abundant_sums.iter().sum::<isize>();
    println!("The sum is {}", non_abundant_sum);
    // The sum is 4179871
}
