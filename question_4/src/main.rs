/*
Prompt: A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.

Link: https://projecteuler.net/problem=4
 */

fn is_palindrome(n: usize) -> bool {
    n.to_string() == n.to_string().chars().rev().collect::<String>()
}
fn main() {
    let mut max: usize = 0;
    for a in 100..1000 {
        for b in a..1000 {
            if is_palindrome(a * b) && a * b > max {
                max = a * b;
            }
        }
    }
    println!("The max is {}", max)
    // The max is 906609
}
