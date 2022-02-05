use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
        as_: [(u64, u64); t],
    }

    for &(a, s) in &as_ {
        println!("{}", if f(a, s) { "Yes" } else { "No" });
    }
}

//補集合でboolを判定する。bool問題は簡単に書けることがおおい
//論理学っぽい？
fn f(a: u64, s: u64) -> bool {
    // (a << 1) + x == s
    if s < (a << 1) {
        return false;
    }
    let x = s - (a << 1);
    // x y (x&y) (x^y)
    // ===============
    // 0 0   0     0
    // 0 1   0     1
    // 1 0   0     1
    // 1 1   1     0
    x & a == 0
}


/* 
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input! {t:usize}
    input! {AS:[(u128,u128);t]}


    for (a,s) in AS {
        let mut flag = false;
        let to_bin = |v:u128| -> u128 { format!("{:b}",v ).to_string().parse().unwrap() };

        let abin = to_bin(a);
        println!("{}", abin);
        
        /* */
        for x in 0..s/2 +1{
            let y = s-x;
            if x & y == a {
                flag = true;
                break;
            }
            
        }
        if flag {
            println!("Yes");
        }else {
            println!("No");
        }

    }
}
*/