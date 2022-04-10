use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: u32,
        x: u32,
        mut a: [u32; n],
    }

    for i in 0..n {
        let coupon_num = a[i] / x; //貪欲法のようにあらかじめそれぞれに対して効率よく使える枚数を計算
        if coupon_num >= k {
            a[i] -= k * x;
            k = 0;
            break;
        } else {
            a[i] -= coupon_num * x;
            k -= coupon_num;
        }
    }
    //それでも余ったら…
    if k > 0 {
        a.sort();
        a.reverse();
        for i in 0..n {
            a[i] = 0;
            k -= 1;
            if k <= 0 {
                break;
            }
        }
    }
    let mut total: u64 = 0;
    for i in 0..n {
        total += a[i] as u64;
    }
    println!("{}", total);
}



/*
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input! {n:isize, k:isize, x:isize,mut A: [isize;n]}

        A.sort();
        A.reverse();

    
        let k = coupon(&mut A, k , x, false);



    A.sort();
    A.reverse();

    let _k = coupon(&mut A, k , x, true);



    println!("{}",A.iter().sum::<isize>());






}


fn coupon(vec: &mut Vec<isize>, k:isize, x:isize, issecond:bool) -> isize{ //残りのkを返す。vecはソートしておけ
    let mut count = 0;

    if issecond == false {
        for i in 0..vec.len() {
            while (vec[i].clone()  - x ) > 0{
                vec[i] = std::cmp::max(0,vec[i] - x) ;
                count +=1;
                if count == k {
                    return 0;
                }
            }
    
        }
    
        return k - count;

    }else {
        
        for i in 0..vec.len() {
            if count == k {
                return 0;
            }
                vec[i] = std::cmp::max(0,vec[i] - x) ;

                count +=1;
    
        }

    }

    return k;

}
*/