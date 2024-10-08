fn main() {
    println!("Least comomn sub");
    let subseq = lcs("abcde", "ace");
    println!("The subseq is: {}", subseq);
}

fn lcs(text1: &str, text2: &str) -> i32 {
    let (m, n) = (text1.len(), text2.len());
    // converting text to vectors
    let text1v: Vec<char> = text1.chars().collect();
    let text2v: Vec<char> = text2.chars().collect();
    // dp table
    let mut dp = vec![vec![0; n + 1]; m + 1];
    // no base case, direct enumeration
    for idx in 1..=m {
        for jdx in 1..=n {
            if text1v[idx - 1] == text2v[jdx - 1] {
                dp[idx][jdx] = dp[idx - 1][jdx - 1] + 1;
            } else {
                dp[idx][jdx] = std::cmp::max(dp[idx - 1][jdx], dp[idx][jdx - 1])
            }
        }
    }
    dp[m][n]
}
