#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use proconio::*;
 
#[fastout]
fn main() {
    input! { n:String }

    let mut s = n.chars().collect::<Vec<char>>();

    println!("{}{}{}{}",0, s[0], s[1], s[2]);

}
