use proconio::input;

const M: u64 = 998244353;

fn main() {
    input! {
        mut n: u64,
    }
    let mut m = n;
    let mut keta = 0;
    while m > 0 {
        keta += 1;
        m /= 10;
    }
    eprintln!("keta:{}", keta);

    let mut ans = 0;
    for k in 1..keta {
        let base = 10u64.pow(k as u32 - 1);
        let m0 = 1;
        let m1 = 10u64.pow(k as u32) - 1 - base + 1;
        //m0 + m1 は個数
        let sum = ((m0 + m1) % M) * ((m1 - m0 + 1) % M) / 2; 
        // sigma(x) = 1/2 x x+1 
        // modM : x*(x+1) = (mod M : x) * (mod M : (x+1))

        eprintln!("{}, {}, {}, {}", k, m0, m1, sum);//error起こしたときに値を確認できる。これでデバッグする

        ans += sum;
        ans %= M;
    }

    let base = 10u64.pow(keta as u32 - 1);
    let m0 = 1;
    let m1 = n - base + 1; //最後は10e+(n-1) ~ 10e+(n) -1 じゃないのでfor から外す
    let sum = ((m0 + m1) % M) * ((m1 - m0 + 1) % M) / 2;
    eprintln!("{}, {}, {}, {}", keta, m0, m1, sum);
    ans += sum;
    ans %= M;

    println!("{}", ans);
}


/* 
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    const SHOU : usize = 998244353;

    input! {n:usize}




    // 簡潔に書けばこれ
    //for i in 1..n+1 {
    //    sum += f(i);
    //    sum %= SHOU;
    //}

    
    let mut sum = 0;

    //桁事に場合分け
    for i in 0..18{ //e+18まで



        if n >= 10usize.pow(i) {
            //i+1桁目
            let start = 10usize.pow(i); //1,9 ... 10,99
            let end = std::cmp::min(n, 10usize.pow(i+1) - 1);

            sum +=  sigma(end,start )  - (end - start +1 )* ( 10usize.pow(i) -1);

            //shou
            sum %= SHOU;

        }
        else{
            break;
        }
    } 

    
println!("{}",sum );



}

fn f(x: usize) -> usize {
    let mut count = 0;
    let mut tmp = x;
    while true {
        tmp = tmp/ 10;
        count += 1;
        if tmp == 0 {
            break;
        }
    }

    let ret = x - 10usize.pow(count-1) +1;
    ret
}

fn sigma(x:usize,start:usize)->usize{
    let mut sum = 0;
    for i in start..x+1{
        sum += i;
    }
    sum
}

*/