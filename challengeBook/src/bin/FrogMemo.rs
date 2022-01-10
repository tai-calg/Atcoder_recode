#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::vec;

use proconio::*;
 
#[fastout]
fn main (){
    input! {n:usize}
    input! {H:[i64;n]}

    //require H[0] → H[n-1] に行くまでのコストのmin
    const INF:i64 = 10000;

    let mut dp :Vec<i64>  = Vec::new(); //dynamic programming の略
    dp.resize_with(n, || INF);
    
    assert_eq!((5i8-7i8).abs(), 2i8);



    println!("{}", solve(n, &H));

    
}

fn rec(i:usize, dp :&mut Vec<i64>,H: &Vec<i64>, INF:i64) -> i64{
    if dp[i] < INF {
        return dp[i] ;
    }

    //base case 
    if i==0 { return 0; }

    let mut ret = INF;
    
    //足場 i-1から来た場合
    assert_eq!(dp[i], INF);
    chmin(&mut ret, &mut ( rec(i-1, dp,H, INF) +  (H[i] - H[i-1]).abs()));
    //足場 i-2から来た場合
    if i > 1 {chmin(&mut ret,&mut( rec(i-2, dp,H, INF) +  (H[i] - H[i-2]).abs()) );}
    //ret を最小に更新していく。
    dp[i] =  ret;
    return dp[i] ;

    
}
fn chmin<T>( a: &mut T, b: &mut T)
where T : Ord + Copy
{
    if *a > *b {
        *a = *b;
    }
}

fn solve (n:usize, H: &Vec<i64>)->usize{
    let mut dp = vec![0i64;n];
    dp[1] = (H[1] - H[0]).abs();
    for i in 2..n{
        dp[i] = std::cmp::min(
            dp[i-2]+(H[i-2] - H[i]).abs(),
            dp[i-1]+(H[i-1]-H[i]).abs()
         );
    }
    return dp[n-1] as usize;
}
//fn chmin( a:&mut i64, b:& i64)
//{
//    if *a > *b {
//        *a = *b;
//    }
//}
////たぶん数字がコピートレイトを持っているからa,bともにDropしない
//Copyできない型を引数にとるとき、＆を使うのだろうけど、その時の処理をどうするかはまたその時考えよう。


/*
fn chminds<'a, T:'a>(mut a: &'a T, mut b:&'a T)
where T : Ord
{
    if *a > *b {
        a = b;
    }
}

*/
