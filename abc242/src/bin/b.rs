#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input! {s: String}

    let mut s: Vec<char> = s.chars().collect();
    s.sort();
    let s: String = s.into_iter().collect();
    println!("{}", s);
}
