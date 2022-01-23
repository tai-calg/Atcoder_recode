#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input! {n:usize, m:usize,
        mut S:[String;n],
        mut T:[String;m]
    }
    S.reverse();

    T.reverse();


    for _ in 0..S.len(){
        if T[T.len() - 1] == S.pop().unwrap() {
            println!("Yes");
            T.pop();
        }else {
            println!("No" );
        }
    }



}
