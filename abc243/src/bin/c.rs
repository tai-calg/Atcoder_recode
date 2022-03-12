
use proconio::input;
use std::collections::HashMap;
use std::cmp::{min, max};
use std::usize::{MAX, MIN};
//https://atcoder.jp/contests/abc243/tasks/abc243_c
fn main() {
    //前提：端っこの人間がどう動くかだけに注目すればいい。
    input! {
        n: usize,
        xy: [(usize, usize); n],
        s: String
    }
    let mut hm: HashMap<usize, (usize, usize)> = HashMap::new();//y をkey にする, value は xのmin,max .
    
    for (i, si) in s.chars().into_iter().enumerate() {
        let r = *hm.entry(xy[i].1 /* y */).or_insert((MAX, MIN));
        if si == 'L' {
            hm.insert(xy[i].1, (r.0, max(xy[i].0, r.1))); 
            // L → update 右端.　but if (最右端 && L) を更新できない→何もしない。

        } else if si == 'R' {
            
            hm.insert(xy[i].1, (min(xy[i].0, r.0), r.1)); 
            // R → update 左端.　but if　(最左端&&R) を更新できない　→何もしない。

        }

        let h = hm.get(&xy[i].1).unwrap();
        if h.0 < h.1 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}


/* 
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
use std::{collections::HashMap, process::exit};
 //TorF は簡単
#[fastout]
fn main() {
    input! {n:usize,
    XY:[(usize,usize);n],
    S:String
    }
    //S is L or R
    let s = S.chars().collect::<Vec<char>>();


        let mut flg = false;

    for (i,&xy1) in XY.iter().enumerate() {
        if flg { break;}

        for (j,&xy2) in XY.iter().enumerate() {
            if xy1 == xy2 {
                continue;
            }

            if xy1.1 != xy2.1 {
                continue;
            }else {

                if xy1.0 < xy2.0 && s[i] == 'R' && s[j] == 'L' {
                    flg = true;
                    break;
                    
                }else if xy1.0 > xy2.0 && s[i] == 'L' && s[j] == 'R' {
                    flg = true;
                    break;                    
                }

            }
        }

    }

    if flg {
        println!("Yes");
    }else {
        println!("No");
    }
}

*/