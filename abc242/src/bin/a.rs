#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input! {a:usize, b:usize, c:usize, x:usize}

    if x <= a {
        println!("{}", 1 as f64);
    }else if x <= b && x > a{
        println!("{}", (c as f64)/ ( (b-a) as f64));
    }else{
        println!("{}", 0 as f64);
    }

}
