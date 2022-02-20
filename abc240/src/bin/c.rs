#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
/*
解説
https://twitter.com/kyopro_friends/status/1495394270282661893 

*/

#[fastout]
fn main() {

    input! {n:usize, x:usize}
    let mut a: Vec<(usize, usize)> = Vec::with_capacity(n);
    for _ in 0..n {
        input!{
            i1: usize,
            i2: usize
        }
        a.push((i1, i2));
    }


    let mut dp :Vec<Vec<bool>> = vec![vec![false;x+1];n+1];
    assert!(dp.len()== n+1);
/*
  ------------------------------ 
|
|
... x+1 横幅
|
|
 -------------------------------
 n+1 
 縦幅
*/

    dp[0][0] = true;

    for k in 0..n{
        for j in 0..x+1{
            if !dp[k][j] {
                continue;
            }
            if j + a[k].0 <= x {
                dp[k+1][j+a[k].0] = true;
            }
            if j + a[k].1 <= x{
                dp[k+1][j+a[k].1] = true;
            }
        }
    }
    println!("{}", if dp [n][x] { "Yes" } else {"No"});
    
    for i in 0..dp.len(){
        println!("{:?}", dp[i]);
    }


    

}
