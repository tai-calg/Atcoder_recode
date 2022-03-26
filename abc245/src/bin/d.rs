#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input! {n:usize, m:usize,mut A:[isize;n+1], mut C:[isize;n+m+1]}


    A.reverse();
    C.reverse();

    let mut ans:Vec<isize> = Vec::new();

    for i in 0..(m+1) {
        let ele = C[i] /A[0];

        for j in 1..A.len() {
            C[i+1+j-1] -= ele*A[j];
        }
        ans.push(ele);
    }


    ans.reverse();

    for a in ans {
        print!("{} ", a);
    }
}
