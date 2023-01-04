/* Prompt: A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

a^2 + b^2 = c^2
For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.

Link: https://projecteuler.net/problem=9
 */

fn main() {
    let target = 1000;
    for a in 1..(target / 3 as usize) {
        for b in a..(target / 2 as usize) {
            let c = target - a - b;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                println!(
                    "The unique triple is {}, {}, {} with product {}",
                    a,
                    b,
                    c,
                    a * b * c
                );
                break;
            }
        }
    }
}
