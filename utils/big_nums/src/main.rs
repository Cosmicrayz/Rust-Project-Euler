fn main() {
    // Testing BigNum implementation
    let a: big_nums::BigNum = big_nums::BigNum { digits: vec![1, 2] };
    let b: big_nums::BigNum = big_nums::BigNum { digits: vec![9, 7] };
    println!("The answer is {:?}", a + b);
}
