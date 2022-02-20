#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input! {a:isize,b:isize}

    if solve(a,b) {
        println!("Yes");
    }else {
        println!("No");
    }
}

fn solve(a:isize, b:isize)->bool {
    let a1 = a %10;
    let b1 = b %10;
    if (b1-a1).abs() == 1 || (b-a).abs() ==1 {
        return true;
    } else {
        return false;
    }


}
