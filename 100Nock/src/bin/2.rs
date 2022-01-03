/*
105 という数は, 非常に特殊な性質を持つ - 奇数なのに, 約数が 8 個もある.
さて, 1 以上 N 以下の奇数のうち, 正の約数を ちょうど 8 個持つようなものの個数を求めよ.
*/

use proconio::{input, fastout};

fn main() {
    input!{N : i32};
    //let mut vec : Vec<i32> = Vec::new();
    //let mut a = -1;
    //while a <= N {
    //    a += 2;
    //    vec.push(a);
    //}
//
    //let mut count =0;
    //for i in vec {
    //    
    //}

    let pred = |x| {x % 2 != 0 && (1..=x).filter(|&y| x % y == 0).count() == 8};
    let ans = (1..=N).filter(|&k| pred(k)).count();
    let a = (1..=N);

    println!("{}", ans);
}