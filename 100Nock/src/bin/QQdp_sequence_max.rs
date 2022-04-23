
use std::cmp::max;
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_10_C&lang=ja 
//　最長共通部分列問題
fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n: usize = n.trim().parse::<usize>().unwrap();
    for _ in 0..n{
        let mut a = String::new();
        std::io::stdin().read_line(&mut a).ok();
        let a: String = a.trim().to_string();
        let mut b = String::new();
        std::io::stdin().read_line(&mut b).ok();
        let b: String = b.trim().to_string();
        let m: usize = a.len();
        let n: usize = b.len();
        let a: Vec<char> = a.chars().collect();
        let b: Vec<char> = b.chars().collect();
        let mut dp: Vec<Vec<usize>> = vec![vec![0; n+1]; m+1];
        for i in 1..m+1 {
            for j in 1..n+1 {
                if a[i-1] == b[j-1] {
                    dp[i][j] = dp[i-1][j-1] + 1;
                } else {
                    dp[i][j] = max(dp[i-1][j], dp[i][j-1]);
                }
            }
        }
        println!("{}", dp[m][n]);
    }
}