#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
use std::collections::HashSet;
 
#[fastout]
fn main() {
    input! {S: String}

    let  S = S.chars().collect::<Vec<char>>();


    let mut hs: HashSet<char> = HashSet::new();
    for s in S.clone() {
        if hs.insert(s) == false {
            println!("{}", "No");
            return;
        }
    }


    let ans = S.iter().any(|c| c.is_ascii_uppercase()) && S.iter().any(|c| c.is_ascii_lowercase());
    println!("{}", if ans { "Yes" } else { "No" });

}
