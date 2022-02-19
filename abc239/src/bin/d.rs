#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
use std::{collections::{ HashSet}, vec}; 


#[fastout]
fn main() {


    input! {a:usize,b:usize,c:usize,d:usize};

    if canbePrime(a,b,c,d) {
        println!("Aoki" );
    }else {
        println!("Takahashi");
    }
}

fn canbePrime (a:usize,b:usize,c:usize,d:usize)->bool {
    
    let prime = vec![ 2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97
    ,101 ,103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199];
    //差分max 14 127 - 131

    //let taka = select(a, b,c,d);

    let mut flag = false;
    let mut count =0;
    for i in a..b+1 {
        for j in c..d+1 {
            if prime.contains(&(i+j)) { //一つでも存在したらまける、そのiは選べない
                count +=1;
                break; //次のiへ
            }

        }
    }

    if count == ( b+1 -a) //高橋は何選んでも素数になってしまう
    {
        flag = true;//青木勝ち
    }
    return flag;
}


