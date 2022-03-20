/*

https://atcoder.jp/contests/abc239/tasks/abc239_e 

問題文
N 頂点の根付き木があります。頂点には 1 から N の番号がついており、根は頂点 1 です。
i 番目の辺は頂点 AiとBi​を結んでいます。
頂点 i には整数 Xi
​
  が書かれています。

Q 個のクエリが与えられます。i 番目のクエリでは整数の組 (Vi,Ki) が与えられるので、次の問題に答えてください。

問題：
頂点 Vi​
  の部分木に含まれる頂点に書かれた整数のうち、大きい方から Ki番目の値を求めよ
*/

#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
use proconio::marker::Usize1 as U1;
 
#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        x: [u32; n], //ノードの中身
        ab: [(U1, U1); n - 1], //ab is edge 
        vk: [(U1, U1); q] //ans... v:vertex のmaxからk番目  
    }
 
    let mut graph = vec![vec![]; n]; //graph : 無向グラフ
    for &(a, b) in &ab { //無向グラフ ... 蟻本p90の隣接行列
        graph[a].push(b);
        graph[b].push(a);
    }
 
    
    let mut y = vec![vec![]; n];//yは答え表
    dfs(&graph, &x, &mut y, 0, 0);
    
    for &(v, k) in &vk { //important! v:vertex のmaxからk番目 を行列として作成
        println!("{}", y[v][k]);
    }
    println!("{:?}", y);
}

fn dfs(graph: &Vec<Vec<usize>>, x: &Vec<u32>, y: &mut Vec<Vec<u32>>, i: usize, p: usize) -> Vec<u32> {
    let mut c = vec![x[i]];
    for &j in &graph[i] {
        if j != p {
            c.append(&mut dfs(graph, x, y, j, i));
        }
    }
    c.sort_by(|a, b| b.cmp(&a));
    c.resize(20, 0);
    y[i] = c.clone();
    c
}