#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use proconio::*;
 
#[fastout]
fn main() {
    input! {r: usize, c:usize,
    a:[[usize;c];r]}// r << c

    let mut ans = 0;
    for bit in 0..(1 << r){ //行だけ返すか返さないかの2bit探索
        let mut count = 0;
        for i in 0..c { //列は効率がいいほうに返すので2bitである必要なし
            let mut count2 = 0;
            for j in 0..r {
                if bit & (1 << j) != 0 { //この行はひっくり返さないという選択の時
                    if a[j][i] == 1 {
                        count2 += 1;
                    }
                }else{ //この行はひっくり返すという選択の時
                    if a[j][i] == 0 { 
                        count2 += 1;
                    }
                }
            }
            count += std::cmp::max(count2, r - count2);
        }

        ans = std::cmp::max(ans, count);
    }


    println!("{}",ans );
}