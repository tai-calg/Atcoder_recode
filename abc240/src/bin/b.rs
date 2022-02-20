#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::HashSet;

use proconio::*;
 
#[fastout]
fn main() {
    input! {n:usize, A:[usize;n]};

    let mut hash :HashSet<usize> = HashSet::new();

    for a in A {
        hash.insert(a);
    }
    println!("{}", hash.len());


}
