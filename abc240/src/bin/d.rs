#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::vec;

use proconio::*;
 
#[fastout]
fn main() {
    input! {n:usize}
    input! {A:[usize;n]}

    let mut stack:Vec<usize> = Vec::new();


    let mut count = 1;


    for i in 0..A.len() {
        solve(&mut stack, A[i],&mut count);
        println!("{} ", stack.len());
    }

    fn solve(stack : &mut Vec<usize>, a:usize, count :&mut usize) {
        //stackにpush,  連続かの確認、連続してるなら回数を数える、回数がaに一致したら、resize
        let continue_time = a;

        //stack が０になったときの処理を書かなきゃ
        if stack.len() == 0 {
            *count = 1;
            stack.push(a);
        }else {

            if stack[stack.len()-1] == a{ // 連続なら
                *count += 1;
                
            }else {
                *count = 1;
            }

            stack.push(a);



            if continue_time == *count {
                stack.resize(stack.len() - a, 0);
                *count =1;
                //連鎖で消えるときの処理

                    continuefn(stack);


            }


        }


        
        fn continuefn(stack:&mut Vec<usize>)->bool {
            if stack.len() <= 1 {
                return false;
            }
            let len = stack.len();
            let a = stack[len -1];
            let mut count = 1;


            for r in (1..len).rev(){

                if stack[r] == stack[r-1] {
                    count += 1;
                }else{
                    return false;
                }

                if a == count {
                    stack.resize(stack.len() - a, 0);
                    break;
                }

            }

            return continuefn(stack);
        }

    
    
    }
    

}

