/* Prompt: 215 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

What is the sum of the digits of the number 21000?

Link: https://projecteuler.net/problem=16
 */

use big_nums::BigNum;

fn main() {
    let base = 2;
    let n = 1000;
    let mut res = BigNum { digits: vec![1] };
    for _i in 0..n {
        if _i == 15 {
            println!("hi")
        }
        res = res * BigNum { digits: vec![base] };
    }
    println!("{:?}", (res).digits.iter().sum::<usize>());
    // The sum of the digits of 2^100 is 1366
}
