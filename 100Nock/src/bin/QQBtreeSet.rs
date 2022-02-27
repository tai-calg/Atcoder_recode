#![allow(unused_imports)]
use std::collections::BTreeSet;

use proconio::{fastout, input, marker::Usize1};
// https://atcoder.jp/contests/abc241/tasks/abc241_d

#[fastout]
fn main() {
    input! {q: usize}
    let mut st = BTreeSet::new();
    for i in 0..q {
        input! {qq: usize}
        if qq == 1 {
            input! {x: i64}
            st.insert((x, i));//iで何回目にぶち込んだかを明記することで重複を避ける
        } else if qq == 2 {
            input! {x: i64, k: Usize1}
            let ans = st.range((0,0)..=(x, q)).rev().nth(k).map_or(-1, |v| v.0);
            //.range(..=(x, q))は０～ｘまでのなかの範囲に含まれる要素をループする;(x,q)はand andのイメージ
            // range(min..max) will yield elements from min (inclusive) to max (exclusive)
            //BTreeSetは小→大がデフォ
            println!("{}", ans);
        } else {
            input! {x: i64, k: Usize1}
            let ans = st.range((x, 0)..).nth(k).map_or(-1, |v| v.0);
            println!("{}", ans);
        }
    }
}
