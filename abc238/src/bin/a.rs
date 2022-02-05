#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use proconio::*;
 
#[fastout]
fn main() {
    input! {n:f32}

    if n > 2f32 * f32::log2(n) {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }

}
