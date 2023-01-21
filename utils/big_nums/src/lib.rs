use std::iter::zip;
use std::ops;

#[derive(Debug)]
pub struct BigNum {
    pub digits: Vec<usize>,
}

impl ops::Add<BigNum> for BigNum {
    type Output = BigNum;
    fn add(self, other: BigNum) -> BigNum {
        if self.digits.len() != other.digits.len() {
            panic!(
                "Error occured adding vectors of different lengths {:?}, {:?}",
                self, other
            )
        } else {
            let mut carry = 0;
            let mut output: Vec<usize> = Vec::new();
            for (i, j) in zip(self.digits, other.digits).rev() {
                let digit = (i + j + carry) % 10;
                carry = (i + j + carry) / 10;
                output.insert(0, digit)
            }
            if carry != 0 {
                output.insert(0, carry)
            }
            return BigNum { digits: output };
        }
    }
}
