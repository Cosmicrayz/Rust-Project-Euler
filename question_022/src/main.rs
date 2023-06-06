/* Prompt: Using names.txt (right click and 'Save Link/Target As...'), a 46K text file containing over five-thousand first names, begin by sorting it into alphabetical order. Then working out the alphabetical value for each name, multiply this value by its alphabetical position in the list to obtain a name score.

For example, when the list is sorted into alphabetical order, COLIN, which is worth
, is the
th name in the list. So, COLIN would obtain a score of
.

What is the total of all the name scores in the file?

Link: https://projecteuler.net/problem=22

 */
use std::collections::HashMap;
use std::fs;

fn letter_score_map() -> HashMap<String, usize> {
    HashMap::from([
        ("A".to_string(), 1),
        ("B".to_string(), 2),
        ("C".to_string(), 3),
        ("D".to_string(), 4),
        ("E".to_string(), 5),
        ("F".to_string(), 6),
        ("G".to_string(), 7),
        ("H".to_string(), 8),
        ("I".to_string(), 9),
        ("J".to_string(), 10),
        ("K".to_string(), 11),
        ("L".to_string(), 12),
        ("M".to_string(), 13),
        ("N".to_string(), 14),
        ("O".to_string(), 15),
        ("P".to_string(), 16),
        ("Q".to_string(), 17),
        ("R".to_string(), 18),
        ("S".to_string(), 19),
        ("T".to_string(), 20),
        ("U".to_string(), 21),
        ("V".to_string(), 22),
        ("W".to_string(), 23),
        ("X".to_string(), 24),
        ("Y".to_string(), 25),
        ("Z".to_string(), 26),
    ])
}

fn main() {
    let path = "names.txt";
    let mut names = fs::read_to_string(&path)
        .unwrap()
        .split(",")
        .map(|line| line.to_string().replace('"', ""))
        .collect::<Vec<String>>();
    println!("{}", names[5]);
    names.sort();
    let mut total = 0;
    let scores = letter_score_map();
    for (idx, name) in names.into_iter().enumerate() {
        for char in name.chars() {
            total += (idx + 1) * scores[&char.to_string()];
        }
    }
    println!("The total score of all names in the file is {}", total)
    // The total score of all names in the file is 871198282
}
