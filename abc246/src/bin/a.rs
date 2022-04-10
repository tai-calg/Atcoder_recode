#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input!{
        (x1, y1): (i64, i64),
        (x2, y2): (i64, i64),
        (x3, y3): (i64, i64),
    }

    let xv = vec![x1, x2, x3];
    let yv = vec![y1, y2, y3];


    let mut  notans = (0,0);
    let mut  ans = (0,0);


    for i in 0..3{
        for j in i+1..3 {

            if xv[i] == xv[j] {
                notans.0 = xv[i];
            }
        }
    }

    for i in 0..3 {
        if notans.0 != xv[i] {
            ans.0 = xv[i];
        }
    }
    //===

    for i in 0..3{
        for j in i+1..3 {

            if yv[i] == yv[j] {
                notans.1 = yv[i];
            }
        }
    }

    for i in 0..3 {
        if notans.1 != yv[i] {
            ans.1 = yv[i];
        }
    }


    println!("{} {}", ans.0, ans.1);
}
