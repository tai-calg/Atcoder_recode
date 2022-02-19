#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
use std::collections::{ HashSet}; 

#[fastout]
fn main() {

    input! {xy:(i128,i128)};
    input! {xy2:(i128,i128)};
    if solve(xy,xy2) {
        println!("Yes");
    }else {
        println!("No" );
    }

}

fn solve ((x1,y1):(i128,i128),(x2,y2):(i128,i128))->bool {

    let tuplexy1 : [(i128,i128);8] = [(x1+1,y1+2),(x1-1,y1+2),(x1-1,y1-2),(x1+1,y1-2),
    (x1+2,y1+1),(x1-2,y1-1),(x1+2,y1-1),(x1-2,y1+1)];
    //おなじ
    
    let tuplexy2 : [(i128,i128);8] = [(x2+1,y2+2),(x2-1,y2+2),(x2-1,y2-2),(x2+1,y2-2),
    (x2+2,y2+1),(x2-2,y2-1),(x2+2,y2-1),(x2-2,y2+1)];


    let mut hashset = HashSet::new();
    for xy in tuplexy1.iter(){
        hashset.insert(xy);
    }
    assert_eq!(hashset.len(),8);


    for xy2 in tuplexy2.iter(){
        if hashset.contains(&xy2) {
            return true;
        }
    }

    return false;

}