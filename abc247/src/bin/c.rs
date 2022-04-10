#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input! {n:usize}

    let s1:String = "1".to_string();

    // let s2 = format!("{} {} {}", s1, 2 , s1);

    let mut sn : Vec<String> = Vec::new();
    sn.push(s1);
    for i in 1..n {
        sn.push(format!("{} {} {}", sn[i-1], i+1, sn[i-1]) );
    }

    println!("{}", sn[n-1]);

}
