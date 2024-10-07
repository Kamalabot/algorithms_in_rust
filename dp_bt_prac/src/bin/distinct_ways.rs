fn main() {
    println!("Distinct Ways to decode string");
    println!(
        "Number of ways to decode 2156 is {}",
        distinct_ways_rust("2156".to_owned())
    );
    println!(
        "Number of ways to decode 21a6 is {}",
        distinct_ways_rust("21a6".to_owned())
    );
    println!("Done...")
}

fn distinct_ways(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }
    // establish dp table
    let mut dp = vec![0; s.len() + 1];
    // base cases
    dp[0] = 1;

    #[allow(warnings)]
    if s.chars().nth(0).unwrap() != '0' {
        dp[1] = 1;
    } else {
        dp[1] = 0;
    }
    // enumerate over the rest
    for idx in 2..s.len() + 1 {
        if let Some(val) = s.chars().nth(idx) {
            if val != '0' {
                println!("Char is: {}", val);
                dp[idx] += dp[idx - 1];
            }
        }
        // s.chars().nth(idx).map(|c| {
        // if c != '0' {
        // dp[idx] += dp[idx - 1];
        // }
        // });

        // the below code looks at two chars together tries to parse them to numbers or else unwraps
        // TODO: need to add error handling
        // if 10 <= s[idx - 2..idx].parse::<i32>().unwrap()
        //     && s[idx - 2..idx].parse::<i32>().unwrap() <= 26
        // {
        //     dp[idx] += dp[idx - 2]
        // }
        // NOTE: Completed error handling
        if let Ok(num) = s[idx - 2..idx].parse::<i32>() {
            println!("2 digit num: {num}");
            if (10..=26).contains(&num) {
                dp[idx] += dp[idx - 2]
            }
        }
    }
    println!("final dp: {:?}", dp);
    dp[s.len()]
}

fn distinct_ways_rust(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }

    let mut dp = vec![0; s.len() + 1];

    dp[0] = 1;

    // Access string as bytes since we are dealing with digits.
    let bytes = s.as_bytes();

    if bytes[0] != b'0' {
        dp[1] = 1;
    } else {
        dp[1] = 0;
    }

    for idx in 2..=s.len() {
        // Single digit check
        if bytes[idx - 1] != b'0' {
            dp[idx] += dp[idx - 1];
        }

        // Two digit check (parsing the substring)
        if let (Some(b1), Some(b2)) = (bytes.get(idx - 2), bytes.get(idx - 1)) {
            // the two digit number is created, observe the *
            let two_digit_num = (*b1 as i32 - b'0' as i32) * 10 + (*b2 as i32 - b'0' as i32);
            if (10..=26).contains(&two_digit_num) {
                dp[idx] += dp[idx - 2];
            }
        }
    }

    dp[s.len()]
}
