#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use proconio::*;
 
#[fastout]
fn main() {
    input!{n:usize, k:usize}
    input! {a:[usize;n]}

    let first = a[0].clone();
    let mut min_ans = std::usize::MAX;
    
    for bit in 0..(1 << n) {
        if (bit as u64).count_ones() != k as u32 {
            continue;
        }
        let mut sum = 0;
        let mut cost = 0;
        let mut cnt = 0;
        let mut  beforeH = first;
        

        for i in 1..n { //a[0]は選択しないので1..から始める
            if (bit >> i) & 1  == 1 {//伸ばすと決めた建物
                cnt += 1; //cnt = 4 で５つの建物が見える
                if beforeH >= a[i] {
                    cost = beforeH + 1 - a[i];
                    sum += cost;
                    beforeH += 1;
                }else{ //すでに伸ばす必要がないとき
                    //cost is 0
                    beforeH = a[i];
                }



            }
        }


        min_ans = min_ans.min(sum);
    }

    println!("{}", min_ans);
}