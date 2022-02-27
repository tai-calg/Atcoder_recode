#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
input! {A:[usize;10]}

    println!("{}", A[A[A[0]]]);
}
