#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input! {S:String}
    input! {a:usize, b : usize}

    let a1 = a-1;
    let b1 = b -1;
    
    let mut s = S.chars().collect::<Vec<char>>(); //文字列操作はこれ

    let tmp :char ;

    tmp = s[a1]  ;
    s[a1 ] = s[b1];
    s[b1] = tmp;
    
    println!("{}", s.iter().collect::<String>()); //　最後にString型に直す


}
