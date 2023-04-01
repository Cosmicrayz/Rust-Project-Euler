/* Prompt: n! means n × (n − 1) × ... × 3 × 2 × 1

For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

Find the sum of the digits in the number 100!

Link: https://projecteuler.net/problem=20
 */

use big_nums::BigNum;

fn main() {
    let mut start_num = BigNum { digits: vec![1] };
    let target = 100;
    for i in 1..target + 1 {
        start_num = start_num * BigNum { digits: vec![i] }
    }
    println!(
        "The sum of digits for {}! {}",
        target,
        start_num.digits.iter().sum::<usize>()
    );
}
