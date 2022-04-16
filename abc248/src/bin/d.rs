use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [usize; n],
        q: usize,
        lrx: [(usize, usize, usize); q],
    }

    let mut dict = vec![vec![]; n+1];
    for i in 0..n {
        dict[a[i]].push(i+1); //何番目かを記録　dp[A[i]][i]
    }

    for (l, r, x) in lrx {
        // eprintln!("{:?}", dict[x]);
        let r = match dict[x].binary_search(&r) { //何番目かを探して、そこのインデックスをとる：つまり何回入れたのかが出力される！
            Ok(pos) => pos+1,
            Err(pos) => pos,
        };
        let l = match dict[x].binary_search(&l) {
            Ok(pos) => pos,
            Err(pos) => pos,
        };
        // eprintln!("[{:?}, {:?}]", l, r);
        let ans = r - l;
        println!("{}", ans);
    }
}


/* 
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
use std::collections::HashMap;

 
#[fastout]
fn main() {

    input! {n:usize}
    let mut dp = vec![vec![0;n+1];n+1];
    input! {A:[usize;n]}
    input! {q:usize}


    //make dp
    dp[1][A[0]] = 1;
    for i in 1..n {
        for j in 0..n {
            dp[i+1][A[j]] = dp[i][A[j]]; 
        }
        dp[i+1][A[i]] += 1;
    }

    for _ in 0..q {
        input! {l:usize, r:usize, x:usize}
        let ans = dp[r][x] - dp[l-1][x];
        println!("{}", ans);
    }
        


}
*/