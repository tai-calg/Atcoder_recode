#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
use std::collections::HashSet;


#[fastout]
fn main() {
    input! {S:String}


    let mut sum = 45;

    for s in S.chars() {

        let z = s.to_string().parse::<i32>().unwrap();

        sum -= z;

    }

    println!("{}", sum);





}
