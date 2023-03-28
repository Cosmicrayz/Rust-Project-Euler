/* Prompt: By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from top to bottom is 23.

3
7 4
2 4 6
8 5 9 3

That is, 3 + 7 + 4 + 9 = 23.

Find the maximum total from top to bottom of the triangle below:

75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23

NOTE: As there are only 16384 routes, it is possible to solve this problem by trying every route. However, Problem 67, is the same challenge with a triangle containing one-hundred rows; it cannot be solved by brute force, and requires a clever method! ;o)

Link: https://projecteuler.net/problem=18
 */
use std::cmp::max;

fn parse_input(number_pyramid: &str) -> Vec<Vec<usize>> {
    let mut pyramid: Vec<Vec<usize>> = Vec::new();
    // Go backwards along the pyramid
    for line in number_pyramid.split("\n") {
        let row: Vec<usize> = line
            .trim()
            .split(" ")
            .map(|x| (*x).parse().ok().expect("Found an error"))
            .collect();
        pyramid.push(row);
    }
    pyramid
}

fn get_max_path(mut pyramid: Vec<Vec<usize>>) -> usize {
    for i in (0..pyramid.len() - 1).rev() {
        for j in 0..pyramid[i].len() {
            pyramid[i][j] = pyramid[i][j] + max(pyramid[i + 1][j], pyramid[i + 1][j + 1])
        }
    }
    pyramid[0][0]
}

fn main() {
    let z = "75
    95 64
    17 47 82
    18 35 87 10
    20 04 82 47 65
    19 01 23 75 03 34
    88 02 77 73 07 63 67
    99 65 04 28 06 16 70 92
    41 41 26 56 83 40 80 70 33
    41 48 72 33 47 32 37 16 94 29
    53 71 44 65 25 43 91 52 97 51 14
    70 11 33 28 77 73 17 78 39 68 17 57
    91 71 52 38 17 14 91 43 58 50 27 29 48
    63 66 04 68 89 53 67 30 73 16 69 87 40 31
    04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";
    println!("{:?}", get_max_path(parse_input(z)));
}
