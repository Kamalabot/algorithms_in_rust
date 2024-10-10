use std::collections::HashSet;

fn main() {
    let s = "leetcode";
    let dict: Vec<&str> = vec!["leet", "code"];
    println!("{}", word_break(s, dict));
}

fn word_break(s: &str, dict: Vec<&str>) -> bool {
    let word_set = dict.into_iter().collect::<HashSet<_>>();
    println!("{:?}", word_set);
    let mut dp = vec![false; s.len() + 1];
    // base case
    dp[0] = true;
    // filling the dp array
    for idx in 1..=s.len() {
        for jdx in 0..idx {
            if word_set.contains(&s[jdx..idx]) && dp[jdx] {
                dp[idx] = true;
                break;
            }
        }
    }
    dp[s.len()]
}
