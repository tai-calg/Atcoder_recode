#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input! {n:usize, mut A:[usize;n]}

    A.sort();

    for i in 0..=2001 {
        if check(&A, i) {
            println!("{}", i);
            return;
        }


    }


}

fn check (A: &Vec<usize>, i: usize) -> bool{
    for &a in A {
        if i == a {
            return false;//存在したらダメ
        }
    }

    return true;
}