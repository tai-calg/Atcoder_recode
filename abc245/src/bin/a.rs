#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input! {a:usize, b:usize, c:usize, d:usize}

    if (a *60 + b) <= (c * 60 + d) {
        println!("Takahashi");
    }else {
        println!("Aoki");
    }

}
