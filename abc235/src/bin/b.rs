#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input! {n:usize};
    input! {H:[usize;n]};

    let mut ans = H[0];
    for i in 1..H.len(){
        if ans < H[i] {
            ans = H[i];
        }else{
            break;
        }
    }

    println!("{}", ans);
}
