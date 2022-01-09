#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use proconio::*;
 
#[fastout]
fn main() {
    input! {n:usize, m:usize}
    
    input! {S:[[usize];m]}; //index 1 からs ik になる
    input! {P:[usize;m]}

    //let mut switch = 0; // 後でfor ++ で全パターン分やる(2^n回)
    let to_bin = |v:u32| -> String { format!( "{: >10}", format!("{:b}",v )).to_string() };
    //let mut bin_swch = to_bin(switch);

    //to_bin →　        0　スペースor 0にしよう



    for i in 0..2u32.pow(n as u32){

        let bin_swch = to_bin(i);
        //println!("{}", bin_swch);

        for &s in &S[3]{
            
        }

    }



}

/* 
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut s = vec![];
    for _ in 0..m {
        input! {
            k: usize,
            v: [Usize1; k]
        }
        s.push(v);
    }
    input! {
        p: [usize; m]
    }

    let mut ans = 0;
    for i in 0..(1 << n) {
        let mut state = vec![0; n];
        for j in 0..n {
            if (i >> j) & 1 == 1 { // & 1 は一番右の桁以外をそぎ落とす
                state[j] = 1;
            }
        }
        let mut ok = true;
        for j in 0..m {
            let mut count = 0;
            for &k in &s[j] {
                if state[k] == 1 {
                    count += 1;
                }
            }
            if count % 2 != p[j] {
                ok = false;
            }
        }
        if ok {
            ans += 1;
        }
    }
    println!("{}", ans);
}

*/