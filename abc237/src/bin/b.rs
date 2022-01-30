#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input! {h:usize, w:usize}
    input! {A:[[usize;w];h]}

    for i in 0..w{
        for j in 0..h{
            println!("{}", A[j][i]);
        }
    }

}
