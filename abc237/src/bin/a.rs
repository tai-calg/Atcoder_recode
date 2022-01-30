#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {

    input! {n:i128}

    let bit32 :i128 = 1 << 31;
    let mbit32 :i128 = bit32 * -1;

    if  n < bit32  && n >= mbit32 {
        println!("Yes");
    }
    else{
        println!("No");
    }

}
