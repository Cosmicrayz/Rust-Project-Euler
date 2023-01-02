/* Prompt: The sum of the squares of the first ten natural numbers is 385,

The square of the sum of the first ten natural numbers is 3025,

Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 2640.

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

Link: https://projecteuler.net/problem=6
 */

fn main() {
    let range = 1..101;
    let diff: usize =
        range.clone().sum::<usize>().pow(2) - range.clone().map(|x: usize| x.pow(2)).sum::<usize>();
    println!("The difference is {}", diff);
}
