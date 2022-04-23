use proconio::*;

fn main() {
    input!(n: usize, p: u64);
    let mut dp = vec![vec![0u64; n + 2]; n + 1];
    for (len, &d) in [1, 10, 100, 1000].iter().enumerate() {
        let len = len + 2;
        if len <= n {
            if d <= n {
                dp[len][d] += 26;
            }
            if 10 * d <= n {
                dp[len][10 * d] += p - 26;
            }
        }
    }
    for i in 1..=n {
        for j in 1..=n {
            dp[i][j] %= p;
            let v = dp[i][j];
            dp[i][j + 1] += v;
            if v == 0 {
                continue;
            }
            let v = 25 * v % p;
            for (len, &d) in [1, 10, 100, 1000].iter().enumerate() {
                let t = i + len + 2;
                if t <= n {
                    if j + d <= n {
                        dp[t][j + d] += v;
                    }
                    if j + 10 * d <= n {
                        dp[t][j + 10 * d] += p - v;
                    }
                }
            }
        }
    }
    let ans = dp[..n].iter().fold(0, |s, a| s + a[n]) % p;
    println!("{}", ans);
}
