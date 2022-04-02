#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input! {n:usize}

    let minb = ((n/4) as f64).sqrt() as usize;
    let mut ans:u128 = u128::MAX;

    for b in minb..=n {
        for a in b..=n {
            let ni = a*a + b * b;
            let wa = a+b;
            let x: u128 = ni as u128 * wa as u128 ;
            if x >= n as u128  && x < ans {
                ans = x;
            }
        }
    }
    println!("{}",ans);
}
