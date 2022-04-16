#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input! {mut a:usize, b:usize, k:usize}

    let mut cont = 0;

    while a < b {
        a *= k;
        cont += 1;
    }

    println!("{}", cont);

}
