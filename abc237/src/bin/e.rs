#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::{vec, collections::BinaryHeap};

use proconio::*;
 
#[fastout]
fn main() {
    input! {n:usize, m:usize,
    H : [i32;n]}

    let mut g = vec![Vec::new();n];

    for _ in 0..m { //graph作成
        input!{
            mut u:usize,
            mut v:usize,
        }

        u -= 1;
        v -= 1;
        let cost = H[u] - H[v];
        if cost < 0 { //上る痛み（マイナスコスト）
            g[u].push((v,-cost));
            g[v].push((u,0));
        }
        else{
            g[u].push((v,0));
            g[v].push((u,cost));
        }
    }

    //dijkstra
    let mut q = BinaryHeap::new(); 
    q.push((0i64,0usize));
    const INF : i64 = i64::max_value();
    let mut result = vec![INF;n];


}
