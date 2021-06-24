struct Solution {}
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        digits
            .chars()
            .map(|c| to_letter(c))
            .reduce(|d1, d2| cartesian_product(d1, d2))
            .unwrap_or(vec![])
    }
}
fn cartesian_product(s1: Vec<String>, s2: Vec<String>) -> Vec<String> {
    s1.iter()
        .map(|d1| {
            s2.iter()
                .map(move |d2| {
                    let mut s = String::from(d1);
                    s.push_str(d2);
                    s
                })
                .collect::<Vec<_>>()
        })
        .flat_map(|s| s)
        .collect::<Vec<_>>()
}
fn to_letter(d: char) -> Vec<String> {
    match d {
        '2' => vec!["a".to_owned(), "b".to_owned(), "c".to_owned()],
        '3' => vec!["d".to_owned(), "e".to_owned(), "f".to_owned()],
        '4' => vec!["g".to_owned(), "h".to_owned(), "i".to_owned()],
        '5' => vec!["j".to_owned(), "k".to_owned(), "l".to_owned()],
        '6' => vec!["m".to_owned(), "n".to_owned(), "o".to_owned()],
        '7' => vec![
            "p".to_owned(),
            "q".to_owned(),
            "r".to_owned(),
            "s".to_owned(),
        ],
        '8' => vec!["t".to_owned(), "u".to_owned(), "v".to_owned()],
        '9' => vec![
            "w".to_owned(),
            "x".to_owned(),
            "y".to_owned(),
            "z".to_owned(),
        ],
        _ => vec![],
    }
}

fn main() {
    let result = Solution::letter_combinations("23".to_owned());
    println!("{:?}", result);
}
