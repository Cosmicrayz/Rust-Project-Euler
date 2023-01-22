/* Prompt: The following iterative sequence is defined for the set of positive integers:

n → n/2 (n is even)
n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate the following sequence:

13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

Which starting number, under one million, produces the longest chain?

NOTE: Once the chain starts the terms are allowed to go above one million.

Link: https://projecteuler.net/problem=14
*/
use linked_hash_map::LinkedHashMap;

fn collatz_iter(num: usize) -> usize {
    if num % 2 == 0 {
        return num / 2;
    }
    3 * num + 1
}

fn main() {
    let mut nums_map: LinkedHashMap<usize, usize> = LinkedHashMap::new();
    for num in 2..1000000 {
        let mut count = 0;
        let mut start = num;
        while start != 1 {
            start = collatz_iter(start);
            match nums_map.get(&start) {
                Some(n) => {
                    start = 1;
                    count += n + 1;
                }
                None => count += 1,
            }
        }
        nums_map.insert(num, count);
    }
    println!(
        "The number with the longest seqeunce and associated sequence length is {:?}",
        nums_map
            .iter()
            .map(|x| (*x.0, *x.1 + 1))
            .max_by_key(|&(_k, v)| v)
            .unwrap()
    );
    // The number with the longest seqeunce and associated sequence length is (837799, 525)
}
