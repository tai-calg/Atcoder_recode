use std::cmp::min;
use std::io;
use std::io::prelude::*;

// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_A&lang=ja 

// answer
// https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=3286099#1 

fn main() {
    let mut stdin = io::stdin();
    let mut buf = String::new();
    let _ = stdin.read_to_string(&mut buf);
    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let c: Vec<usize> = iter.map(|s| s.parse().unwrap()).collect();

    let mut dp = vec![std::usize::MAX; n + 1];
    dp[0] = 0;

    while dp[n] == std::usize::MAX {
        for i in 0..n {
            for c_i in c.iter() {
                if i + c_i <= n && dp[i] < std::usize::MAX { // idx overflow への対策はifでやる
                    dp[i+c_i] = min(dp[i+c_i], dp[i] + 1);
                }
            }
        }
    }

    println!("{}", dp[n]);
}
