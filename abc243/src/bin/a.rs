#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input!{
        mut v:isize,a:isize, b:isize,c:isize
    }

    //F M T loop

    loop {
        v -= a;
        if v <0 {
            println!("F");
            break;
        }
        v -= b;
        if v <0 {
            println!("M");
            break;
        }
        v -= c;
        if v <0 {
            println!("T");
            break;
        }
        
    }
}
