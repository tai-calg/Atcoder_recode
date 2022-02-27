use proconio::{input, fastout};

use std::collections::VecDeque;
/*
フェネック「D問題は、筒の中の状態を底から順に
「iが書かれたボールがj個連続してる」って形で持っておくと、ボールを追加したときの操作がO(1)でできるねー」
*/
#[fastout]
fn main() {
    input!{
        n: usize,
        a: [usize;n]
    }
 
    let mut que = VecDeque::new();//_backだけじゃねーか！Vecでおｋ。
    let mut ans = 0;
    for element in a.into_iter() {
        ans += 1;
        let (u, v) = match que.pop_back() { //let (u,v)が原因でqueのTが(usize,usize)と決定されてる
            Some(x) => x,
            None => (0, 0),
        };
        //println!("{}: {}", u,v);

        //uがvalue, vがvalueの連続回数！
        if element == u {//前回と今回が連続
            if v + 1 == element {
                ans -= element;
            } else {
                que.push_back((u, v+1));//連続だけど消される長さではないので連続カウントを＋＋
            }
        } else {
            que.push_back((u, v));//始めにpopしちゃった分を入れなおしてるだけ
            //例　1回目：elem = 2 →pop(0,0), push(0,0)、push(2,1) 
            //2回目:elem=2 →pop(2,1), 長さ削る, 以上。
            que.push_back((element, 1));
        }
 
        println!("{:?}", ans);
    }
}
/*



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

*/