fn main() {
    println!("edit distance of words");
    println!("{}", edit_distance("horse".to_owned(), "ros".to_owned()))
}

fn edit_distance(word1: String, word2: String) -> usize {
    let (m, n) = (word1.len(), word2.len());
    // initiate dp table
    let mut dp = vec![vec![0; n + 1]; m + 1];
    // emptying the word1, w/o moving dp
    for (idx, row) in dp.iter_mut().take(m + 1).enumerate() {
        row[0] = idx;
    }
    // take yields m + 1 element iterator, or it can
    // be made to yield fewer numbers.
    // updating the word2 with chars of word2
    for jdx in 0..n + 1 {
        dp[0][jdx] = jdx
    }
    // convert given words to vectors of chars for the next steps
    let word1chars: Vec<char> = word1.chars().collect();
    let word2chars: Vec<char> = word2.chars().collect();
    for idx in 1..=m {
        for jdx in 1..=n {
            if let Some(x) = word1chars.get(idx - 1) {
                if let Some(y) = word2chars.get(jdx - 1) {
                    if x == y {
                        dp[idx][jdx] = dp[idx - 1][jdx - 1];
                    } else {
                        dp[idx][jdx] = [
                            dp[idx - 1][jdx] + 1,
                            dp[idx][jdx - 1] + 1,
                            dp[idx - 1][jdx - 1] + 1,
                        ]
                        .iter()
                        .min()
                        .unwrap()
                        .to_owned();
                    }
                }
            }
        }
    }
    use bt_pattern_02::print_vector_generic;
    print_vector_generic(dp.clone());
    dp[m][n]
}
