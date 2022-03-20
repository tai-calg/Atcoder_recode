#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::{input, fastout};

#[fastout]
fn main() {

    input! {
        s1:char, s2:char, s3:char,
        t1:char, t2:char, t3:char,
    }


    //count 揃っている文字
    let mut count = 0;
    for (s,t) in vec![(s1,t1),(s2,t2),(s3,t3)].iter() {
        if s == t {
            count += 1;
        }
    }



    if count == 3 {
        println!("Yes");
    }else if count == 1 {
        println!("No");   
    }else if count == 0 {
        //count == 1の状態に奇数回で持っていけたらYes
        println!("Yes");   
    }

}

