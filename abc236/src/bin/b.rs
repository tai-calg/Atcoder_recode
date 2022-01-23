#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input! {n:usize}
    let n4 = 4 * n -1;
    input! {A:[usize;n4]}

    //let mut dp = Vec::new().resize_with(n, Default::default());

    let mut dp:Vec<usize> =Vec::new();

    dp.resize_with(n, || 4);
    let mut  ans :usize = 0;

    for i in A{
        dp[i -1] -= 1;
    }

    ans = dp.iter().position(|x| *x == 1).unwrap() + 1;

    println!("{}", ans);

}
