/* Prompt: The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.

Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?

Link: https://projecteuler.net/problem=8
 */

fn max_product(window_size: usize, vec: &Vec<usize>) -> usize {
    match window_size {
        0 => return 0,
        1 => return *vec.iter().max().unwrap(),
        _ => println!("Hi"),
    }
    if window_size > vec.len() {
        return vec.iter().product::<usize>();
    }
    let mut max_prod: usize = 0;
    let mut current_prod = 1;
    for i in 0..vec.len() - window_size + 1 {
        current_prod = vec[i..i + window_size].iter().product();
        if current_prod > max_prod {
            max_prod = current_prod;
        }
    }
    max_prod
}

fn main() {
    let window_size = 13;
    let s = r"
    73167176531330624919225119674426574742355349194934
    96983520312774506326239578318016984801869478851843
    85861560789112949495459501737958331952853208805511
    12540698747158523863050715693290963295227443043557
    66896648950445244523161731856403098711121722383113
    62229893423380308135336276614282806444486645238749
    30358907296290491560440772390713810515859307960866
    70172427121883998797908792274921901699720888093776
    65727333001053367881220235421809751254540594752243
    52584907711670556013604839586446706324415722155397
    53697817977846174064955149290862569321978468622482
    83972241375657056057490261407972968652414535100474
    82166370484403199890008895243450658541227588666881
    16427171479924442928230863465674813919123162824586
    17866458359124566529476545682848912883142607690042
    24219022671055626321111109370544217506941658960408
    07198403850962455444362981230987879927244284909188
    84580156166097919133875499200524063689912560717606
    05886116467109405077541002256983155200055935729725
    71636269561882670428252483600823257530420752963450
    ";
    print!("{:?}", s.chars());
    let digits: Vec<usize> = s
        .chars()
        .filter_map(|c: char| c.to_digit(10))
        .map(|d| d as usize)
        .collect::<Vec<usize>>();
    println!(
        "The largest product is {}",
        max_product(window_size, &digits)
    );
    // The largest product is 23514624000
}
