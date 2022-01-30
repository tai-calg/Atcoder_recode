#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::VecDeque;

use proconio::*;
 
#[fastout]
fn main() {
    //デフォの文字列が回文的かどうかが最重要
    //最後の文字がaじゃないなら元が回文でないとだめ
    //最後の文字がaの回数分だけaを追加するのが必要条件

    input! {s:String}

    

    let  vecS = s.chars().collect::<Vec<char>>(); //文字列操作はこれ


    let mut flg = true;

    if vecS[vecS.len() -1] != 'a' { //追加がいらないとき

        let revS = s.chars().rev().collect::<Vec<char>>() ;

        for i in 0..vecS.len(){

            if vecS[i] == revS[i] {
            }else {
                flg = false;
                break;
            }


        }

    }

    else{ //追加しなきゃいけないとき
        let countLast = countLastA(&vecS);
        let countFront = countFrontA(&vecS);
        let mut news = s.chars().collect::<VecDeque<char>>();

        let cnt = (countLast - countFront) as isize;
        if cnt > 0 { 
            for _ in 0..cnt{
                news.push_front('a');
            }
        }



        let mut revS = Vec::from(news.clone());
        revS.reverse();

        for i in 0..revS.len(){

            if news[i] == revS[i] {
            }else {
                flg = false;
                break;
            }

        }
        



        

        
    }



    if flg { println!("Yes");}
    else{ println!("No");}



}


fn countLastA (s :&Vec<char>)->usize{
    let mut res = 0;

    for i in 0..s.len(){
        if s[s.len() -1-i] == 'a'{
            res +=1;
        }else {
            break;
        }
    }

    return res ;
}

fn countFrontA (s :&Vec<char>)->usize{
    let mut res = 0;

    for i in 0..s.len(){
        if s[i] == 'a'{
            res +=1;
        }else {
            break;
        }
    }

    return res ;
}