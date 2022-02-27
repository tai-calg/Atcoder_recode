
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::{collections::HashMap, vec};


use proconio::*;
 
#[fastout]
fn main() {
    input! {n:usize, q:usize}
    input! {A:[usize;n]}
    input! {XK: [(usize,usize);q]}


    let mut m = HashMap::new();
    for i in 0..n {
        let count = m.entry(A[i]).or_insert(vec![]); 
        //entryで挿入。or_insert ではじめて挿入するときの処理
        //https://keens.github.io/blog/2020/05/23/rustnohashmaphaentrygabenri/

        count.push(i+1);

    }

    for (x,k) in XK.into_iter() {
        if !m.contains_key(&x){
            println!("{}", -1);
        }else if k > m[&x].len() {
            println!("{}", -1);
        }else {
            println!("{}", m[&x][k-1]);
        }
    }
}