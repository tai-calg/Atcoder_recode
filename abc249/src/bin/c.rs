#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {n:usize, k:usize}
    let mut v:Vec<Vec<char>> = vec![];
    for _ in 0..n {
        input! {S:String}
        v.push(S.chars().collect::<Vec<char>>());
    }

    let mut ans = 0;


    for bit in 0..(1<<n) {

        let mut S_all: Vec<char> = vec![];

        for i in 0..n {
            if  bit & (1<<i) == 0 {
                continue;
            }else {
                S_all.append(&mut v[i].clone());
            }
        }

        let mut map = HashMap::new();
        for s in S_all {
            *map.entry(s).or_insert(0) += 1;
        }

        let res = map.values().filter(|&v| *v == k).count();

        ans = std::cmp::max(ans, res);


    }

    println!("{}", ans);


    



}
