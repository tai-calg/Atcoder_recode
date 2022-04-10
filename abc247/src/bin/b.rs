
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    }

    for i in 0..n {
        let mut set = HashSet::new();
        for j in (0..n).filter(|x| *x != i) { //自分との比較を除く
            set.insert(&st[j].0);
            set.insert(&st[j].1);
        }
        if set.contains(&st[i].0) && set.contains(&st[i].1) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
/*
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
use std::collections::HashSet;

#[fastout]
fn main() {

    input! {n:usize, ST: [(String,String);n]}
    //同性or 同名の択は選べない

    let mut hs:HashSet<String> = HashSet::new();
    let mut checkhs:HashSet<String> = HashSet::new();


    for (s,t) in ST.clone() {
        if hs.insert(s.clone()) == false {
            checkhs.insert(s.clone());
        }
        if hs.insert(t.clone()) == false {
            checkhs.insert(t.clone());

        }

    }


    for (s,t) in ST {
        if checkhs.contains(&s) && checkhs.contains(&t) {
            println!("{}", "No");
            return;
        }

    }

    println!("{}", "Yes");






}
*/