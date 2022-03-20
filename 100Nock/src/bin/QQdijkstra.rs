use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;
 
const MOD:usize = 998244353;
fn main() {
  input! {
    n:usize,
    m:usize,
    k:usize,
    s:Usize1,
    t:Usize1,
    x:Usize1,
    edges:[(Usize1,Usize1);m]
  }
 
  let mut g = vec![vec![];n];
  for (a, b) in edges {
    g[a].push(b);
    g[b].push(a);
  }
 
  let mut memo = vec![vec![0;n+1];2];
  memo[0][s] = 1usize;
 
  for _ in 0..k {
    let mut new_memo = vec![vec![0;n+1];2];
    for ci in 0..n {
      for &ni in &g[ci] {
        if ni == x {
          for a in 0..2 {
            let nni = (a+1) % 2;
            new_memo[nni][ni] += memo[a][ci];
            new_memo[nni][ni] %= MOD;
          }
        } else {
          for a in 0..2 {
            new_memo[a][ni] += memo[a][ci];
            new_memo[a][ni] %= MOD;
          }
        }
      }
    }
    memo = new_memo;
  }
 
  println!("{}", memo[0][t]);  
}