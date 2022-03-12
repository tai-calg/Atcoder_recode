#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::VecDeque;

use proconio::*;
 
#[fastout]
fn main() {
    input! {n:usize,mut x:usize, S:String}
    let mut s = S.chars().collect::<Vec<char>>();
    let mut idx = 1;
    let mut deque :VecDeque<char> = VecDeque::new();

    deque.push_back(s[0]);
    while idx < n {



        if s[idx] == 'R' {
            deque.push_back('R');
            idx +=1;
        } else if s[idx] == 'L' {
            deque.push_back('L');
            idx +=1;
        }else {

            match deque.pop_back() {
                Some(c) => {
                    if c == 'R' || c == 'L' {
                        //何もしない
                    } else {
                        deque.push_back(c); //入れなおす
                        deque.push_back(c); //さらに入れる
                    }
                },
                None => {
                    deque.push_back('U'); 

                }
            }

            idx +=1;
        }


        
    }


    for &e in deque.iter() {
        if e == 'R' {
            x = 2*x+1;
        }else if e == 'L' {
            x = 2*x;
        }else{
            x /=2;
        }
    }

    println!("{}", x);



}
