#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
use std::collections::BTreeSet;
// 典型的なマッチング問題。チョコと箱が与えられて、うまく組み合わせて全チョコを箱に入れられるかどうかという問題。
// https://atcoder.jp/contests/abc245/tasks/abc245_e

// 解法は”大は小を兼ねる、が小は大を兼ねない”ので、でかいチョコから順に残ってるでかい箱に入れていく。

#[fastout]
fn main() {
    input! {n:usize, m:usize}
    input! {a:[usize;n], b:[usize;n], c:[usize;m], d:[usize;m]}

//Ai <= Ci , Bi <= Di

let mut all = vec![];
    for i in 0..n {
        all.push((a[i], 0, i));// (縦長さ, チョコor箱, index)
    }
    for i in 0..m {
        all.push((c[i], 1, i));
    }
    all.sort();// acの長さ→第二引数→indexの順でソート判定。ａ，ｃの長さが同じならチョコが前に来るようにする。
 
    let mut ans = true;
    let mut set = BTreeSet::new();
    for (_, e, i) in all.into_iter().rev() {//長いほうから処理（箱が前に来るようになった）
        if e == 0 { //if チョコ
            if let Some(&val) = set.range((b[i], 0)..).nth(0) {//b[i]はチョコの中でいちばん長い
                set.remove(&val);
            } else {
                ans = false;
            }
        } else { //if 箱
            set.insert((d[i], i));
        }
    }
 
    let ans = if ans {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);

}
