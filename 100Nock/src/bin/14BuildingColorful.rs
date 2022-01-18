#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use proconio::*;
 
#[fastout]
fn main() {
    input!{n:usize, k:usize}
    input! {a:[usize;n]}

    let first = a[0].clone();
    let mut min_ans = 10u128.pow(18);
    
    for bit in 0..(1 << n) {
        let mut sum = 0;
        let mut cost = 0;
        let mut cnt = 0;
        let mut  beforeH = first;
        

        for i in 1..n { //a[0]はせんたくしないので1..から始める
            if (bit >> i) & 1  == 1 {//伸ばすと決めた建物
                cnt += 1; //cnt = 4 で５つの建物が見える
                if beforeH > a[i] {
                    cost = beforeH + 1 - a[i];
                    sum += cost;
                    beforeH += 1;
                }else{ //すでに伸ばす必要がないとき
                    //cost is 0
                    beforeH = a[i];
                }

            }
        }

        if cnt +1 >= k && min_ans > sum as u128{
            min_ans = sum as u128;
        }
    }

    println!("{}", min_ans);
}