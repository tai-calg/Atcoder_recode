#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input! {x:f64, y:f64}

    let d = (x * x + y * y).sqrt();
    let tan = y / x;
    let cos = x / d;
    let sin = y / d;

    println!("{} {}", cos, sin);
}
