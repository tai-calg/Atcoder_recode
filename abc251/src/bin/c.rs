#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input!{
        n:usize,
        ST:[(String, usize);n],
    }

    //hashmap
    let mut map = std::collections::HashMap::<String,(usize,usize)>::new();

    for i in 0..n {
        let (s,t) = ST[i].clone();
        map.entry(s).or_insert((t,i+1));//初回のinsertしか受け付けない
        //iは提出順

        
    }
    //print map
    // for (k,v) in map.iter() {
    //     println!("{} {} {}",k,v.0, v.1);
    // }
    //tの最大値の鍚素をpushする、iが一番小さいのが答え
    let mut ans = vec![std::usize::MAX;n];
    let max = map.values().max().unwrap().0;
    let mut t = 0;
    for (k,v) in map.iter() {
        if v.0 == max {
            t = v.0;
            ans.push(v.1);
        }
    }
    //print min ans
    println!("{}",ans.iter().min().unwrap());


}
/*
use proconio::input;
use std::collections::HashSet;

fn main() {
    input!{
        n: usize,
        st: [(String, usize); n],
    }
    let mut set = HashSet::new();
    let mut ti = vec![];
    for (i, (s, t)) in st.iter().enumerate() {
        if !set.contains(s) {
            set.insert(s);
            ti.push((*t, i));
        }
    }
    let mut ans = 0;
    let mut max_v = 0;
    for (t, i) in ti {
        if max_v < t {
            max_v = t;
            ans = i + 1;
        }
    }
    println!("{}", ans);
}
*/
