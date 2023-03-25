use std::cmp::max;
use std::iter::zip;
use std::ops;
#[derive(Debug)]
pub struct BigNum {
    pub digits: Vec<usize>,
}

impl ops::Add<BigNum> for BigNum {
    type Output = BigNum;
    fn add(self, other: BigNum) -> Self::Output {
        // Equalise vector lengths
        let mut first_num: Vec<usize> = vec![
            0;
            max(
                (other.digits.len() as isize) - (self.digits.len() as isize),
                0
            ) as usize
        ];
        let mut second_num: Vec<usize> = vec![
            0;
            max(
                (self.digits.len() as isize) - (other.digits.len() as isize),
                0
            ) as usize
        ];
        first_num.extend(&self.digits);
        second_num.extend(&other.digits);
        let mut carry = 0;
        let mut output: Vec<usize> = Vec::new();
        for (i, j) in zip(first_num, second_num).rev() {
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

impl ops::Mul<BigNum> for BigNum {
    type Output = BigNum;
    fn mul(self, other: BigNum) -> Self::Output {
        let mut first_num: Vec<usize> = vec![
            0;
            max(
                (other.digits.len() as isize) - (self.digits.len() as isize),
                0
            ) as usize
        ];
        let mut second_num: Vec<usize> = vec![
            0;
            max(
                (self.digits.len() as isize) - (other.digits.len() as isize),
                0
            ) as usize
        ];
        first_num.extend(&self.digits);
        second_num.extend(&other.digits);

        let mut result: BigNum = BigNum { digits: vec![0; 1] };
        for (index, num_1) in first_num.iter().rev().enumerate() {
            let mut carry = 0;
            let mut output: Vec<usize> = vec![0; index];
            for num_2 in second_num.iter().rev() {
                let digit = (num_1 * num_2 + carry) % 10;
                carry = (num_1 * num_2 + carry) / 10;
                output.insert(0, digit);
            }
            if carry != 0 {
                output.insert(0, carry);
            }
            // Truncate any leading zeroes to keep the result minimal
            output = output.into_iter().skip_while(|&x| x == 0).collect();
            result = result + BigNum { digits: output };
        }
        return result;
    }
}
