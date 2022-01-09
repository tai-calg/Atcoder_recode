use proconio::input;
//use std::fmt;
fn main() {
    input! {k:i128};

    //let kbin = to_binary(k);

    let kbin:i128 = format!("{:b}",k).parse().unwrap();

    println!("{}", kbin *2);



}
/*
fn to_binary(mut decimal: i128) -> i128 {
    if decimal == 0 {
        decimal
    } else {
        let mut bits = String::new();

        while decimal > 0 {
            if decimal % 2 == 0 {
                bits.push_str("0");
            } else {
                bits.push_str("1");
            }

            decimal /= 2;
        }

        // reverse the bits
        //println!("{}", &bits);
        match bits.chars().rev().collect::<String>().parse() {
            Ok(num) => num,
            Err(_e) => panic!("Something went wrong"),
        }
    }
}
*/

/* Answer
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
 
#[fastout]
fn main() {
    input! {
        k: usize,
    }
    fn base_10_to_2(k: u64) -> String {
        let mut binary = String::new();
        let mut i = k;
        while i > 0 {
            binary.push_str(&format!("{}", i % 2));
            i /= 2;
        }
        binary.chars().rev().collect()
    }
    // Decimal number to Binary number
    let ans = base_10_to_2(k as u64);
    println!("{}", ans.replace("1", "2"));
}
*/