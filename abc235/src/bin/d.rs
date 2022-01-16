#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::{vec, collections::VecDeque};

use proconio::*;
 
#[fastout]
fn main() {
    input! {a:usize, n:usize}

    let m = 10usize.pow(6);

    let oo = 10*m;
    let mut dp = vec![oo;m];
    dp[1] = 0;
    let mut q = VecDeque::new();
    q.push_back(1);

    while let Some(x)  = q.pop_front() {
        for nx in next(x,a) {
            if nx < m && dp[nx] > dp[x] +1 {
                dp[nx] = dp[x] + 1;
                q.push_back(nx);
            }
        }
    }

    if dp[n] < oo {
        println!("{}", dp[n]);
    }else {
        println!("{}", -1);
    }

}


fn next (x:usize, a:usize)->Vec<usize> {
    let mut res = vec![a*x];
    if x>= 10 && x % 10 > 0 {
    res.push(format!("{}{}", x % 10 , x / 10).parse().unwrap()); //最下桁を最上桁に移動
    }

    res
}