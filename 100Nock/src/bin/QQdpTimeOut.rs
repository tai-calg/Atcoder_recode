//n+1, nっぽい関係が見えてきた瞬間dp　xor  再帰　を使うの確定。
/*
dpにするならクソデカdpにした時点でTLEが確定する。dpは小さくしてそれをforの中で更新していくスタイルで行くべき

たとえばn+1, n, n-1の関係までしか参照しないなら、それ以下の記憶は捨てていいということになる。

for i in 0..=n {　この中でi++しながらdpをどんどん更新して行くスタイルもあり　}
*/

use proconio::input;

static V: u64 = 998244353;
fn main() {
    input! {
        n:usize
    };

    let mut arr = [1; 9];
    for _ in 1..n {
        let new_arr = [
            arr[0..=1].iter().sum::<u64>() % V,
            arr[0..=2].iter().sum::<u64>() % V,
            arr[1..=3].iter().sum::<u64>() % V,
            arr[2..=4].iter().sum::<u64>() % V,
            arr[3..=5].iter().sum::<u64>() % V,
            arr[4..=6].iter().sum::<u64>() % V,
            arr[5..=7].iter().sum::<u64>() % V,
            arr[6..=8].iter().sum::<u64>() % V,
            arr[7..=8].iter().sum::<u64>() % V,
        ];
        arr = new_arr; //!!! 
    }

    let result = arr.iter().sum::<u64>() % V;

    println!("{}", result);
}
//ダメな例↓　クソデカdpをHashで作成してアクセスしているのでアクセスの探索に時間がかかってTLEする

/*
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
use proconio::marker::Usize1;
use std::collections::HashMap; 


#[fastout]
fn main() {

    let cons =  998244353;

    input! {n:usize}

    let mut dp:HashMap<(usize,usize),usize> = HashMap::new(); 
    //dfsの中でdpは参照するだけにしないとダメ。
    
    let mut ans =0;

    //make dp here
    for i in 1..=9{
        dp.insert((i,0), 1);
    }

    //++でdpを作る
    //count は1,2,...,nが入る
    for count in 1..n {
        for i in 1..=9{
            let dp_key = (i,count);
            

            if i == 1 {
                let mut val = (dp[&(i,count-1)] + dp[&(i+1,count-1)] );
                if val > cons{
                    val %= cons;
                } 

                dp.insert(dp_key, val);
            }else if i == 9{
                let mut val = (dp[&(i,count-1)] + dp[&(i-1,count-1)] );
                if val > cons{
                    val %= cons;
                } 

                dp.insert(dp_key, val);
            }else{
                let mut val = (dp[&(i,count-1)] + dp[&(i-1,count-1)] + dp[&(i+1,count-1)]);
                if val > cons{
                    val %= cons;
                } 

                dp.insert(dp_key,  val);
            }

        }


    }

    for i in 1..=9{
        ans += dp[&(i,n-1)];
    }

    println!("{}", ans % cons);
}
*/