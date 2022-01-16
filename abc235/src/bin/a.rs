#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input! {abc: usize};

    let a = abc / 100 ;

    let b = abc / 10 - a*10;

    let c = abc - a*100 - b*10;


    let bca = b * 100 + c * 10 + a;
    let cab = c * 100 + a * 10 + b;

    let ans = abc + bca + cab ;

    println!("{}", ans);
}

fn solve(){

}
