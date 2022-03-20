use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

// https://atcoder.jp/contests/abc244/tasks/abc244_e
// 以下の条件を満たす数列 A=(A0, .., Ak) は何通りありますか？

/* 
アライグマ「E問題は要するに「頂点SからTへ、ちょうどK回で移動する方法のうち、頂点Xを偶数回通るものは何通り？」なのだ！
dp[n][v][f]=Sからn回移動して頂点vにいて、Xを通った回数の偶奇がfの移動方法の数
とすればO(K(N+M))で解けるのだ！」

dp[i][j][l]:=i回目の推移で現在jにいてXをl回(mod 2)通った場合の数,でDP. dp[0][S][0]から初めてdp[K][T][0]が答え

graphの根幹。
答えに沿った答え行列[[]]を作る（dpであることも多い） → g[[]]を作る → 辺をg.pushする 
→ for any_node の for any隣接ノード に対して処理を描く || dfsの中で graph の for any隣接ノードに対して処理を描く

今回の答え行列は「Xが偶奇回における。頂点kへの到達は何通りあるか。」
*/

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
 
  let mut g = vec![vec![];n]; //グラフは 2 dim
  for (a, b) in edges {
    g[a].push(b);
    g[b].push(a); // 無向グラフ
  }
 
  let mut memo = vec![vec![0;n+1];2]; //all 0, Xが偶奇回における。頂点kへの到達は何通りあるか。
  memo[0][s] = 1usize;
 
  for _ in 0..k {
    let mut new_memo = vec![vec![0;n+1];2];
    for ci in 0..n {//any 全頂点に対して
      for &ni in &g[ci] { //any その頂点に接続している頂点
        if ni == x { //問題文：xが偶数個という制約より, aはXを通った回数の偶奇
          for a in 0..2 {
            let nni = (a+1) % 2;//0→１、1→0
            new_memo[nni][ni] += memo[a][ci]; //ci→ni。niがXだったのでXは偶数になる。
            new_memo[nni][ni] %= MOD;
          }
        } else {
          for a in 0..2 {
            new_memo[a][ni] += memo[a][ci];//Xが奇数回を更新。niはXじゃなかったから。
            new_memo[a][ni] %= MOD;
          }
        }
      }
    }
    memo = new_memo;
  }
 
  println!("{}", memo[0][t]);  //Xが偶数回 && T
}