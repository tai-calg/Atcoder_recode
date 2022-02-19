#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input! {x:i128}

    if x > 0 {
        println!("{}",  x/10 );
    }else{

        if x% 10 == 0 {
            println!("{}", x/10);
        }else {
            println!("{}", x/10 -1);
        }

    }
}
