#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::BinaryHeap;

use proconio::*;
 
#[fastout]
fn main() {
    input! {n:usize, m:usize}
    input! {mut A:[usize;n]}
    input! {mut B:[usize;m]}
    let mut flg = true;

    A.sort();
    B.sort();

    for i in B{
        let val = A.binary_search(&i);

        match val {
            Ok(_) => {A.remove(val.unwrap());},
            Err(_) => {flg = false; break;}
        }
            
        

        
    }

    if flg { println!("Yes"); } else { println!("No"); }
}
