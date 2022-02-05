#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {

    input! {n:usize, A:[usize;n]}

    let mut dp: Vec<usize> = vec![0;n+1];
    const RAD : usize = 360;
    
    let mut sum = 0;
    for (i,&a) in A.iter().enumerate() {

        sum += a;
        sum %= RAD;
        dp[i+1] = sum; //dp[0] is 0 by default

        
    }

    dp.sort();
    dp.push(360); //pos 360を入れておく
    //println!("{:?}",dp );

    let mut ans = 0;
    for i in 0..dp.len()-1 {
        ans = std::cmp::max(ans, dp[i+1] - dp[i]);

    } 
    println!("{}", ans);

}
