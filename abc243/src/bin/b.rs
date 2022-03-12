#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input! {n:usize, 
    A:[usize;n],
    B:[usize;n],
    }
    let mut ans1 = 0;
    let mut ans2 = 0;
    for i in 0..n {
        if A[i] == B[i] {
            ans1 += 1;
        }
        for j in 0..n {
            if A[i] == B[j] && i != j {
                ans2 += 1;
            }
        }
    }
    println!("{}", ans1);
    println!("{}", ans2);




}
