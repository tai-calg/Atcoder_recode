
/* 
use std::vec;

use proconio::{*};
#[fastout]
fn main() {
    input! {n:usize, k:usize}
    input! {mut  p : [usize;n]}


    let mut ppart = vec![];

    ppart = p[0..k].to_vec();
    ppart.sort();
    ppart.reverse();
    println!("{}", ppart[k-1]);
    for i in k+1..=n{
        //let mut Ppart = vec![];
        //Ppart.copy_from_slice(&p[..i-1]);


        //let mut pi = ppart.iter().position(|&u| u >= p[i-1]);
        //if pi == None { pi = Some(i-1)}
        //
        //ppart.insert(pi.unwrap(),p[i-1]);
        ppart.push(p[i-1]);
        ppart.sort();
        ppart.reverse();
        //println!("{:?}", &ppart);
        println!("{}", ppart[k-1]);
    }
}
*/


#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::{*, marker::*};

use std::cmp::{max, min};
use std::collections::{VecDeque, HashMap, BTreeMap, HashSet, BTreeSet};
use itertools::Itertools;
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
        P: [usize; n],
    }
    let mut f = vec![0;n];
    for &i in P[0..k].iter() {
        f[i-1] = 1;
    }
    //n番目に1があればnがあるというのを知らせる配列ｆ. k <= n の制約あり 

    //println!("{:?}", &f);
    let mut index = P[..k].iter().min().unwrap() - 1;
    println!("{}", index+1); // 一回目は最小値求めるだけ

    for &i in P[k..].iter() {
        if i > index+1 {
            f[i-1] = 1;
            index = next_index(index, &f);
        }
        println!("{}", index+1);
    }
}
 
fn next_index(index: usize, n: &Vec<usize>) -> usize {//次の最小値を返す。
    let mut result = 0;
    for i in index+1..n.len() {
        if n[i] == 1 {
            result = i;
            break;
        }
    }
    result
}
