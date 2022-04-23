#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input! { a: i64, b: i64, c: i64 , d: i64 , e: i64 ,f: i64 , x: i64 }

    let cont = x/(a+c);
    let amari = if (x%(a+c)) <= a {
        x%(a+c)
    } else {
        a
    };

    let taka = cont*(a*b)+ amari * b;

    let cont2 = x/ (d+f);
    let amari2 = if (x%(d+f)) <= d {
        x%(d+f)
    } else {
        d
    };

    let ao = cont2*(d*e)+ amari2 * e;

    if taka > ao {
        println!("{}","Takahashi");
    } else if taka < ao {
        println!("{}","Aoki");
    }else {
        println!("{}","Draw");
    }

}
