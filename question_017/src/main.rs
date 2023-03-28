/* Prompt: If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?


NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out numbers is in compliance with British usage.

Link: https://projecteuler.net/problem=17
 */
use std::collections::HashMap;

fn nums_lookup() -> HashMap<i32, &'static str> {
    HashMap::from([
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
        (11, "eleven"),
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
        (20, "twenty"),
        (30, "thirty"),
        (40, "forty"),
        (50, "fifty"),
        (60, "sixty"),
        (70, "seventy"),
        (80, "eighty"),
        (90, "ninety"),
        (100, "hundred"),
        (1000, "thousand"),
    ])
}

fn tens_and_units_portion(num: i32) -> String {
    let nums_map = nums_lookup();
    let remainder = num % 100;

    if remainder == 0 {
        return "".to_string();
    } else if remainder <= 20 {
        return format!(
            "{} {}",
            match num {
                1..=99 => "",
                _ => "and",
            },
            nums_map[&remainder].to_string()
        );
    }
    // Otherwise it's between 21 to 99
    let (tens, units) = (remainder - remainder % 10, remainder % 10);
    format!(
        "{} {}{}",
        match num {
            1..=99 => "",
            _ => "and",
        },
        nums_map[&tens],
        match units {
            0 => "".to_string(),
            _ => format!("-{}", nums_map[&units]),
        }
    )
}

fn hundreds_portion(num: i32) -> String {
    let hundreds_digit = (num % 1000 - num % 100) / 100;
    if hundreds_digit == 0 {
        return "".to_string();
    }
    let digit_str = format!("{} hundred", nums_lookup()[&hundreds_digit]);
    digit_str
}

fn thousands_portion(num: i32) -> String {
    let thousands_digit = num / 1000;
    if thousands_digit == 0 {
        return "".to_string();
    }
    let digit_str = format!("{} thousand", nums_lookup()[&thousands_digit]);
    digit_str
}

fn main() {
    let mut count = 0;
    for target in 1..1001 {
        let word = format!(
            "{} {} {}",
            thousands_portion(target),
            hundreds_portion(target),
            tens_and_units_portion(target)
        );
        count += word.replace(" ", "").replace("-", "").len();
    }
    println!("The toal number of letters are {}", count)
    // The toal number of letters are 21124
}
