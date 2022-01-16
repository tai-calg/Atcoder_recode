#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::{collections::HashMap, vec};


use itertools::Itertools;
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


    // Another Answer

    //A.iter().enumerate().for_each(|(i,ai)| {
    //    m.entry(ai).or_insert_with(|| vec![]).push(i as i32);
    //});
//
    //let empt = vec![];
    //let ans  = XK.into_iter().map(
    //    |(x,k)| 
    //        m.get(&x).unwrap_or_else(|| &empt)
    //        .get(k).cloned().unwrap_or_else(|| -2 ) +1
    //).join("\n");
    //println!("{}",ans);


    // My Answer

    //let a_max = *A.iter().max().unwrap();
    //let mut dp  = vec![vec![-1;10usize.pow(2)]; a_max+1];
    //assert!(dp[0].iter().all(|&x| x ==-1));
//
    //make_dp(&A, &mut dp);
//
    //for i in 0..q{
    //    let (x,k) = XK[i];
//
//
    //    if x > a_max {
    //        println!("{}", -1);
    //    }else{
    //        println!("{}",  dp[x][k]);
//
    //    }
    //}

}


//
//fn make_dp(a: &Vec<usize>, dp:&mut Vec<Vec<i32>>){ //dp[][]の中身を作る。中身は何番目か；インデックスである。
//
//    let mut loop_count = 0;
//    for &i in a{
//        loop_count += 1;
//
//        let mut j = 1;
//        loop {
//            if dp[i][j] == -1{
//                dp[i][j] = loop_count;
//                break;
//            }else{
//                j += 1;
//            }
//        }
//
//
//
//    }
//
//
//}