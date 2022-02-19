#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use proconio::*;
 
#[fastout]
fn main() {

    input! {h:f64}
    println!("{}", f(h));
}

fn f(x:f64)->f64{
   ( x*(12800000f64 + x)).sqrt()
}