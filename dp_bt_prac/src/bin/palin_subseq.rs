// A palindromic subsequence is a sequence of
// characters that reads the same
// forwards and backwards. In this case,
// we need to find the longest
// subsequence that is a palindrome within
// the string "bbbab".
//
// bbbb is longest subseq, and it need not
// be contiguous
fn main() {
    println!("palindromic subseq");
    let s = "bbbab";
    println!("{}", longest_palin_subseq(s));
}

fn longest_palin_subseq(s: &str) -> i32 {
    let n = s.len();
    let s_chars: Vec<char> = s.chars().collect();
    // dp table
    let mut dp = vec![vec![0; n]; n];
    // single chars are palins of length 1
    // for idx in 0..n {
    //     dp[idx][idx] = 1
    // }
    for (idx, row) in dp.iter_mut().take(n).enumerate() {
        row[idx] = 1;
        // println!("{:?}", row);
        // println!("idx is {:?}", idx);
    }
    for length in 2..=n {
        for idx in 0..(n - length + 1) {
            let jdx = idx + length - 1;
            if s_chars[idx] == s_chars[jdx] {
                dp[idx][jdx] = dp[idx + 1][jdx - 1] + 2;
            } else {
                dp[idx][jdx] = std::cmp::max(dp[idx + 1][jdx], dp[idx][jdx - 1]);
            }
        }
    }
    dp[0][n - 1]
}
