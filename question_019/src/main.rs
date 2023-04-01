/* Prompt: You are given the following information, but you may prefer to do some research for yourself.

1 Jan 1900 was a Monday.
Thirty days has September,
April, June and November.
All the rest have thirty-one,
Saving February alone,
Which has twenty-eight, rain or shine.
And on leap years, twenty-nine.
A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.
How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?

Link: https://projecteuler.net/problem=19
 */
fn is_leap_year(num: usize) -> bool {
    if num % 400 == 0 {
        return true;
    }
    if num % 100 == 0 {
        return false;
    }
    if num % 4 == 0 {
        return true;
    }
    false
}

const DAYS_FOR_MONTH: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const DAYS_FOR_MONTH_LEAP_YEAR: [i32; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn count_for_year(start_day: i32, year: i32, target_day: i32) -> (i32, i32) {
    /* Takes in the starting day and the year value as well as the target day
    we're counting and returns a tuple of the number of target days
    in the given year and next starting day
     */
    let day_counts: [i32; 12] = match is_leap_year(year as usize) {
        true => DAYS_FOR_MONTH_LEAP_YEAR,
        false => DAYS_FOR_MONTH,
    };
    let mut target_day_counter = 0;
    let mut start = start_day;
    if start == target_day {
        target_day_counter += 1;
    }
    for month in 1..12 {
        start = (start + day_counts[month - 1]) % 7;
        if start == target_day {
            target_day_counter += 1
        }
    }
    // returns how many of that target day is in the year and the starting day for the next year
    (target_day_counter, (start + (day_counts[11] % 7)) % 7)
}

fn main() {
    let mut day_counts = 0;
    let mut start_day = 0;
    for year in 1900..2001 {
        let (day_count, start) = count_for_year(start_day, year, 6);
        start_day = start;
        if year > 1900 {
            day_counts += day_count;
        }
    }
    println!(
        "The number of sundays between 1st Jan 1901 and 31 Dec 2000 is {:?}",
        day_counts
    );
    // The number of sundays between 1st Jan 1901 and 31 Dec 2000 is 171
}
